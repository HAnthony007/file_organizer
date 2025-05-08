use std::{env, fs, io, path::{Path, PathBuf}};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: organize <source-directory> <target-directory>");
        return;
    }

    let source = Path::new(&args[1]);
    let target = Path::new(&args[2]);

    if !source.exists() || !source.is_dir() {
        eprintln!("Erreur: Le dossier source n'existew pas ou n'est pas un dossier. ");
        return;
    }

    if !target.exists() {
        if let Err(e) = fs::create_dir_all(&target) {
            eprintln!("Erreur lors de la creation du dossier cible: {}. ", e);
            return;
        }
    }

    if let Err(e) = organize_files(source, target) {
        eprintln!("Erreur: {}. ", e);
        return;
    }

    println!("Organisation terminee avec succes ! ");
}

fn organize_files(source: &Path, target: &Path) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            organize_files(&path, target)?;
        } else if path.is_file() {
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown");

            let mut target_subdir = PathBuf::from(target);
            target_subdir.push(extension);

            if !target_subdir.exists() {
                fs::create_dir_all(&target_subdir)?;
            }

            let file_name = path.file_name().unwrap();
            let mut destination = target_subdir;
            destination.push(file_name);

            fs::rename(&path, &destination)?;
            println!("Deplace: {} => {}", path.display(), destination.display())
        }
    }

    Ok(())
}
