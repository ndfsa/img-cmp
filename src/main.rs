extern crate data_encoding;
extern crate image;
extern crate img_hash;

use data_encoding::HEXLOWER;
use img_hash::{HashAlg, HasherConfig};
use std::env;

fn main() {
    let mut args = env::args();

    if args.len() > 1 {
        args.next();
    } else {
        panic!("No arguments");
    }

    for item in args {
        let image = image::open(&item).unwrap();
        let hasher = HasherConfig::new()
            .hash_alg(HashAlg::DoubleGradient)
            .to_hasher();
        let hash = hasher.hash_image(&image);
        println!("{}: {:?}", &item, HEXLOWER.encode(hash.as_bytes()));
        println!("{}: {:?}", &item, hash.to_base64());
    }
}
