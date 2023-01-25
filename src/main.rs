use markdown_gen::markdown::*;
use std::fs::{self, OpenOptions, File};
use std::io::Write;
use chrono::Datelike;
use chrono::prelude::*;
use num_traits::cast::FromPrimitive;

#[macro_use]
extern crate fstrings;

fn main() {

    let current_date = chrono::Local::now();

    let year = current_date.year();
    let month = current_date.month();
    let month_name = Month::from_u32(current_date.month()).expect("Current month should be found in this enum").name();
    let day = current_date.day();

    let file_name = f!("{year}/{month_name}/{year}-{month}-{day}.md");

    println!("{}", &file_name);

    //ensure that the path exists to create the file
    let path = std::path::Path::new(&file_name);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let file = File::create(&file_name).unwrap();
    let mut md = Markdown::new(file);

    md.write(f!("{year}-{month}-{day}").heading(1)).unwrap();

    md.write(
        List::new(false)
            .title("Activity")
            .item(r"[x] did this")
            .item(r"[x] did that")
            .item(r"[ ] didn't get to this however")
    ).unwrap();

    md.write("Bob said i should check the whos-a-What".quote()).unwrap(); 


    // ------------------------------------------------
    //Then unescape everything cause its annoying
    let contents = fs::read_to_string(&file_name).unwrap();
    let new = contents.replace("\\", "");

    let mut file = OpenOptions::new().write(true).truncate(true).open(&file_name).unwrap();
    file.write(new.as_bytes()).unwrap();
}

