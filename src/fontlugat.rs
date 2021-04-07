// use std::fs;
use cell::Cell;
use ffi::OsString;
use fs::{File, OpenOptions};
use io::prelude::*;
use path::{Path, PathBuf};
use std::{cell, ffi, fs, io, path};
use walkdir::WalkDir;

pub fn init(dir: &str) -> () {
    // Getting ABSPATH
    match fs::canonicalize(PathBuf::from(&dir)) {
        Ok(dir) => {
            // Converting PathBuf to OS string
            let fonts: Vec<[String; 2]> = collect(dir.into_os_string());
            println!("\n{:?}\n", fonts);
        }
        Err(e) => {
            println!("{:?} doesn't exist.\n\n{:?}", dir, e)
        }
    }
}

pub fn collect(dir: OsString) -> Vec<[String; 2]> {
    let skipped = ["".to_string(), "".to_string()];

    let mut fonts = vec![];
    for entry in WalkDir::new(&dir) {
        match &entry {
            Ok(entry) => match get_fonts(entry) {
                font => {
                    if font != skipped {
                        fonts.push(font);
                    }
                }
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }

    return fonts;
}

pub fn get_fonts(entry: &walkdir::DirEntry) -> [String; 2] {
    let entry_path = Path::new(entry.path());
    let skipped = ["".to_string(), "".to_string()];
    return match entry_path.extension() {
        Some(ext) => match ext.to_str() {
            // Some("zip") => {
            //     return 1;
            // }
            Some(ext) => match is_font(ext) {
                true => {
                    let font = Font::new(entry, ext);
                    let component = Font::generate(font);
                    return component;
                }
                false => skipped,
            },
            None => skipped,
        },
        None => skipped,
    };
}

#[derive(Debug)]
pub struct Font {
    id: usize,
    name: String,
    filename: String,
    ext: String,
    path: String,
}

thread_local!(static FONT_ID: Cell<usize> = Cell::new(1));

impl Font {
    fn new(file: &walkdir::DirEntry, ext: &str) -> Font {
        return FONT_ID.with(|thread_id| {
            let font_id = thread_id.get();
            thread_id.set(font_id + 1);
            let font_filename_ref = file.file_name().to_str();
            let font_filename = String::from(opt_ref_to_string(font_filename_ref));
            let font_path = file.path().to_str();
            let font_filename_arr: Vec<&str> = font_filename.split(".").collect();
            let font_name = font_filename_arr.first().unwrap().to_string();
            match font_filename {
                _ => {
                    return Font {
                        id: font_id,
                        name: font_name,
                        filename: font_filename,
                        ext: ext.to_string(),
                        path: opt_ref_to_string(font_path),
                    }
                }
            }
        });
    }

    // fn css(font_css: String) -> io::Result<()> {
    //     return writeln!(
    //         OpenOptions::new()
    //             .write(true)
    //             .append(true)
    //             .create(true)
    //             .open("./src/templates/fonts.css")
    //             .unwrap(),
    //         "{}",
    //         font_css
    //     );
    // };

    // fn html(font_html: String) -> io::Result<()> {
    //     return writeln!(
    //         OpenOptions::new()
    //             .write(true)
    //             .append(true)
    //             .create(true)
    //             .open("./src/templates/template.font.html")
    //             .unwrap(),
    //         "{}",
    //         font_html
    //     );
    // };

    fn generate(self) -> [String; 2] {
        let font_css = format!(
            "{start}{name:?}; src:url({path:?}){end}",
            start = "@font-face{font-family: ",
            name = self.name,
            path = self.path,
            end = "}"
        );
        let font_html = format!(
            "{start}{name}{family}'{name}'{end}",
            start = "<div class=\"fontholder\"><label>",
            family = "</label><input style=\"font-family:",
            name = self.name,
            end = "\" type=\"text\" value=\"The quick brown fox jumps over the lazy dog.\"><input onchange=\"changeSize(this)\" type=\"range\" min=\"4\" max=\"80\" value=\"40\"><span class=\"size\">40px / 30pt / 2.5rem</span></div>"
        );
        // let font_css_buffer = Font::css(font_css);
        // let font_html_buffer = Font::html(font_css);
        let array = [font_css, font_html];
        return array;
    }
}

fn opt_ref_to_string(x: Option<&str>) -> String {
    return x.unwrap().to_string();
}

pub fn is_font(x: &str) -> bool {
    match x {
        // Supported formats
        "otf" | "woff" | "woff2" | "eot" | "ttf" => {
            return true;
        }
        _ => false,
    }
}

/*
pub fn unzip(entry_path: &Path) -> i32 {
    let zip_dir = fs::File::open(entry_path).unwrap();
    // let mut archive = zip::ZipArchive::new(zip_dir).unwrap();
    // for i in 0..archive.len() {
    //     let file = archive.by_index(i).unwrap();
    //     let outpath = match file.enclosed_name() {
    //         Some(path) => {
    //             let test = path.to_owned();
    //             println!("{:?}, ", test);
    //         }
    //         None => continue,
    //     };
    // }
    // {
    //     let comment = file.comment();
    //     if !comment.is_empty() {
    //         println!("File {} comment: {}", i, comment);
    //     }
    // }

    // if (&*file.name()).ends_with('/') {
    //     println!("File {} extracted to \"{}\"", i, outpath.display());
    //     fs::create_dir_all(&outpath).unwrap();
    // } else {
    //     println!(
    //         "File {} extracted to \"{}\" ({} bytes)",
    //         i,
    //         outpath.display(),
    //         file.size()
    //     );
    //     if let Some(p) = outpath.parent() {
    //         if !p.exists() {
    //             fs::create_dir_all(&p).unwrap();
    //         }
    //     }
    //     let mut outfile = fs::File::create(&outpath).unwrap();
    //     io::copy(&mut file, &mut outfile).unwrap();
    // }
    // // Get and Set permissions
    // #[cfg(unix)]
    // {
    //     use std::os::unix::fs::PermissionsExt;

    //     if let Some(mode) = file.unix_mode() {
    //         fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
    //     }
    // }
    return 1;
}
*/
