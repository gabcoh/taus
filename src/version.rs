// use std::{fmt, str::FromStr};

// use semver;
// use rocket::form::{self, FromFormField, ValueField};
// use rocket::serde::{Serialize, Deserialize, Serializer, Deserializer, de};

// #[derive(PartialEq)]
// pub struct SemVer(semver::Version);
// impl<'de> Deserialize<'de> for SemVer {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where D: Deserializer<'de>
//     {
//         let s = String::deserialize(deserializer)?;
//         semver::Version::from_str(&s).map_err(de::Error::custom).map(SemVer)
//     }
// }
// impl Serialize for SemVer {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&self.0.to_string())
//     }
// }

// #[derive(PartialEq, Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub enum Version {
//     Latest,
//     SemVer(SemVer)
// }
// impl FromStr for Version {
//     type Err = std::io::Error;
    
//     fn from_str(raw: &str) -> Result<Self, Self::Err> {
// 	let maybe_semver = semver::Version::from_str(raw).map(SemVer);
// 	if maybe_semver.is_ok() {
// 	    return Ok(Version::SemVer(maybe_semver.unwrap()));
// 	}
// 	match raw {
// 	    "LATEST" => Ok(Version::Latest),
// 	    _ => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unable to parse '{}' as Version", raw)))
// 	}
//     }
// }
// #[rocket::async_trait]
// impl<'r> FromFormField<'r> for Version {
//     fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
// 	Version::from_str(field.value).map_err(|_e| form::Errors::from(form::Error::validation("Unable to parse as Version")))
//     }
// }

// impl Into<String> for Version {
//     fn into(self) -> String {
// 	match self {
// 	    Version::Latest => "LATEST".to_string(),
// 	    Version::SemVer(ver) => ver.0.to_string()
// 	}
//     }
// }
// impl fmt::Debug for Version {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 	match self {
// 	    Version::Latest => f.write_str("VERSION<Latest>"),
// 	    Version::SemVer(ver) => f.write_fmt(format_args!("Version<{}>", ver.0))
// 	}
//     }
// }
// mod tests {
//     #[test]
//     fn it_parses_speacial() {
// 	let res = Version::from_str("LATEST");
// 	assert_eq!(res.is_ok(), true);
// 	let unwrapped = res.unwrap();
// 	assert_eq!(unwrapped, Version::Latest)
//     }

//     #[test]
//     fn it_parses_semver() {
// 	let res = Version::from_str("1.2.3");
// 	assert_eq!(res.is_ok(), true);
// 	let unwrapped = res.unwrap();
// 	assert_eq!(unwrapped, Version::SemVer(SemVer::from_str("1.2.3").unwrap()))
//     }
// }
