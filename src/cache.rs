use colored::*;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind},
    path::Path,
};

use image_hasher::Hasher;

pub fn clean_cache(path_list: Vec<String>, hasher: &Hasher) -> Result<(), Error> {
    let mut file_list = load_cache().unwrap_or(HashMap::new());
    for elem in &path_list {
        cache_elem(&elem, &hasher, &mut file_list)?;
    }
    trim_cache(&mut file_list)?;
    save_cache(file_list);
    Ok(())
}

pub fn trim_cache(file_list: &mut HashMap<String, String>) -> Result<(), Error> {
    for elem in file_list.clone() {
        let path = Path::new(&elem.0);
        if !path.exists() {
            file_list.remove(&elem.0);
            println!("Removed {} from cache", elem.0.red());
        }
    }
    Ok(())
}

pub fn save_cache(files: HashMap<String, String>) {
    if let Ok(file) = File::create("./cache.json") {
        serde_json::to_writer(&file, &files).unwrap();
    }
}

pub fn load_cache() -> Option<HashMap<String, String>> {
    match File::open("./cache.json") {
        Ok(file) => {
            let mut de = serde_json::Deserializer::from_reader(file);
            HashMap::<String, String>::deserialize(&mut de).ok()
        }
        Err(_) => None,
    }
}

pub fn cache_elem(
    item: &str,
    hasher: &Hasher,
    file_list: &mut HashMap<String, String>,
) -> Result<(), Error> {
    if file_list.contains_key(item) {
        eprintln!("{}: {}", &item.bright_green(), file_list[item]);
        return Ok(());
    } else {
        return match image::open(&item) {
            Ok(image) => {
                let hash = hasher.hash_image(&image);
                let b64 = hash.to_base64();
                file_list.insert(String::from(item), String::from(&b64));
                eprintln!("{}: {}", &item.bright_red(), &b64);
                return Ok(());
            }
            Err(error) => Err(Error::new(ErrorKind::Other, error.to_string())),
        };
    }
}
