

extern crate serde_json;

use std::path::Path; 
use std::fs::File;
use std::io::Write;
use std::error::Error;
use PassEntry;
//use serde::*;

pub fn save_passwords(passwords: &Vec<PassEntry>){
    // os independent path
    let path = Path::new("db.txt");
    let display = path.display();
    
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };
    
    let serialized = serde_json::to_string(passwords).unwrap();
    match writeln!(file, "{}", serialized) {
        Err(why) => {panic!("couldn't write to {}: {}", display, Error::description(&why))},
        Ok(_) => {}//println!("successfully wrote to {}", display),
    }
    
    //for pass in passwords {
        //match file.write_all(pass1.title.as_bytes()) {
    //}        
}
