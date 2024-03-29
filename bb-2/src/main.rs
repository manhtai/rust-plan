use bson;
use ron;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{Read, Result, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    square: i32,
}

const FILE_NAME: &str = "target/a.txt";

fn json_to_file() -> Result<()> {
    println!("JSON to File");

    let a = Move { square: 1 };
    println!("a: {:?}", a);

    let a_str = serde_json::to_string(&a)?;

    let mut file = File::create(FILE_NAME)?;
    file.write_all(a_str.as_bytes())?;

    let mut file = File::open(FILE_NAME)?;
    let mut b_str = String::new();
    file.read_to_string(&mut b_str)?;
    let b: Move = serde_json::from_str(&b_str)?;
    println!("b: {:?}", b);

    Ok(())
}

fn ron_to_vec() -> Result<()> {
    println!("RON to Vec");

    let a = Move { square: 1 };
    println!("a: {:?}", a);

    let a_str = ron::ser::to_string(&a).unwrap();
    let mut v: Vec<u8> = Vec::new();
    v.write_all(a_str.as_bytes())?;

    let b: Move = ron::de::from_bytes(v.as_mut()).unwrap();

    println!("v: {:?}", v);
    println!("b: {:?}", b);

    Ok(())
}

fn bson_to_file() -> Result<()> {
    println!("BSON to File");

    let mut arr: Vec<Move> = Vec::with_capacity(10);
    for x in 0..10 {
        arr.push(Move { square: x });
    }

    let mut file = File::create(FILE_NAME)?;
    let mut doc = bson::Document::new();
    for a in arr {
        let a_bson = bson::to_bson(&a).unwrap();
        doc.insert(a.square.to_string(), a_bson);
    }
    println!("doc: {:?}", doc);

    bson::encode_document(&mut file, &doc).unwrap();
    let mut file = File::open(FILE_NAME)?;
    let doc = bson::decode_document(&mut file).unwrap();
    println!("doc: {:?}", doc);

    Ok(())
}

fn main() -> Result<()> {
    json_to_file()?;
    ron_to_vec()?;
    bson_to_file()?;

    Ok(())
}
