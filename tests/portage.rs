#[cfg(test)]
mod tests {
    use rage::portage::package::{PackageAtom, UseFlag};

    #[test]
    fn parse_package_atom_normal() {
        let Ok(atom) = PackageAtom::try_from("sys-apps/portage-3.0.55 doc") else {
            panic!("Unable to parse correct package atom");
        };
        assert_eq!(atom.category, "sys-apps");
        assert_eq!(atom.name, "portage");
        assert_eq!(atom.version.unwrap(), "3.0.55");
        assert_eq!(
            atom.flags.first().unwrap().to_string(),
            UseFlag::from("doc").to_string()
        );
    }

    #[test]
    fn parse_package_atom_whitespace() {
        assert!(PackageAtom::try_from("\t\t    sys-apps/portage-3.0.55  \t  doc").is_ok());
    }

    #[test]
    fn parse_package_atom_invalid() {
        assert!(PackageAtom::try_from("random-package some flags here").is_err());
    }
}
