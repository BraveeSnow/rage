use once_cell::sync::Lazy;
use regex::{Match, Regex};

// Constant variables for portage
pub const PORTAGE_CONFIGDIR: &str = "/etc/portage";
pub const PORTAGE_PACKAGE_USE: &str = "package.use";

pub const PORTAGE_VERSION_DELIMITER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"-\d").expect("Portage version delimiter failed to compile"));

#[derive(Clone, Debug)]
pub struct PackageAtom<'a> {
    pub name: &'a str,
    pub category: &'a str,
    pub version: Option<&'a str>,
    pub flags: Vec<UseFlag<'a>>,
}

impl<'a> From<&'a str> for PackageAtom<'a> {
    /// Takes a string and converts it into a PackageAtom.
    fn from(value: &'a str) -> Self {
        let mut tokens: Vec<&str> = value.split_ascii_whitespace().collect();
        let mut name_split: (&str, &str) = tokens
            .get(0)
            .expect("Package atom token collection failed")
            .split_once('/')
            .expect("Invalid package atom");
        let version_pos: Option<Match<'_>> = PORTAGE_VERSION_DELIMITER.find(name_split.1);
        let version: Option<&str> = if let Some(pos) = version_pos {
            let v = Some(&name_split.1[pos.start()..]);
            name_split.1 = &name_split.1[..pos.start()];
            v
        } else {
            None
        };

        tokens.remove(0);

        PackageAtom {
            name: name_split.1,
            category: name_split.0,
            version,
            flags: tokens.into_iter().map(UseFlag::from).collect(),
        }
    }
}

impl<'a> ToString for PackageAtom<'a> {
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
