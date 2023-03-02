
use std::fmt;
use log::trace;
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Display, EnumString)]
pub enum RomType {
    // This comes from Lineage
    UNOFFICIAL,
    // This are the standart one
    RELEASE,
    NIGHTLY,
    SNAPSHOT,
    EXPERIMENTAL,
    /// This type should not be used!
    #[default]
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Package {
    pub filename: String,
    pub size: u64,
    pub checksum: String,
    pub datetime: u64,
    pub version: String,
    pub releasetype: RomType,
    pub has_zip: bool,
    pub has_prop: bool,
    pub has_md5sum: bool,
}

impl Package {
    pub fn complete(&self) -> bool {
        let mut is_complete = true;

        if self.filename == String::default() { trace!("filename is default"); is_complete = false };
        if self.size == u64::default() { trace!("size is default"); is_complete = false };
        if self.checksum == String::default() { trace!("checksum is default"); is_complete = false };
        if self.datetime == u64::default() { trace!("datetime is default"); is_complete = false };
        if self.releasetype == RomType::default() { trace!("releasetype is default"); is_complete = false };
        if !self.has_zip { trace!("has no zipfile"); is_complete = false };
        if !self.has_prop { trace!("has no propfile"); is_complete = false };
        if !self.has_md5sum { trace!("has no md5sumfile"); is_complete = false };

        if is_complete {
            debug!("package {} IS  complete", self.filename);
        } else {
            debug!("package {} NOT complete", self.filename);
        };

        is_complete
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:32} - {:011} - {}", &self.datetime, &self.size, &self.filename)
    }
}

