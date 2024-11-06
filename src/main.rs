#![allow(unused_variables)]
use std::path::PathBuf;

use pngme::commands::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme")]
pub enum PngMeArgs {
    //A demo app for my *very basic* png library that "hides" messages in the chunks
    //of a png file
    Encode(EncodeArgs),
    Decode(DecodeArgs), //TODO: add encode all, add burn after reading
    Remove(RemoveArgs), //TODO: add delete all
    Print(PrintArgs),
}

#[derive(StructOpt, Debug)]
pub struct EncodeArgs {
    path: PathBuf,
    chunk_type: String,
    message: String,
}
#[derive(StructOpt, Debug)]
pub struct DecodeArgs {
    path: PathBuf,
    chunk_type: String,
}
#[derive(StructOpt, Debug)]
pub struct RemoveArgs {
    path: PathBuf,

    chunk_type: String,
}
#[derive(StructOpt, Debug)]
pub struct PrintArgs {
    path: PathBuf,
}

fn main() {
    let args = PngMeArgs::from_args();
    let result = match args {
        PngMeArgs::Encode(args) => encode(args.path.as_path(), args.chunk_type, args.message),
        PngMeArgs::Decode(args) => decode(args.path.as_path(), args.chunk_type),

        PngMeArgs::Remove(args) => remove(args.path.as_path(), args.chunk_type),
        PngMeArgs::Print(args) => print(args.path.as_path()),
    };
    println!("{}", result.unwrap())
}
