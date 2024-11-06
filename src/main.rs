#![allow(unused_variables)]
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use pngme::{commands::encode, commands::decode, Error};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(StructOpt, Debug)]
pub struct EncodeArgs {
    path: PathBuf,

    chunkType: String,

    message: String,
}
#[derive(StructOpt, Debug)]
pub struct DecodeArgs {
    path: PathBuf,
    chunkType: String,
}
#[derive(StructOpt, Debug)]
pub struct RemoveArgs {
    path: String,

    chunkType: String,
}
#[derive(StructOpt, Debug)]
pub struct PrintArgs {
    path: String,
}

fn main() {
    let args = PngMeArgs::from_args();
    let result =  match args {
        PngMeArgs::Encode(args) => encode(args.path.as_path(), args.chunkType, args.message).unwrap(),
        PngMeArgs::Decode(args) => decode(args.path.as_path(), args.chunkType).unwrap(),
        
    
        PngMeArgs::Remove(args) => {
            println!("{:?}", args);
            "Wowzers".to_string()
        }
        PngMeArgs::Print(args) => {
            println!("{:?}", args);
            "Wowzers".to_string()
        }
    };
    println!("{}", result)
}
