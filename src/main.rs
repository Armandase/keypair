use std::fs::File;
use std::io::prelude::*;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, Message};
use secp256k1::hashes::{sha256, Hash};

fn main() -> std::io::Result<()> {
    //cuvre formula : y^2 mod p = (x^3 + 7) mod p
    /*
    let mut buf = rustyline::Editor::<()>::new();
    let mut readline = buf.readline("path so save private key: ");
    let private = File::create(readline.unwrap())?;
    readline = buf.readline("path so save public key: ");
    let public = File::create(readline.unwrap())?;

    */
    let secp = Secp256k1::new();
    let (private, public) = secp.generate_keypair(&mut OsRng);
    let msg = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());
    let sign = secp.sign_ecdsa(&msg, &private);
    assert!(secp.verify_ecdsa(&msg, &sign, &public).is_ok());
    println!("private key : {:#0?}", private);
    //println!("public key : {:?}", public);
    //println!("sign : {:?}", sign);
    Ok(())
}
