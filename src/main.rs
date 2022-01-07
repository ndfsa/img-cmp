use img_hash::{HashAlg, Hasher, HasherConfig};
use sha1::Digest;
use std::{
    env, fs,
    io::{self, Result},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        match parse_cmd(&args[1], args[2..].to_vec()) {
            Ok(()) => println!("Process finished"),
            Err(e) => println!("{}", e.to_string()),
        }
    }
}

fn parse_cmd(cmd: &str, path_list: Vec<String>) -> Result<()> {
    match cmd {
        "rename" => {
            for elem in flatten_list(path_list) {
                rename(&elem)?;
            }
        }
        "run" => {
            let hasher = HasherConfig::new()
                .preproc_dct()
                .hash_alg(HashAlg::Mean)
                .to_hasher();
            for elem in flatten_list(path_list) {
                run(&elem, &hasher)?;
            }
        }
        _ => panic!("Unknown command {}", cmd),
    }
    Ok(())
}

fn run(item: &str, hasher: &Hasher) -> Result<()> {
    let image = image::open(&item).unwrap();
    let hash = hasher.hash_image(&image);
    let b64 = hash.to_base64().replace("+", "-").replace("/", "_");
    println!("{}: {}", &item, &b64);
    Ok(())
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
            result.push(path.into_os_string().into_string().unwrap());
        }
    }
    result
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
