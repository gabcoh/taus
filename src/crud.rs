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
    mapping: models::TargetVersionMapping
) -> Result<(), Error> {
    use self::schema::target_version_mappings;
    diesel::insert_into(target_version_mappings::table)
	.values(mapping)
	.execute(conn)
	.map(|_r| ())
}
pub fn get_mapping(
    conn: &diesel::SqliteConnection,
    target_id: i32,
    current_version: models::SemVer
) -> Result<Option<models::TargetVersionMapping>, Error> {
    use self::schema::target_version_mappings::dsl;
    dsl::target_version_mappings.filter(dsl::target_id.eq(target_id).and(dsl::current_version.eq(current_version)))
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
