use std::fs::File;
use std::io::prelude::*;
use openssl::ec::{EcGroup, EcKey};
use openssl::nid::Nid;

fn main() -> std::io::Result<()> {
    //cuvre formula : y^2 mod p = (x^3 + 7) mod p
    /*
    let mut buf = rustyline::Editor::<()>::new();
    let mut readline = buf.readline("path so save private key: ");
    let private = File::create(readline.unwrap())?;
    readline = buf.readline("path so save public key: ");
    let public = File::create(readline.unwrap())?;

    */
    let nid = Nid::SECP256K1;
    let group = EcGroup::from_curve_name(nid).unwrap();
    let key = EcKey::generate(&group).unwrap();
    println!("key : {:?}", key.private_key());
    Ok(())
}
