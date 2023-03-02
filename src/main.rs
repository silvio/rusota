
// TODO: This will get an option in future
static OTA_ROOT: &'static str = "ota";

extern crate log;
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
extern crate strum_macros;
extern crate dotenv;

mod package;
mod otamanager;
use otamanager::*;
use package::Package;

use rocket::State;
use rocket_dyn_templates::*;
use rocket::serde::{Deserialize, Serialize, json::Json};
use rocket::fs::{NamedFile, relative};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ResponseEntry {
    /// `datetime`: Date time of the release as unix timestamp
    datetime: u64,
    /// `filename`: Just the filename, not the path
    filename: String,
    /// `id`: will be the md5 checksum of the file
    id: String,
    /// `romtype`: Type of the rom, like stable, nightly. See enum RomType
    romtype: package::RomType,
    /// `size`: size of the file
    size: u64,
    /// `url`: url of the file for download
    url: String,
    /// `version`:
    version: String,
}

impl From<Package> for ResponseEntry {
    fn from(package: Package) -> Self {
        ResponseEntry {
            datetime: package.datetime,
            filename: package.filename,
            id: package.datetime.to_string(),
            romtype: package.releasetype,
            size: package.size,
            url: format!("{}/{}", dotenv::var("DOWNLOADURL").unwrap(), package.datetime),
            version: package.version
        }
    }
}

//impl trait From<Package> {
//    fn from(Package) -> Self {
//
//    }
//}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Response {
    response: Vec<ResponseEntry>,
}

#[get("/", format="html", rank=2)]
fn root_index_html(ota: &State<OtaManager>) -> Template {
    Template::render("index", context!(packages: &ota.list()))
}

#[get("/", format="json")]
fn root_index_json(ota: &State<OtaManager>) -> Json<Response> {
    let mut out: Response = Response::default();
    let response = &mut out.response;

    for package in ota.list() {
        response.push(package.clone().into());
    }

    return Json(out);
}

#[get("/<datetime>")]
pub async fn root_otapackage(datetime: String, ota: &State<OtaManager>) -> Option<NamedFile> {
    trace!("datetime: {}", datetime);
    let datetime = u64::from_str_radix(&datetime, 10).unwrap();
    trace!("datetime: {}", datetime);
    let path = Path::new(relative!(".")).join(ota.find_by_datetime(datetime));
    println!("PATH: {:?}", path);
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    info!("Log initialized ...");
    let mut otas = OtaManager::new(OTA_ROOT.to_string());
    otas.populate();

    dotenv::dotenv().ok();

    rocket::build()
        .manage(otas)
        .mount("/", routes![
            root_index_html,
            root_index_json,
            root_otapackage,
        ])
        .attach(Template::fairing())
}
