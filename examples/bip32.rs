extern crate digibyte;

use std::{env, process};
use std::str::FromStr;

use digibyte::secp256k1::Secp256k1;
use digibyte::util::key::PrivateKey;
use digibyte::util::bip32::ExtendedPrivKey;
use digibyte::util::bip32::ExtendedPubKey;
use digibyte::util::bip32::DerivationPath;
use digibyte::util::bip32::ChildNumber;
use digibyte::util::address::Address;

fn main() {
    // This example derives root xprv
    // from a 32-byte secret of the input WIF string,
    // derives the child xprv with path m/84h/0h/0h,
    // prints out corresponding xpub,
    // calculates and prints out the first receiving segwit address.
    // Run this example with cargo and WIF argument:
    // cargo run --example bip32 L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("not enough arguments. usage: {} <WIF>", &args[0]);
        process::exit(1);
    }

    let wif = PrivateKey::from_wif(&args[1]).unwrap();
    println!("Seed WIF: {}", wif);

    // use the network from WIF key
    let network = wif.network;
    println!("Network: {:?}", network);
    // seed is a 32-byte secret in WIF
    let seed = wif.to_bytes();

    // we need secp256k1 context for key derivation
    let secp = Secp256k1::new();

    // calculate root key from seed
    let root = ExtendedPrivKey::new_master(network, &seed).unwrap();
    println!("Root key: {}", root);

    // derive child xpub
    let path = DerivationPath::from_str("m/84h/0h/0h").unwrap();
    let child = root.derive_priv(&secp, &path).unwrap();
    println!("Child at {}: {}", path, child);
    let xpub = ExtendedPubKey::from_private(&secp, &child);
    println!("Public key at {}: {}", path, xpub);

    // generate first receiving address at m/0/0
    // manually creating indexes this time
    let zero = ChildNumber::from_normal_idx(0).unwrap();
    let public_key = xpub.derive_pub(&secp, &vec![zero, zero])
                         .unwrap()
                         .public_key;
    let address = Address::p2wpkh(&public_key, network).unwrap();
    println!("First receiving address: {}", address);

}
