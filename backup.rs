
use std::{fs, io::stdin, path::Path};

fn copy_dir(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir(&source_path, &destination_path)?;
        } else if source_path.is_file() {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

pub fn backup_dir() {
    println!("Enter the path of the directory to back up:");

    let mut dir_path = String::new();

    if stdin().read_line(&mut dir_path).is_err() {
        eprintln!("Failed to read directory path.");
        return;
    }

    let source = Path::new(dir_path.trim());

    if !source.is_dir() {
        eprintln!("Error: Source is not a directory.");
        return;
    }

    let Some(dir_name) = source.file_name() else {
        eprintln!("Invalid source directory path.");
        return;
    };

    let backup_root = Path::new("/run/media/sourav/Sourav/RustProject");

    if let Err(e) = fs::create_dir_all(backup_root) {
        eprintln!("Could not create backup directory: {e}");
        return;
    }

    let destination = backup_root.join(dir_name);

    if destination.exists() {
        eprintln!("A backup with that directory name already exists.");
        return;
    }

    match copy_dir(source, &destination) {
        Ok(()) => println!("Directory backup successful: {}", destination.display()),
        Err(e) => eprintln!("Directory backup failed: {e}"),
    }
}
