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
use std::fs::{create_dir_all, copy, DirEntry};
use std::io;

fn main() {
    let matches = App::new("photo_copier")
                          .version("1.0")
                          .author("Rob Day <rkd@rkd.me.uk>")
                          .about("Copies camera photos to a new location")
                          .args_from_usage(
                              "--input=<inputdir>  'Sets the source directory'
                               --output=<outputdir> 'Sets the output directory'
                               --dry-run      'Don't actually move files'")
                          .get_matches();

    // Same as previous example...
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    println!("Copying from {} to {}", input, output);
    let dry_run = matches.is_present("dry-run");
    if dry_run {
        println!("Dry run only");
    }

    let input_path = Path::new(input);
    let output_path = Path::new(output);
    match copy_photos(input_path, output_path, dry_run) {
        Ok(num) => { println!("{} photos copied", num); },
        Err(e) => { println!("Failure: {}", e) }
    }
}

fn copy_photos(input_path: &Path, output_path: &Path, dry_run: bool) -> io::Result<u64> {
    let mut copied = 0;
    let dir_entries: Vec<io::Result<DirEntry>> = input_path.read_dir()?.collect();
    for (num, entry) in dir_entries.iter().enumerate() {
        if let Ok(entry) = entry {
            if let Some(fname) = entry.path().file_name() {
                let unix_timestamp = entry.metadata().unwrap().created().unwrap().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
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
                    if output_file.exists() && output_file.metadata().unwrap().len() == entry.path().metadata().unwrap().len() {
                        println!("{} already exists at the correct file size, not copying", output_file.display());
                    } else {
                        println!("Copying {} to {} ({}/{})",
                        entry.path().display(),
                        output_file.display(),
                        num+1,
                        dir_entries.len());
                        let result = copy(entry.path(), output_file);
                        match result {
                            Ok(_) => { copied += 1; },
                            Err(e) => { println!("{} failed to copy: {}", entry.path().display(), e) },
                        }
                    }
                }
            }
        }
    }
    return Ok(copied);
}
