use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};


pub fn last_char_of_first_line(text : &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn test_propagation_errors() -> Result<String, io::Error> {
    let path_str = "/home/ivan/documents/lic1.xml";

    let mut user_file = match File::open(path_str) {
        Ok(file) =>  file,
        Err(error) => return Err(error)  //возвращаем Err
    };

    let mut user_name = String::from("TEST");

    match user_file.read_to_string(&mut user_name) {
        Ok(_) => { Ok(user_name) }
        Err(e) => { Err(e) }
    }
}

pub fn test_less_propagation_errors() -> Result<String, io::Error> {
    let path_str = "/home/ivan/documents/lic1.xml";

    let mut user_file = File::open(path_str)?;

    let mut user_name = String::new();

    user_file.read_to_string(&mut user_name)?;

    Ok(user_name)
}

//Open or Create file match style
pub fn test_match_exc() {
    let path_str = "/home/ivan/documents/lic1.xml";
    let file = match File::open(path_str) {
        Ok(_) => {
            println!("File {} opened!", path_str)
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create(path_str) {
                    Ok(file) => { println!("File {} created!", path_str) }
                    Err(error) => { panic!("Error while create file {:?}", error) }
                }
                other => {
                    println!("Error while open file! {:?}", other)
                }
            }
        }
    };
}

//Open or Create file if style
pub fn test_if_exc() {
    let path_str = "/home/ivan/documents/lic1.xml";
    let file = File::open(path_str).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(path_str).unwrap_or_else(|error| {
                panic!("Error while create file {:?}", error)
            })
        } else {
            panic!("Error while open file {:?}", error)
        }
    });
}

//Open file expect style
pub fn test_expect_exc() {
    let path_str = "/home/ivan/documents/lic1.xml";
    let file = File::open(path_str).expect("Error while open file");
}

