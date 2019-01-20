#[macro_use] extern crate nickel;
extern crate serde;
extern crate walkdir;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate env_logger;

use nickel::{Nickel, StaticFilesHandler, QueryString};
use walkdir::WalkDir;
use std::collections::HashMap;
use std::path::Path;
use std::ffi::OsStr;

#[derive(Serialize)]
struct PageData<'a> {
    data: i32,
    image: &'a str,
    next: &'a str,
    prev: &'a str,
    dir: &'a str
}

#[derive(Serialize)]
struct DirContents {
    images: Vec<String>,
    subdirs: Vec<String>,
}

static dir_s: &'static str = "/Volumes/Seagate Backup Plus Drive/Media/Pictures";

fn get_dir_contents(path: &str) -> DirContents {
    let dir = Path::new(dir_s);
    let mut images = vec!();
    let mut subdirs = vec!();
    let truedir = dir.join(path);
    println!("{}", truedir.display());
    for entry in WalkDir::new(truedir).max_depth(1).min_depth(0) {
        let e = entry.unwrap();
        let p = e.path();
        if (p.is_dir()) {
            let filename = p.strip_prefix(dir_s).unwrap().to_str().unwrap().to_owned();
            subdirs.push(filename);
        } else {
            let ext = p.extension().unwrap();
            if ((ext == "jpg") || (ext == "JPG")) {
                let filename = p.strip_prefix(dir_s).unwrap().to_str().unwrap().to_owned();
                images.push(filename);
            }
        }
    }
    DirContents{images: images, subdirs: subdirs}
}

fn main() {
    env_logger::init();
    debug!("this is a debug {:?}", "message");
    error!("this is printed by default");
    let mut server = Nickel::new();

    server.utilize(router! {
        get "/picture" => |req, res| {
            let picture = req.query().get("pic").unwrap().to_owned();
            let pic_path = Path::new(&picture).parent().unwrap().to_str().unwrap();
            let folder_contents = get_dir_contents(pic_path);
            let pic_basename = Path::new(&picture).file_name().unwrap();
            println!("{:?}", pic_basename);
            let idx = folder_contents.images.iter().position(|elem|
                                                             Path::new(&elem).file_name().unwrap_or(OsStr::new("foo")) == pic_basename);
            println!("idx {}", idx.unwrap());
            let maxidx = folder_contents.images.len() - 1;
            let prev = match idx {
                None => &picture,
                Some(idx2) => if (idx2 == 0) { &picture } else { folder_contents.images.get(idx2 - 1).unwrap() }
            };

            let next = match idx {
                None => &picture,
                Some(idx2) => if (idx2 == maxidx) { &picture } else { folder_contents.images.get(idx2 + 1).unwrap() }
            };

            let data = PageData { data: 5, image: &picture, next: next, prev: prev, dir: pic_path };
            return res.render("assets/template.tpl", &data);
        }
    });
    server.utilize(router! {
        get "/listdir" => |req, res| {
            let subdir = req.query().get("dir").unwrap_or("./");
            let pd = get_dir_contents(subdir);
            return res.render("assets/list.tpl", &pd);
        }
    });

    server.utilize(StaticFilesHandler::new(dir_s));

    server.listen("127.0.0.1:4080");
}
