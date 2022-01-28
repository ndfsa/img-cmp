use img_hash::{HashAlg, HasherConfig, ImageHash};
use sha1::Digest;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Result},
    path::Path,
};
mod cache;

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
        "cache" => cache::clean_cache(path_list)?,
        _ => panic!("Unknown command {}", cmd),
    }
    Ok(())
}

fn run(path_list: Vec<String>) -> Result<()> {
    let hasher = HasherConfig::new()
        .preproc_dct()
        .hash_alg(HashAlg::Gradient)
        .to_hasher();
    let mut file_list = cache::load_cache().unwrap_or(HashMap::new());
    for elem in path_list {
        cache::cache_elem(&elem, &hasher, &mut file_list)?;
    }
    find_duplicates(&file_list);
    cache::save_cache(file_list);
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
            "{} -> {}",
            path.to_str().unwrap(),
            new_path.to_str().unwrap()
        );
    } else {
        println!("{}: Could not find extension", path.to_str().unwrap());
    }

    Ok(())
}

fn find_duplicates(file_list: &HashMap<String, String>) {
    let mut lookup = file_list.clone();
    for (path_i, hash_i) in file_list {
        lookup.remove(path_i);
        if let Ok(hash1) = ImageHash::<Box<[u8]>>::from_base64(hash_i) {
            for (path_j, hash_j) in &lookup {
                if let Ok(hash2) = ImageHash::<Box<[u8]>>::from_base64(hash_j) {
                    let diff = hash1.dist(&hash2);
                    println!("{} {} {}", path_i, path_j, diff);
                } else {
                    println!("Could not read hash: {} from file {}", hash_j, path_j)
                }
            }
        } else {
            println!("Could not read hash: {} from file {}", hash_i, path_i)
        }
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
