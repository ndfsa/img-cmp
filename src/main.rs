// use img_hash::{HashAlg, HasherConfig};
use sha1::Digest;
use std::{
	env, fs,
	io::{self, Result},
	path::Path,
};

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 2 {
		match handle_paths(args[2..].to_vec(), parse_cmd(&args[1])) {
			Ok(()) => println!("Process finished"),
			Err(e) => println!("{}", e.to_string()),
		}
	}

	panic!("Please consult the \"help\" command");
}

fn parse_cmd(cmd: &str) -> fn(&str) -> Result<()> {
	match cmd {
		"rename" => rename,
		"run" => run,
		_ => panic!("Unknown command {}", cmd),
	}
}

fn run(_item: &str) -> Result<()> {
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
	println!("WIP :(");
	Ok(())
}

fn handle_paths(args: Vec<String>, func: fn(&str) -> Result<()>) -> Result<()> {
	for elem in args {
		let path = Path::new(&elem);
		if !path.exists() {
			println!("\"{}\" not found", path.to_str().unwrap());
			continue;
		}

		let path = path.canonicalize()?;
		if path.is_dir() {
			handle_paths(
				path.read_dir()?
					.map(|sub_elem| {
						String::from(sub_elem.expect("Cannot get file").path().to_string_lossy())
					})
					.collect(),
				func.clone(),
			)?;

		// one day, symlinks will be supported
		// } else if path.is_symlink() {
		//     handle_paths(vec![path.into_os_string().into_string().unwrap()]);
		} else {
			func(path.to_str().unwrap())?;
		}
	}
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
