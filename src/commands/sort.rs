use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string, remove_file, DirEntry, File, ReadDir},
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::args::RageSortOptions;
use rage::portage::package::{PackageAtom, PORTAGE_CONFIGDIR, PORTAGE_PACKAGE_USE};

/// Entry point to the sort subcommand. Depending on whether package.use is a
/// file or directory, `rage_command_sort` will choose the correct function for
/// the job. The procedure for sorting use flags is as follows:
///
/// 1. Determine if package.use is a file or directory.
/// 2. Read from the package.use file(s).
/// 3. Record all package atoms which are mapped by category.
/// 4. If the split option is enabled, delete files and create new ones by
///    category name.
/// 5. Write the recorded package atoms to their respective files.
pub fn rage_command_sort(opts: &RageSortOptions) {
    let package_uses: PathBuf = PathBuf::from(PORTAGE_CONFIGDIR).join(PORTAGE_PACKAGE_USE);

    println!("Sorting USE flags...");

    if package_uses.is_dir() {
        if let Err(why) = process_use_directory(opts, package_uses.as_path()) {
            println!("{}", why);
        }
    } else if let Err(why) = process_use_file(opts, package_uses.as_path()) {
        println!("{}", why);
    }
}

/// Processes the system package.use directory. This should only be called
/// within the `rage_command_sort` function.
fn process_use_directory(opts: &RageSortOptions, path: &Path) -> io::Result<()> {
    let file_map: ReadDir = read_dir(path)?;
    let mut raw_atoms: HashMap<PathBuf, String> = HashMap::new();
    let mut writable_atoms: HashMap<PathBuf, Vec<PackageAtom>> = HashMap::new();

    // read from package.use subfiles
    for use_file in file_map {
        let file_entry: &DirEntry = &use_file?;
        let file_atoms: String = read_to_string(file_entry.path())?;
        raw_atoms.insert(file_entry.path(), file_atoms);
    }

    // check if user wants to split by category
    // TODO: clean this up
    if opts.split {
        for raw_atom_str in raw_atoms.values() {
            let raw_atom_list: Vec<&str> =
                raw_atom_str.split('\n').filter(|s| !s.is_empty()).collect();
            for atom in raw_atom_list {
                match PackageAtom::try_from(atom) {
                    Ok(clean_atom) => {
                        let path: PathBuf = PathBuf::from(PORTAGE_CONFIGDIR)
                            .join(PORTAGE_PACKAGE_USE)
                            .join(clean_atom.category);
                        if let Some(v) = writable_atoms.get_mut(&path) {
                            v.push(clean_atom);
                        } else {
                            writable_atoms.insert(path, vec![clean_atom]);
                        }
                    }
                    Err(why) => return Err(why),
                }
            }
        }

        // remove existing files if split is enabled
        for removable_file in raw_atoms.keys() {
            if let Err(why) = remove_file(removable_file) {
                return Err(why);
            }
        }
    } else {
        for (location, raw_atom_str) in &raw_atoms {
            let raw_atom_list: Vec<&str> =
                raw_atom_str.split('\n').filter(|s| !s.is_empty()).collect();
            for atom in raw_atom_list {
                match PackageAtom::try_from(atom) {
                    Ok(clean_atom) => {
                        if let Some(v) = writable_atoms.get_mut(location) {
                            v.push(clean_atom);
                        } else {
                            writable_atoms.insert(location.to_path_buf(), vec![clean_atom]);
                        }
                    }
                    Err(why) => return Err(why),
                }
            }
        }
    }

    for atom_vec in writable_atoms.values_mut() {
        atom_vec.sort_by_key(|a| a.to_string());
    }

    // pass map to file writer
    for (writable_path, clean_atom_list) in writable_atoms {
        if let Err(why) = write_to_file(writable_path.as_path(), clean_atom_list, opts.pretend_mode)
        {
            return Err(why);
        }
    }

    Ok(())
}

fn process_use_file(opts: &RageSortOptions, path: &Path) -> io::Result<()> {
    let Ok(raw_atoms) = read_to_string(path) else {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Insufficient permissions: unable to read the package.use file",
        ));
    };

    let atom_list: Vec<&str> = raw_atoms.split('\n').filter(|s| !s.is_empty()).collect();
    let mut clean_atom_list: Vec<PackageAtom> = Vec::new();

    for atom in atom_list {
        match PackageAtom::try_from(atom) {
            Ok(clean_atom) => {
                clean_atom_list.push(clean_atom);
            }
            Err(why) => return Err(why),
        }
    }

    clean_atom_list.sort_by_key(|a| a.to_string());
    if let Err(why) = write_to_file(path, clean_atom_list, opts.pretend_mode) {
        return Err(why);
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
