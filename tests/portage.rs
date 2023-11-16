#[cfg(test)]
mod tests {
    use rage::portage::package::{PackageAtom, UseFlag};

    #[test]
    fn parse_package_atom() {
        let atom: PackageAtom = PackageAtom::from("sys-apps/portage-3.0.55 doc");
        assert_eq!(atom.category, "sys-apps");
        assert_eq!(atom.name, "portage");
        assert_eq!(atom.version.unwrap(), "3.0.55");
        assert_eq!(
            atom.flags.first().unwrap().to_string(),
            UseFlag::from("doc").to_string()
        );
    }
}
