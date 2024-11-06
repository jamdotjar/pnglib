#![allow(unused_variables)]
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use pngme::{commands::*, Error};
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
   path: PathBuf,

    chunkType: String,
}
#[derive(StructOpt, Debug)]
pub struct PrintArgs {
      path: PathBuf,

}

fn main() {
    let args = PngMeArgs::from_args();
    let result =  match args {
        PngMeArgs::Encode(args) => encode(args.path.as_path(), args.chunkType, args.message),
        PngMeArgs::Decode(args) => decode(args.path.as_path(), args.chunkType),
        
    
        PngMeArgs::Remove(args) => {
            remove(args.path.as_path(), args.chunkType)
        }
        PngMeArgs::Print(args) => {
            print(args.path.as_path())
        }
    };
    println!("{}", result.unwrap())
}
