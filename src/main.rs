use std::{
    fs, io, path::{Path, PathBuf}
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "File organizer")]
#[command(version)]
#[command(about = "A program to organize files from a given directory")]
struct Args {
    #[arg(short, long, help="source directory")]
    src: PathBuf,

    #[arg(short, long, help="target directory")]
    tgt: PathBuf,
}

fn main() {
    let args = Args::parse();

    match copy_dir_all(&args.src, &args.tgt) {
        Ok(_) => println!("Copy successfully !"),
        Err(e) => eprintln!("Erreur: {}", e),
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
