extern crate failure;
extern crate clap;

use failure::{Error,err_msg};
use clap::{Arg, App};
use std::fs;
use std::path::{Path,PathBuf};
use std::io::ErrorKind;

fn main() {
    if let Err(err) = main2() {
        eprintln!("ERROR: {}", err);
        std::process::exit(1);
    }
}

fn main2() -> Result<(), Error> {
    let matches = App::new("islink")
        .version("1.0")
        .author("Miles Steele")
        .about("Find out whether a path contains any symlinks")
        .arg(Arg::with_name("path")
             .takes_value(true)
             .required(true))
        .get_matches();

    let full_path = Path::new(matches.value_of("path").ok_or(err_msg("missing path arg"))?);
    let mut path = PathBuf::new();
    for part in full_path.iter() {
        path.push(part);
        // println!("\n{:?}", path);
        match fs::symlink_metadata(&path) {
            Ok(meta) => {
                if meta.file_type().is_symlink() {
                    let target = fs::read_link(&path)?;
                    println!("{} -> {}", path.display(), target.display());
                }
            }
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    println!("{}: {}", path.display(), err);
                    return Ok(());
                }
                Err(err)?;
            }
        }
    }
    Ok(())
}
