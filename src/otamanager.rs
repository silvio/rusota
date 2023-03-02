
extern crate md5;

use std::{fmt, fs, collections::HashMap};
use rocket::serde::{Deserialize, Serialize};
use log::{debug, trace};
use std::str::FromStr;

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

    pub fn populate(&mut self) -> &OtaManager {
        debug!("Populating OtaManager");

        for entry in fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            let path = &entry.path();
            debug!("work on: {}", path.display());

            let suffix = match path.extension().clone() {
                Some(x) => x.to_str().unwrap(),
                None => {
                    debug!("Not interrested in this file");
                    continue;
                }
            };

            // Find the id to identify the package (via name)
            let suffixes: Vec<&'static str> = vec![".zip", ".zip.prop", ".zip.md5sum"];

            let mut id = path.to_str().unwrap().clone();
            for suffix in suffixes {
                id = match id.strip_suffix(suffix) {
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
                    let metadata = fs::metadata(&entry.path()).unwrap();
                    package.size = metadata.len();
                },
                "prop" => {
                    let filecontent = fs::read_to_string(path).expect("path not found");
                    for line in filecontent.lines() {
                        if line.starts_with("#") { continue };
                        let (key, value) = line.split_once("=").unwrap();
                        if key == "ro.system.build.date.utc" {
                            package.datetime = u64::from_str_radix(value, 10).unwrap();
                        }
                        if key == "ro.lineage.build.version" {
                            package.version = value.to_string();
                        }
                        if key == "ro.lineage.releasetype" {
                            package.releasetype = RomType::from_str(value).unwrap();
                        }
                    }

                    package.has_prop = true;
                },
                "md5sum" => {
                    let filecontent = fs::read_to_string(path).expect("path not found");
                    let checksum = filecontent.split_ascii_whitespace().next().unwrap();
                    debug!("Cheksum found: {}", checksum);
                    package.checksum = checksum.to_string();

                    package.has_md5sum = true;
                },
                _ => {
                    debug!("Not interrested in this file");
                    continue;
                },
            };

            trace!("work done on package:\n{}", package);
        }

        debug!("before cleanup\n{:?}", self.packages);
        self.cleanup();
        debug!("after cleanup\n{:?}", self.packages);

        self
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
