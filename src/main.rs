mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::{fs::{self, OpenOptions}, io::{Read}, path::PathBuf, str::FromStr};
use clap::Parser;

use commands::Commands;
use chunk::Chunk;
use chunk_type::ChunkType;
use png::{Png};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    img_file: PathBuf,

    #[command(subcommand)]
    cmd: Commands,
}


fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut png = get_png (&cli.img_file)?;

    match &cli.cmd {
        Commands::Encode(args) => {
            let chunk_type = ChunkType::from_str(&args.chunk_type)?;
            let msg = &args.msg;

            let chunk = Chunk::new(chunk_type, msg.as_bytes().to_owned());
            png.append_chunk(chunk);

            let _ = fs::write(&cli.img_file, &png.as_bytes())?;

            Ok(())
        }
        Commands::Decode(args) => {
            if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
                let decoded_msg = chunk.data_as_string()?;
                println!("Chunk decoded succesfully! The message is {}", decoded_msg);
            }
            else {
                println!("Chunk type is not found in the image..");
            }

            Ok(())
        }
        Commands::Remove(args) => {
            match png.remove_chunk(&args.chunk_type) {
                Ok(_) => {
                    println!("Chunk {} successfuly removed!", args.chunk_type);
                    let _ = fs::write(&cli.img_file, &png.as_bytes())?;
                    Ok(())
                }
                Err(_) => {
                    println!("Chunk not found in image file!");
                    Ok(())
                }
            }
        }
        Commands::Print => {
            for chunk in png.chunks() {
                println!("Chunk that can be searched: {}", chunk.chunk_type());
            }
            Ok(())
        }
    }
}

fn get_png(img_file: &PathBuf) -> Result<Png> {
    let mut png_file = OpenOptions::new()
        .read(true)
        .open(img_file)?;

    let mut png_bytes : Vec<u8> = Vec::new();
    png_file.read_to_end(&mut png_bytes)?;
    let png = Png::try_from(png_bytes.as_slice())?;
    Ok(png)
}
