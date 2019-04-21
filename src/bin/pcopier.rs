// (Full example with detailed comments in examples/01a_quick_example.rs)
//
// This example demonstrates clap's "usage strings" method of creating arguments
// which is less verbose
extern crate clap;
extern crate chrono;

use chrono::{Datelike, NaiveDateTime};
use std::time::SystemTime;

use clap::App;
use std::path::Path;
use std::fs::{create_dir_all, copy, DirEntry, remove_file};
use std::io;

fn main() {
    let matches = App::new("photo_copier")
                          .version("1.0")
                          .author("Rob Day <rkd@rkd.me.uk>")
                          .about("Copies camera photos to a new location")
                          .args_from_usage(
                              "--input=<inputdir>  'Sets the source directory'
                               --output=<outputdir> 'Sets the output directory'
                               --rm 'Remove files after copying (or if already copied)'
                               --dry-run      'Don't actually move files'")
                          .get_matches();

    // Same as previous example...
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    println!("Copying from {} to {}", input, output);
    let dry_run = matches.is_present("dry-run");
    let remove = matches.is_present("rm");
    if dry_run {
        println!("Dry run only");
    }

    let input_path = Path::new(input);
    let output_path = Path::new(output);
    match copy_photos(input_path, output_path, dry_run, remove) {
        Ok(num) => { println!("{} photos copied", num); },
        Err(e) => { println!("Failure: {}", e) }
    }
}

fn copy_photos(input_path: &Path, output_path: &Path, dry_run: bool, remove: bool) -> io::Result<u64> {
    let mut copied = 0;
    let dir_entries: Vec<io::Result<DirEntry>> = input_path.read_dir()?.collect();
    println!("{} files to process", dir_entries.len());
    for (num, entry) in dir_entries.iter().enumerate() {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                println!("Processing {}", entry.path().display());
                if let Some(fname) = entry.path().file_name() {
                    let unix_timestamp = entry.metadata().unwrap().modified().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
                    let ts = NaiveDateTime::from_timestamp(unix_timestamp as i64, 0);
                    let dated_output_dir = output_path.join(ts.year().to_string()).join(ts.month().to_string());
                    if ! dated_output_dir.exists() {
                        if dry_run {
                            println!("Not creating {} (dry run)", dated_output_dir.display());
                        } else {
                            create_dir_all(&dated_output_dir)?;
                        }
                    }
                    let output_file = dated_output_dir.join(fname);
                    if dry_run {
                        println!("Not copying {} to {} ({}/{}) (dry run)",
                        entry.path().display(),
                        output_file.display(),
                        num+1,
                        dir_entries.len());
                    } else {
                        if output_file.exists() {
                            if output_file.metadata().unwrap().len() == entry.path().metadata().unwrap().len() {
                                println!("{} already exists at the correct file size, not copying", output_file.display());
                                if remove {
                                    remove_file(entry.path())?;
                                }
                            } else {
                                return Err(io::Error::new(io::ErrorKind::Other, format!("{} already exists but is different", output_file.display())));
                            }
                        } else {
                            println!("Copying {} to {} ({}/{})",
                            entry.path().display(),
                            output_file.display(),
                            num+1,
                            dir_entries.len());
                            copy(entry.path(), output_file)?;
                            if remove {
                                remove_file(entry.path())?;
                            }
                            copied += 1;
                        }
                    }
                }
            }
        }
    }
    return Ok(copied);
}
