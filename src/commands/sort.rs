use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string, remove_file, File, ReadDir},
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::args::RageSortOptions;
use rage::portage::package::{PackageAtom, PORTAGE_CONFIGDIR, PORTAGE_PACKAGE_USE};

pub fn rage_command_sort(opts: &RageSortOptions) {
    let package_uses: PathBuf = PathBuf::from(PORTAGE_CONFIGDIR).join(PORTAGE_PACKAGE_USE);

    println!("Sorting USE flags...");

    if package_uses.is_dir() {
        if let Err(why) = process_use_directory(opts, package_uses.as_path()) {
            println!("{}", why);
        }
    }
}

fn process_use_directory(opts: &RageSortOptions, path: &Path) -> io::Result<()> {
    let file_map: ReadDir = read_dir(path)?;
    let mut raw_atoms: String = String::new();
    let mut categories: HashMap<&str, Vec<PackageAtom>> = HashMap::new();

    for use_file in file_map {
        let use_file_path = use_file?.path();
        let file_contents: String = read_to_string(path.join(use_file_path.clone()))?;
        raw_atoms.push_str(file_contents.as_str());

        if !opts.pretend_mode {
            if let Err(why) = remove_file(use_file_path) {
                return Err(why);
            }
        }
    }

    let atom_list: Vec<&str> = raw_atoms.split('\n').filter(|s| !s.is_empty()).collect();
    for atom in atom_list {
        let mut clean_atom: PackageAtom = PackageAtom::from(atom);

        if opts.remove_versions {
            clean_atom.version = None;
        }

        if let Some(v) = categories.get_mut(clean_atom.category) {
            v.push(clean_atom);
        } else {
            categories.insert(clean_atom.category, vec![clean_atom]);
        }
    }

    for mut category_pair in categories {
        category_pair.1.sort_by_key(|a| a.name);

        if let Err(why) = write_to_file(
            path.join(category_pair.0).as_path(),
            category_pair.1,
            opts.pretend_mode,
        ) {
            return Err(why);
        }
    }

    Ok(())
}

fn write_to_file(file_path: &Path, atoms: Vec<PackageAtom>, pretend_mode: bool) -> io::Result<()> {
    let file_str: Vec<String> = atoms.iter().map(|a| a.to_string()).collect();
    println!("Wrote to file {}", file_path.display());

    if pretend_mode {
        println!("{}\n", file_str.join("\n"));
        return Ok(());
    }

    let mut category_file: File = File::create(file_path)?;
    if let Err(why) = category_file.write_all(file_str.join("\n").as_bytes()) {
        return Err(why);
    }

    Ok(())
}
