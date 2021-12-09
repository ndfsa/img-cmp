// use img_hash::{HashAlg, HasherConfig};
use sha1::{Digest, Sha1};
use std::{
    env, fs,
    io::{self, Result},
};

fn main() {
    let mut args = env::args();

    if args.len() > 1 {
        args.next();
    } else {
        panic!("No arguments");
    }

    match &args.next().expect("No command was provided")[..] {
        "rename" => {
            rename(args.next().expect("No file or folder provided"))
                .expect("Error processing files");
        }
        "run" => {
            run(args.next().expect("No file or folder provided"));
        }
        x => panic!("Unknown command: {}", x),
    }
}
fn run(item: String) {
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
}

fn rename(item: String) -> Result<()> {
    if fs::metadata(&item).unwrap().is_dir() {
        for entry in fs::read_dir(&item)? {
            let mut file_entry = entry?.path();

            if file_entry.is_dir() {
                return Result::Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Path points to directory",
                ));
            }

            let ext = file_entry
                .extension()
                .expect("Cannot find extension")
                .to_owned();

            let old_path = file_entry
                .to_str()
                .expect("Could not unpack path")
                .to_owned();

            let mut file = fs::File::open(&old_path)?;
            let mut hasher = sha1::Sha1::new();

            io::copy(&mut file, &mut hasher)?;

            file_entry.set_file_name(format!("{:x}", hasher.finalize()));
            file_entry.set_extension(ext);
            let new_path = file_entry
                .to_str()
                .expect("Could not unpack path")
                .to_owned();

            fs::rename(&old_path, &new_path)?;
            println!("\nold path: {}\nnew path: {}", &old_path, &new_path);
        }
        Ok(())
    } else {
        let file = fs::File::open(item)?;
        println!("success");
        Ok(())
    }
}
