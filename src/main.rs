// use img_hash::{HashAlg, HasherConfig};
use sha1::Digest;
use std::{
    env, fs,
    io::{self, Result},
    path::PathBuf,
};

fn main() -> Result<()> {
    let mut args = env::args();

    if args.len() > 1 {
        args.next();
    }

    match &args.next().expect("No command was provided")[..] {
        "rename" => rename(args.next().expect("No file or folder provided")),
        "run" => run(args.next().expect("No file or folder provided")),
        _ => Result::Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Command not recognized",
        )),
    }
}
fn run(_item: String) -> Result<()> {
    // for item in args {
    //     let image = image::open(&item).unwrap();
    //     let hasher = HasherConfig::new()
    //         .preproc_dct()
    //         .hash_alg(HashAlg::Mean)
    //         .to_hasher();
    //     let hash = hasher.hash_image(&image);
    //     let b64 = hash.to_base64().replace("+", "-").replace("/", "_");
    //     println!("{}: {}", &item, &b64);
    // }
    Ok(())
}

fn rename(item: String) -> Result<()> {
    let inner_aux = |file_path: &mut PathBuf| -> Result<()> {
        let ext = file_path
            .extension()
            .expect("Cannot find extension")
            .to_owned();

        let old_path = file_path
            .to_str()
            .expect("Could not unpack path")
            .to_owned();

        let mut file = fs::File::open(&old_path)?;
        let mut hasher = sha1::Sha1::new();

        io::copy(&mut file, &mut hasher)?;

        file_path.set_file_name(format!("{:x}", hasher.finalize()));
        file_path.set_extension(ext);
        let new_path = file_path
            .to_str()
            .expect("Could not unpack path")
            .to_owned();

        fs::rename(&old_path, &new_path)?;
        println!("\nold path: {}\nnew path: {}", &old_path, &new_path);
        Ok(())
    };

    if fs::metadata(&item).unwrap().is_dir() {
        for entry in fs::read_dir(&item)? {
            let mut file_path = entry?.path();
            if file_path.is_dir() {
                println!("Element is directory, skipping");
                continue;
            }
            inner_aux(&mut file_path)?;
        }
        Ok(())
    } else {
        let mut file_path = PathBuf::from(item);
        inner_aux(&mut file_path)?;
        Ok(())
    }
}
