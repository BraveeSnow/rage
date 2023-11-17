use std::io;

use once_cell::sync::Lazy;
use regex::Regex;

// Constant variables for portage
pub const PORTAGE_CONFIGDIR: &str = "/etc/portage";
pub const PORTAGE_PACKAGE_USE: &str = "package.use";

pub const PORTAGE_VERSION_DELIMITER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"-\d").expect("Portage version delimiter failed to compile"));

/// The struct describing a package atom. To create a package atom from a
/// string, it is recommended to call the `try_from` function.
#[derive(Clone, Debug)]
pub struct PackageAtom<'a> {
    pub name: &'a str,
    pub category: &'a str,
    pub version: Option<&'a str>,
    pub flags: Vec<UseFlag<'a>>,
}

impl<'a> ToString for PackageAtom<'a> {
    /// Converts a package atom struct to a string readable by portage.
    fn to_string(&self) -> String {
        let mut package: String = format!("{}/{}", self.category, self.name);
        if let Some(v) = self.version {
            package.push_str(format!("-{}", v).as_str());
        }

        for flag in self.flags.as_slice() {
            package.push_str(format!(" {}", flag.to_string()).as_str());
        }

        package
    }
}

impl<'a> TryFrom<&'a str> for PackageAtom<'a> {
    type Error = io::Error;

    /// Parses a string containing data of a package atom. The string passed
    /// does not need to be trimmed as it is already done internally.
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let trimmed: &'a str = value.trim();
        let mut tokens: Vec<&str> = trimmed.split_ascii_whitespace().collect();
        let mut version: Option<&'a str> = None;

        // process name and category
        let Some(atom_split) = tokens.first() else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid package atom: {}", trimmed),
            ));
        };
        let Some(atom_tuple) = atom_split.split_once('/') else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid package atom: {}", trimmed),
            ));
        };
        let (category, mut name): (&'a str, &'a str) = atom_tuple;

        // check if version is specified
        if let Some(version_match) = PORTAGE_VERSION_DELIMITER.find(name) {
            version = Some(&name[version_match.start() + 1..]);
            name = &name[..version_match.start()];
        };

        tokens.remove(0);

        Ok(PackageAtom {
            name,
            category,
            version,
            flags: tokens.into_iter().map(UseFlag::from).collect(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct UseFlag<'a> {
    pub name: &'a str,
    pub enabled: bool,
}

impl<'a> From<&'a str> for UseFlag<'a> {
    fn from(value: &'a str) -> Self {
        let trimmed: &str = value.trim();
        UseFlag {
            name: trimmed,
            enabled: !trimmed.starts_with('-'),
        }
    }
}

impl<'a> ToString for UseFlag<'a> {
    fn to_string(&self) -> String {
        self.name.to_owned()
    }
}
