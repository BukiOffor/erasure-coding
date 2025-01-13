extern crate reed_solomon_erasure;

extern crate bip39;
extern crate hex;

use bip39::{Language, Mnemonic, MnemonicType, Seed};
use reed_solomon_erasure::galois_8::ReedSolomon;


fn main () {

    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);    
    let seed = Seed::new(&mnemonic, "");
    let seed = seed.as_bytes();
    println!("Hex Seed: {:?}\n", hex::encode(seed));
    
    
   
    let r = ReedSolomon::new(2, 3).unwrap(); // 3 data shards, 2 parity shards

 
    println!("seed: {:?}\n", seed);

    let mut data = vec![
        seed[0..32].to_vec(),
        seed[32..].to_vec(),
        vec![0;32],
        vec![0; 32],
        vec![0; 32],
       
    ];

    // Construct the parity shards
    r.encode(&mut data).unwrap();

    //println!("encoded data: {:?}", data);

    // Make a copy and transform it into option shards arrangement
    // for feeding into reconstruct_shards
    let mut shards: Vec<_> = data.iter().cloned().map(Some).collect();

    println!("shards: {:?} \n", shards);

    // We can remove up to 3 shards, which may be data or parity shards
    shards[0] = None;
    shards[1] = None;
    shards[2] = None;


    println!("shards after some removal: {:?} \n", shards);


    // Try to reconstruct missing shards
    r.reconstruct(&mut shards).unwrap();

    // Convert back to normal shard arrangement
    let result: Vec<_> = shards.into_iter().filter_map(|x| x).collect();

    result.iter().for_each(|x| {
        let result = hex::encode(x);
        println!("result: {:?}", result);
    }
    );

    assert!(r.verify(&result).unwrap());
    assert_eq!(data, result);
}