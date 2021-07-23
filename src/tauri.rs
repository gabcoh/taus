use rocket::serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::{crud, models};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateAvailableResponse {
    pub url: models::Url,
    pub version: models::SemVer,
    pub notes: Option<String>,
    pub pub_date: Option<String>,
    pub signature: Option<String>,
}

pub fn tauri_produce_response(
    conn: &diesel::SqliteConnection,
    target_name: String,
    current_version_raw: String,
) -> Result<Option<UpdateAvailableResponse>, String> {
    let target =
        crud::get_target_by_name(conn, target_name).map_err(|e| format!("Error: {}", e))?;
    let current_version: models::SemVer = models::SemVer::from_str(current_version_raw.as_str())
        .map_err(|e| format!("Error: {}", e))?;
    let mapping = crud::get_mapping(conn, target.id, current_version.clone())
        .map_err(|e| format!("Error: {}", e))?
        .ok_or("Mapping not found")?;
    let asset_mapping = match mapping.update_version {
	models::Version::Latest => {
	    let latest = target.latest.ok_or("Latest not set for target")?;
	    crud::get_mapping(conn, target.id, latest).map_err(|e| format!("Error: {}", e))?
	},
	models::Version::SemVer(ver) => crud::get_mapping(conn, target.id, ver).map_err(|e| format!("Error: {}", e))?,
    }.ok_or("Update Mapping not found")?;
    if asset_mapping.current_version == current_version {
        Ok(None)
    } else {
        Ok(Some(UpdateAvailableResponse {
            url: asset_mapping.download_url,
            version: asset_mapping.current_version,
            notes: Option::None,
            pub_date: Option::None,
            signature: Option::None,
        }))
    }
}
