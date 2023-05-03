
extern crate md5;

use anyhow::Result;
use futures::{ SinkExt, StreamExt, };
use log::{debug, trace};
use notify::*;
use rocket::serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{fmt, fs, collections::HashMap};

use crate::package::{Package, RomType};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct OtaManager {
    path: String,
    packages: HashMap<String, Package>,
}

impl OtaManager {
    pub fn new(p: String) -> OtaManager {
        debug!("Initilaize OtaManager for {}", p);
        OtaManager {
            path: p,
            ..OtaManager::default()
        }
    }

    pub async fn populate(&mut self) -> Result<&OtaManager> {
        debug!("Populating OtaManager");

        futures::executor::block_on(async {
            if let Err(e) = self.async_watch().await {
                println!("error: {:?}", e);
            }
        });

        match self.update_rootfs().await {
            Ok(_) => {
                info!("OTAS populated");
            },
            Err(e) => {
                error!("otas could not populated: {:?}", e);
                unimplemented!();
            },
        };

        debug!("before cleanup\n{:?}", self.packages);
        self.cleanup();
        debug!("after cleanup\n{:?}", self.packages);

        Ok(self)
    }

    /// cleanup the packages list with packages which are not complete
    /// RETRUN: true when changes are done, false if not.
    fn cleanup(&mut self) -> bool {
        let len_before = self.packages.len();

        self.packages.retain(|_, v| v.complete());
        let len_after = self.packages.len();

        trace!("cleanup: before/after:{}/{}", len_before, len_after);

        len_before != len_after
    }

    pub fn list(&self) -> Vec<Package> {
        let mut packages = Vec::<Package>::new();
        for (_, value) in self.packages.iter() {
            packages.push(value.clone());
        }
        packages
    }

    pub fn find_by_checksum(&self, checksum: String) -> String {
        for package in &self.list() {
            if checksum == package.checksum {
                return format!("{}/{}", self.path, package.filename);
            }
        }

        return format!("Package with chcksum {} not found!", checksum);
    }

    pub fn find_by_datetime(&self, datetime: u64) -> String {
        for package in &self.list() {
            if datetime == package.datetime {
                return format!("{}/{}", self.path, package.filename);
            }
        }

        return format!("Package with datetime {} not found!", datetime);
    }

    pub async fn async_watcher(&self) -> notify::Result<(notify::RecommendedWatcher, futures::channel::mpsc::Receiver<notify::Result<notify::Event>>)> {
        let (mut tx, rx) = futures::channel::mpsc::channel(1);

        let watcher = RecommendedWatcher::new(move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        }, Config::default())?;

        Ok((watcher, rx))
    }

    pub async fn async_watch(&mut self) -> Result<()> {
        let (mut watcher, mut rx) = self.async_watcher().await?;

        watcher.watch(self.path.as_ref(), RecursiveMode::Recursive)?;

        while let Some(res) = rx.next().await {
            match res {
                Ok(ref event) => {
                    if event.kind.is_other() { continue };

                    self.update_rootfs().await?;
                },
                Err(e) => println!("error (ignored): {:?}", e),
            }
        }

        Ok(())
    }

    pub async fn update_rootfs(&mut self) -> Result<()> {
        self.packages = HashMap::default();

        // Find the id to identify the package (via name)
        let suffixes: Vec<&'static str> = vec![".zip", ".zip.prop", ".zip.md5sum"];

        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = &entry.path();
            debug!("work on: {}", path.display());

            let suffix = match path.extension().clone() {
                Some(x) => {
                    let Some(x) = x.to_str() else {
                        debug!("Problem with unwrapping of path extension");
                        continue
                    };
                    x
                },
                None => {
                    debug!("Problem with unwrapping of path extension");
                    continue;
                }
            };

            let Some(mut id) = path.to_str() else {
                debug!("Problem with unwrapping of path");
                continue;
            };

            for el in &suffixes {
                id = match id.strip_suffix(el) {
                    Some(x) => x,
                    None => id,
                }
            }
            debug!("+ id: {}", id);

            let mut package = self.packages.entry(id.to_string()).or_insert(Package::default());

            match suffix {
                "zip" => {
                    debug!("zip file found, processing ...");
                    package.has_zip = true;
                    package.filename = format!("{}", &entry.file_name().into_string().unwrap());
                    let metadata = fs::metadata(&entry.path())?;
                    package.size = metadata.len();
                },
                "prop" => {
                    let filecontent = fs::read_to_string(path).expect("path not found");
                    for line in filecontent.lines() {
                        if line.starts_with("#") { continue };
                        let Some((key, value)) = line.split_once("=") else {
                            debug!("Line contains no '='");
                            continue;
                        };
                        if key == "ro.system.build.date.utc" {
                            package.datetime = u64::from_str_radix(value, 10)?;
                        }
                        if key == "ro.lineage.build.version" {
                            package.version = value.to_string();
                        }
                        if key == "ro.lineage.releasetype" {
                            package.releasetype = RomType::from_str(value)?;
                        }
                    }

                    package.has_prop = true;
                },
                "md5sum" => {
                    let filecontent = fs::read_to_string(path).expect("path not found");
                    let Some(checksum) = filecontent.split_ascii_whitespace().next() else {
                        debug!("Cannot extract md5sum from checksum file");
                        self.packages.remove_entry(id);
                        continue;
                    };
                    debug!("Cheksum found: {}", checksum);
                    package.checksum = checksum.to_string();

                    package.has_md5sum = true;
                },
                _ => {
                    debug!("Not interrested in this file");
                    self.packages.remove_entry(id);
                    continue;
                },
            };

            trace!("work done on package:\n{}", package);
        }

        debug!("before cleanup\n{:?}", self.packages);
        self.cleanup();
        debug!("after cleanup\n{:?}", self.packages);

        Ok(())
    }
}

impl fmt::Display for OtaManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out: String ="".to_string();
        for package in &self.list() {
            out = format!("{}{}\n", out, &package);
        }
        write!(f, "{}", out)
    }
}
