use rocket_sync_db_pools::diesel::prelude::*;
use rocket_sync_db_pools::diesel::result::Error;
use std::vec::Vec;

use crate::models;
use crate::schema;

pub fn get_config(conn: &diesel::SqliteConnection) -> Result<models::Config, Error> {
    use self::schema::config::dsl::*;
    config.first(conn)
}
pub fn update_config(conn: &diesel::SqliteConnection, config: models::Config) -> Result<(), Error> {
    use self::schema::config;
    conn.transaction::<_, Error, _>(|| {
        let cur_config: models::Config = config::dsl::config.first(conn)?;
        diesel::delete(&cur_config).execute(conn)?;
        diesel::insert_into(config::table)
            .values(config)
            .execute(conn)
            .map(|_r| ())
    })
}

pub fn get_targets(conn: &diesel::SqliteConnection) -> Result<Vec<models::Target>, Error> {
    use self::schema::targets::dsl::*;
    targets.get_results(conn)
}
pub fn get_target_by_name(
    conn: &diesel::SqliteConnection,
    name: String,
) -> Result<models::Target, Error> {
    use self::schema::targets;
    targets::dsl::targets
        .filter(targets::dsl::target.eq(name))
        .first(conn)
}

pub fn create_target(
    conn: &diesel::SqliteConnection,
    target: models::NewTarget,
) -> Result<(), Error> {
    use self::schema::targets;
    diesel::insert_into(targets::table)
        .values(target)
        .execute(conn)
        .map(|_r| ())
}

// I think having an update target is a bad idea. Changing the regex would require re running the target on all past stuff and it just gets complicated. Lets make targets immutable for now? Versioning targets would probably be best but that gets complicated...
// pub fn update_target(conn: &diesel::SqliteConnection, target: models::Target) -> Result<(), Error> {
//     target.save_changes(conn).map(|v| {
//         // We apparently needed to give the closure parameter a type, so this is what I came up with
//         let _v: models::Target = v;
//         ()
//     })
// }

pub fn get_mappings(
    conn: &diesel::SqliteConnection,
) -> Result<Vec<models::TargetVersionMapping>, Error> {
    use self::schema::target_version_mappings::dsl::*;
    target_version_mappings.get_results(conn)
}
pub fn create_mapping(
    conn: &diesel::SqliteConnection,
    mapping: models::TargetVersionMapping,
) -> Result<(), Error> {
    use self::schema::{target_version_mappings, targets};
    conn.transaction::<_, Error, _>(|| {
        let mut target: models::Target = targets::dsl::targets
            .filter(targets::dsl::id.eq(mapping.target_id))
            .first(conn)?;
        match target.latest {
            Some(ref vers) => {
                if vers.0 < mapping.current_version.0 {
                    target.latest = Some(mapping.current_version.clone());
                }
            }
            None => {
                target.latest = Some(mapping.current_version.clone());
            }
        }
        target.save_changes(conn).map(|r| {
            let _annotation: models::Target = r;
            ()
        })?;
        diesel::insert_into(target_version_mappings::table)
            .values(mapping)
            .execute(conn)
            .map(|_r| ())
    })
}
pub fn get_mapping(
    conn: &diesel::SqliteConnection,
    target_id: i32,
    current_version: models::SemVer,
) -> Result<Option<models::TargetVersionMapping>, Error> {
    use self::schema::target_version_mappings::dsl;
    dsl::target_version_mappings
        .filter(
            dsl::target_id
                .eq(target_id)
                .and(dsl::current_version.eq(current_version)),
        )
        .limit(1)
        .load::<models::TargetVersionMapping>(conn)
        .map(|res| {
            let mut v: Vec<models::TargetVersionMapping> = res;
            if v.len() > 0 {
                Some(v.swap_remove(0))
            } else {
                None
            }
        })
}

// pub fn get_associated_asset(
//     conn: &diesel::SqliteConnection,
//     mapping: models::TargetVersionMapping,
// ) -> Result<models::Asset, Error> {
//     use self::schema::{assets, targets};
//     conn.transaction::<_, Error, _>(|| {
//         let asset_version = match mapping.update_version {
//             models::Version::Latest => {
//                 let target: models::Target = targets::dsl::targets
//                     .filter(targets::dsl::id.eq(mapping.target_id))
//                     .first(conn)?;
//                 target.latest.ok_or(Error::NotFound)?
//             }
//             models::Version::SemVer(ver) => ver,
//         };
//         let asset: models::Asset = assets::dsl::assets
//             .filter(
//                 assets::dsl::target_id
//                     .eq(mapping.target_id)
//                     .and(assets::dsl::version.eq(asset_version)),
//             )
//             .first(conn)?;
//         Ok(asset)
//     })
// }

// pub fn create_asset(conn: &diesel::SqliteConnection, asset: models::Asset) -> Result<(), Error> {
//     use self::schema::assets;
//     diesel::insert_into(assets::table)
//         .values(asset)
//         .execute(conn)
//         .map(|_r| ())
// }
