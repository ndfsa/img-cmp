use colored::*;
use img_hash::{HashAlg, Hasher, HasherConfig, ImageHash};
use serde::Deserialize;
use sha1::Digest;
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Error, Result},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        match parse_cmd(&args[1], flatten_list(args[2..].to_vec())) {
            Ok(()) => (),
            Err(e) => println!("{}", e.to_string()),
        }
    }
}

fn parse_cmd(cmd: &str, path_list: Vec<String>) -> Result<()> {
    match cmd {
        "rename" => {
            for elem in path_list {
                rename(&elem)?;
            }
        }
        "run" => run(path_list)?,
        "cache" => cache(path_list)?,
        _ => panic!("Unknown command {}", cmd),
    }
    Ok(())
}

fn cache(path_list: Vec<String>) -> Result<()> {
    let hasher = HasherConfig::new()
        .preproc_dct()
        .hash_alg(HashAlg::Mean)
        .to_hasher();
    let mut file_list = load_cache();
    for elem in &path_list {
        cache_elem(&elem, &hasher, &mut file_list)?;
    }
    trim_cache(&mut file_list)?;
    save_cache(file_list);
    Ok(())
}

fn run(path_list: Vec<String>) -> Result<()> {
    let hasher = HasherConfig::new()
        .preproc_dct()
        .hash_alg(HashAlg::Mean)
        .to_hasher();
    let mut file_list = load_cache();
    for elem in path_list {
        cache_elem(&elem, &hasher, &mut file_list)?;
    }
    find_duplicates(&file_list);
    save_cache(file_list);
    Ok(())
}

fn rename(item: &str) -> Result<()> {
    let path = Path::new(item);
    let mut file = fs::File::open(&path)?;

    let mut hasher = sha1::Sha1::new();

    io::copy(&mut file, &mut hasher)?;

    if let Some(ext) = path.extension() {
        let new_path = path.with_file_name(format!(
            "{:x}.{}",
            hasher.finalize(),
            ext.to_ascii_lowercase().to_string_lossy()
        ));
        fs::rename(&path, &new_path)?;
        println!(
            "\nold path: {}\nnew path: {}",
            path.to_str().unwrap(),
            new_path.to_str().unwrap()
        );
    } else {
        println!("{}: Could not find extension", path.to_str().unwrap());
    }

    Ok(())
}

fn trim_cache(file_list: &mut HashMap<String, String>) -> Result<()> {
    for elem in file_list.clone() {
        let path = Path::new(&elem.0);
        if !path.exists() {
            file_list.remove(&elem.0);
            println!("Removed {} from cache", elem.0.red());
        }
    }
    Ok(())
}

fn find_duplicates(file_list: &HashMap<String, String>) {
    let mut lookup = file_list.clone();
    for elem1 in file_list {
        lookup.remove(elem1.0);
        if let Ok(hash1) = ImageHash::<Box<[u8]>>::from_base64(elem1.1) {
            for elem2 in &lookup {
                if let Ok(hash2) = ImageHash::<Box<[u8]>>::from_base64(elem2.1) {
                    let diff = hash1.dist(&hash2);
                    if diff < 2 {
                        println!("{} {} {}", elem1.0, elem2.0, diff);
                    }
                } else {
                    println!("Could not read hash: {} from file {}", elem2.1, elem2.0)
                }
            }
        } else {
            println!("Could not read hash: {} from file {}", elem1.1, elem1.0)
        }
    }
}

fn save_cache(files: HashMap<String, String>) {
    if let Ok(file) = File::create("./cache.json") {
        serde_json::to_writer(&file, &files).unwrap();
        // match serde_json::to_writer(&file, &files) {
        //     Ok(_) => println!("{}", "Saved cache".green()),
        //     Err(_) => println!("{}", "Could not save cahce".red()),
        // }
    }
}

fn load_cache() -> HashMap<String, String> {
    if let Ok(file) = File::open("./cache.json") {
        let mut de = serde_json::Deserializer::from_reader(file);
        HashMap::<String, String>::deserialize(&mut de).unwrap_or(HashMap::new())
    } else {
        // println!("{}", "Could not find cache file".red());
        HashMap::new()
    }
}

fn cache_elem(item: &str, hasher: &Hasher, file_list: &mut HashMap<String, String>) -> Result<()> {
    if file_list.contains_key(item) {
        // println!("{}: {}", &item.bright_green(), file_list[item]);
        return Ok(());
    } else {
        return match image::open(&item) {
            Ok(image) => {
                let hash = hasher.hash_image(&image);
                let b64 = hash.to_base64();
                file_list.insert(String::from(item), String::from(&b64));
                // println!("{}: {}", &item.bright_red(), &b64);
                return Ok(());
            }
            Err(error) => Err(Error::new(io::ErrorKind::Other, error.to_string())),
        };
    }
}

fn flatten_list(args: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for elem in args {
        let path = Path::new(&elem);
        if !path.exists() {
            println!("\"{}\" not found", path.to_str().unwrap());
            continue;
        }

        let path = path.canonicalize().unwrap();
        if path.is_dir() {
            result.append(&mut flatten_list(
                path.read_dir()
                    .unwrap()
                    .map(|sub_elem| {
                        String::from(sub_elem.expect("Cannot get file").path().to_string_lossy())
                    })
                    .collect(),
            ));

        // one day, symlinks will be supported
        // } else if path.is_symlink() {
        //     handle_paths(vec![path.into_os_string().into_string().unwrap()]);
        } else {
            if path.file_name().unwrap() != "cache.json" {
                result.push(path.into_os_string().into_string().unwrap());
            }
        }
    }
    result
}
