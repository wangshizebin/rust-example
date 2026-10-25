use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub fn path_op() {
    // convert between Path and str
    let bad_bytes = b"\xFF";
    let path = Path::new(std::ffi::OsStr::from_bytes(bad_bytes));
    let path_str = path.to_str();
    match path_str {
        Some(v) => {
            println!("Directory str: {:?}", v);
        }
        None => {
            println!("Invaild directory");
        }
    }

    let path = Path::new("/Users/wangzebin/Downloads");
    let path_str = path.to_str();
    match path_str {
        Some(v) => {
            println!("Directory str: {:?}", v);
        }
        None => {
            println!("Empty directory");
        }
    }

    // join Path and to juge
    let path_buff = path.join("test1");
    let path = path_buff.as_path();
    println!("Directory joined: {:?}", path);
    let exist = path.exists();
    println!("Directory exist: {:?}", exist);

    //remove dir
    match fs::remove_dir(path) {
        Ok(()) => {
            println!("Deleted successfully: {:?}", path);
        }
        Err(e) => {
            println!("Failed to delete: {}", e);
        }
    }

    match fs::remove_dir(path) {
        Ok(()) => {
            println!("Deleted successfully: {:?}", path);
        }
        Err(e) => {
            println!("Failed to delete: {}", e);
        }
    }

    //create dir
    match fs::create_dir(path) {
        Ok(()) => {
            println!("Created successfully: {:?}", path);
        }
        Err(e) => {
            println!("Failed to create: {}", e);
        }
    }
}

pub fn file_op() {
    let path = Path::new("/Users/wangzebin/Downloads");
    let path = path.join("test1");
    if !path.exists() {
        fs::create_dir(&path).unwrap();
    }

    //read and write file simply
    let file = path.join("1.txt");
    if !file.exists() {
        let content = "AB";
        fs::write(&file, content).unwrap();
    }
    if file.exists() {
        match fs::read_to_string(&file) {
            Ok(content) => {
                println!("The content of the file: {:?}", content);
            }
            Err(e) => {
                println!("Failed to read file: {:?}", e);
            }
        }
    }

    //read and write file with OpenOptions
    match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        //.append(true)
        .open(&file)
    {
        Ok(mut f) => match f.write_all("Hello Rust\n".as_bytes()) {
            Ok(()) => {
                drop(f);
                println!("Write sucessfully");
            }
            Err(e) => {
                println!("Failed to write file: {}", e);
            }
        },
        Err(e) => {
            println!("Failed to open file: {}", e);
        }
    }

    match OpenOptions::new().read(true).open(&file) {
        Ok(mut f) => {
            let mut content = String::new();
            match f.read_to_string(&mut content) {
                Ok(_) => {
                    println!("File content: {}", content);
                }
                Err(e) => {
                    println!("Failed to read file: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to open file: {}", e);
        }
    }
}
