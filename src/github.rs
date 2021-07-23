use regex::RegexSet;
use std::str::FromStr;

use diesel::{Connection, SqliteConnection};
use rocket::serde::{de, Deserialize, Deserializer};
use semver;

use super::crud;
use super::models::{self, SemVer, TargetVersionMapping};

// Note: Unfortuanately, it seems like we can't use the octocrab library for github because we have mismatching dependencies
fn deserialize_github_semver<'de, D>(deserializer: D) -> Result<SemVer, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.chars().nth(0) != Some('v') {
        Err(de::Error::custom(
            "GithubSemVer expected version to start with v",
        ))
    } else {
        semver::Version::from_str(&s[1..])
            .map_err(de::Error::custom)
            .map(|v| SemVer(v))
    }
    // if 'v' == char::deserialize(deserializer)? {
    //     Ok(GithubSemVer(SemVer::deserialize(deserializer)?))
    // } else {
    //      Err(de::Error::custom("GithubSemVer expected version to start with v"))
    // }
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GithubAsset {
    pub browser_download_url: models::Url,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GithubRelease {
    pub url: models::Url,
    #[serde(deserialize_with = "deserialize_github_semver")]
    pub tag_name: SemVer,
    pub draft: bool,
    pub body: String,
    pub assets: Vec<GithubAsset>,
}

async fn get_releases(
    basic_user: String,
    basic_token: String,
    user: String,
    repo: String,
) -> Result<Vec<GithubRelease>, String> {
    let client = reqwest::Client::new();
    client
        .get(format!(
            "https://api.github.com/repos/{}/{}/releases",
            user, repo
        ))
        .basic_auth(basic_user, Some(basic_token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "taus")
        .send()
        .await
        .map_err(|e| format!("Reqwest get error: {}", e))?
        .json::<Vec<GithubRelease>>()
        .await
        .map_err(|e| format!("Reqest json error: {}", e))
}

// TODO maybe make the error something like you see in hyper docs `Box<dyn std::error::Error + Send + Sync>`
pub async fn discover_new_releases() -> Result<(), String> {
    let database_url = std::env::var("DATABASE_URL").map_err(|_e| "DATABASE_URL must be set")?;
    let conn = SqliteConnection::establish(&database_url)
        .map_err(|_e| format!("Error connecting to {}", database_url))?;

    let user = std::env::var("GITHUB_USER").map_err(|_e| "GITHUB_USER must be set")?;
    let pat = std::env::var("GITHUB_PAT").map_err(|_e| "GITHUB_PAT must be set")?;

    let config = crud::get_config(&conn).map_err(|_e| "Failed fetching config")?;
    let targets = crud::get_targets(&conn).map_err(|_e| "Failed to get targets")?;
    let reg_set = RegexSet::new(
        targets
            .iter()
            .map(|t| Into::<String>::into(&t.regex))
            .into_iter(),
    )
    .map_err(|_e| "Failed to create RegexSet")?;

    match (config.github_user, config.github_repo) {
        (Some(github_user), Some(github_repo)) => {
            let rels = get_releases(user, pat, github_user, github_repo).await?;

            for rel in rels.iter() {
                for ass in rel.assets.iter() {
                    let matches: Vec<_> = reg_set.matches(&ass.name).into_iter().collect();
                    match matches.len() {
                        0 => {
                            println!("None matched");
                        }
                        1 => {
                            let matched_ind = matches.iter().next().unwrap().clone();
                            let matched_targ = targets[matched_ind].clone();
                            let maybe_existing =
                                crud::get_mapping(&conn, matched_targ.id, rel.tag_name.clone())
                                    .map_err(|e| format!("Failed to get mapping: {}", e))?;
                            // TODO Write this elsewhere also:
                            // There must be a one to one rel between mappings and assets because each asset could make an update request. This suggests that it makes more sense to have mappings and assets be combined into the same thing
                            if maybe_existing.is_none() {
                                println!(
                                    "We should insert a new target mapping using fill_version"
                                );
                                crud::create_mapping(
                                    &conn,
                                    TargetVersionMapping {
                                        target_id: matched_targ.id,
                                        current_version: rel.tag_name.clone(),
                                        update_version: config.fill_version.clone(),
                                        download_url: ass.browser_download_url.clone(),
                                    },
                                )
                                .map_err(|e| format!("Failed to create mapping: {}", e))?;
                            }
                        }
                        _ => {
                            println!("Multiple regexes matched");
                        }
                    }
                }
            }
            Ok(())
        }
        _ => {
            println!("Skipping discovery interval, user or repo not set");
            Ok(())
        }
    }
}
