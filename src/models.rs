use regex::Regex;
use std::{fmt, str::FromStr};

use diesel::{backend::Backend, deserialize::Queryable};
use rocket::form::{self, FromFormField, ValueField};
use rocket::serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use semver;

use super::schema::{config, target_version_mappings, targets};

#[derive(PartialEq, Clone, Debug, Hash, Eq, AsExpression)]
#[sql_type = "diesel::sql_types::Text"]
pub struct SemVer(pub semver::Version);
impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        semver::Version::from_str(&s)
            .map_err(de::Error::custom)
            .map(SemVer)
    }
}
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for SemVer
where
    DB: Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        let s: String = self.clone().0.to_string();
        s.to_sql(out)
    }
}
impl<DB, ST> Queryable<ST, DB> for SemVer
where
    DB: Backend,
    String: Queryable<ST, DB>,
{
    type Row = <String as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        SemVer(
            semver::Version::from_str(String::build(row).as_str())
                .expect("Parse Version from database"),
        )
    }
}
impl Serialize for SemVer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[derive(PartialEq, AsExpression, Clone)]
#[sql_type = "diesel::sql_types::Text"]
pub enum Version {
    Latest,
    SemVer(SemVer),
}
impl FromStr for Version {
    type Err = std::io::Error;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let maybe_semver = semver::Version::from_str(raw).map(SemVer);
        if maybe_semver.is_ok() {
            return Ok(Version::SemVer(maybe_semver.unwrap()));
        }
        match raw {
            "LATEST" => Ok(Version::Latest),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unable to parse '{}' as Version", raw),
            )),
        }
    }
}
#[rocket::async_trait]
impl<'r> FromFormField<'r> for Version {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Version::from_str(field.value).map_err(|e| {
            form::Errors::from(form::Error::validation(format!(
                "Unable to parse as Version: {}",
                e
            )))
        })
    }
}

impl Into<String> for Version {
    fn into(self) -> String {
        match self {
            Version::Latest => "LATEST".to_string(),
            Version::SemVer(ver) => ver.0.to_string(),
        }
    }
}
impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::Latest => f.write_str("VERSION<Latest>"),
            Version::SemVer(ver) => f.write_fmt(format_args!("Version<{}>", ver.0)),
        }
    }
}
impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for Version
where
    DB: Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        let s: String = self.clone().into();
        s.to_sql(out)
    }
}

impl<DB, ST> Queryable<ST, DB> for Version
where
    DB: Backend,
    String: Queryable<ST, DB>,
{
    type Row = <String as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        Version::from_str(String::build(row).as_str()).expect("Parse Version from database")
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Version::from_str(s.as_str()).map_err(de::Error::custom)?)
    }
}
impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: String = Version::clone(self).into();
        serializer.serialize_str(s.as_str())
    }
}

// We don't have the inner be a Regex because you can't really reverse a Regex
// and write it out can you?
#[derive(Serialize, AsExpression, Debug, Clone)]
#[sql_type = "diesel::sql_types::Text"]
#[serde(crate = "rocket::serde")]
pub struct ValidRegex(String);

impl Into<String> for &ValidRegex {
    fn into(self) -> String {
        self.0.clone()
    }
}
impl Into<String> for ValidRegex {
    fn into(self) -> String {
        self.0
    }
}
impl Into<Regex> for ValidRegex {
    fn into(self) -> Regex {
        Regex::new(&self.0).expect("Unwrapping ValidRegex")
    }
}

impl<'de> Deserialize<'de> for ValidRegex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Regex::new(&s).map_err(de::Error::custom)?;
        Ok(ValidRegex(s))
    }
}
impl<DB, ST> Queryable<ST, DB> for ValidRegex
where
    DB: Backend,
    String: Queryable<ST, DB>,
{
    type Row = <String as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> Self {
        ValidRegex(String::build(row))
    }
}

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for ValidRegex
where
    DB: Backend,
    String: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
    ) -> diesel::serialize::Result {
        let s: String = self.clone().0;
        s.to_sql(out)
    }
}

#[derive(Insertable, Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "config"]
#[serde(crate = "rocket::serde")]
pub struct Config {
    #[serde(default)]
    pub id: i32,
    pub fill_version: Version,
    pub asset_regex: ValidRegex,
    pub github_user: Option<String>,
    pub github_repo: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Insertable)]
#[primary_key(target_id, current_version)]
#[serde(crate = "rocket::serde")]
pub struct TargetVersionMapping {
    pub target_id: i32,
    pub current_version: SemVer,
    pub update_version: Version,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable, Insertable, AsChangeset, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Target {
    #[serde(default)]
    pub id: i32,
    pub target: String,
    pub regex: ValidRegex,
}

#[derive(Insertable, Deserialize)]
#[table_name = "targets"]
#[serde(crate = "rocket::serde")]
pub struct NewTarget {
    target: String,
    regex: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    msg: String,
}

mod tests {
    #[test]
    fn it_parses_speacial() {
        let res = Version::from_str("LATEST");
        assert_eq!(res.is_ok(), true);
        let unwrapped = res.unwrap();
        assert_eq!(unwrapped, Version::Latest)
    }

    #[test]
    fn it_parses_semver() {
        let res = Version::from_str("1.2.3");
        assert_eq!(res.is_ok(), true);
        let unwrapped = res.unwrap();
        assert_eq!(
            unwrapped,
            Version::SemVer(SemVer::from_str("1.2.3").unwrap())
        )
    }
}
