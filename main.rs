
use std::{fs, io, path::Path};

fn main() {
    println!("=== SafeBack ===");
    println!("1 : Backup file");
    println!("2 : Delete file");
    println!("Enter your choice:");

    let mut ch = String::new();

    io::stdin()
        .read_line(&mut ch)
        .expect("Failed to read input");

    match ch.trim() {
        "1" => {
            println!("Enter the file path to back up:");

            let mut file_path = String::new();

            io::stdin()
                .read_line(&mut file_path)
                .expect("Failed to read file path");

            let source = Path::new(file_path.trim());

            if !source.is_file() {
                println!("Error: That file does not exist!");
                return;
            }

            let backup_dir = Path::new("/home/sourav/RustBackUp");

            if let Err(e) = fs::create_dir_all(backup_dir) {
                println!("Could not create backup directory: {e}");
                return;
            }

            let Some(file_name) = source.file_name() else {
                println!("Error: Invalid file name!");
                return;
            };

            let destination = backup_dir.join(file_name);

            if destination.exists() {
                println!("Error: Backup already exists. Nothing overwritten.");
                return;
            }

            match fs::copy(source, &destination) {
                Ok(bytes) => println!(
                    "Backup successful: {} ({} bytes)",
                    destination.display(),
                    bytes
                ),
                Err(e) => println!("Backup failed: {e}"),
            }
        }

        "2" => {
            println!("Delete file selected — not implemented yet.");
        }

        _ => println!("Invalid choice!"),
    }
}
