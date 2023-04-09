use std::fs::File;
use std::io::prelude::*;
use openssl::ec::{EcGroup, EcKey, PointConversionForm};
use openssl::nid::Nid;
use openssl::bn::BigNumContext;

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
    let private = key.private_key().to_hex_str().unwrap().to_lowercase();

    //let mut point_copy = key.clone();
    let mut ctx0 = BigNumContext::new().unwrap();
    let public = key.public_key().to_bytes(&group, PointConversionForm::UNCOMPRESSED, &mut ctx0).unwrap();

    //let mut ctx1 = BigNumContext::new().unwrap();
    //let test = EcPoint::from_bytes(&group, &public, &mut ctx1).unwrap();
    println!("private key : {}", private);
    println!("public key : ");
    for n in public {
        print!("{:x}", n);
    }
    Ok(())
}
