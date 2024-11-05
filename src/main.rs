#![allow(unused_variables)]
use std::path::PathBuf;

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
    input: String,

    chunkType: String,

    message: String,
}
#[derive(StructOpt, Debug)]
pub struct DecodeArgs {
    input: String,
    chunkType: String,
}
#[derive(StructOpt, Debug)]
pub struct RemoveArgs {
    input: String,

    chunkType: String,
}
#[derive(StructOpt, Debug)]
pub struct PrintArgs {
    input: String,
}

fn main() {
    let args = PngMeArgs::from_args();
    match args {
        PngMeArgs::Encode(args) => {
            println!("{:?}", args)
        }
        PngMeArgs::Decode(args) => {
            println!("{:?}", args)
        }
        PngMeArgs::Remove(args) => {
            println!("{:?}", args)
        }
        PngMeArgs::Print(args) => {
            println!("{:?}", args)
        }
    };
}
