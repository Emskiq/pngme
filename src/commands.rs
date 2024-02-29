// Commands file, in which the following commands are defined:
//
// Encode (file_path, chunk_type, message out_file <- optional)
//      - Will just append_chunk(...) to the Png struct
//          pngme encode ./dice.png ruSt "This is a secret message!
//
// Decode (file_path, chunk_type)
//      - Will chunk_by_type(chunk_type) from a png struct
//          pngme decode ./dice.png ruSt
//
// Remove (file_path, chunk_type)
//      - Will remove_chunk(chunk_type) from a png struct
//          pngme remove ./dice.png ruSt
//
// Print (file_path)
//      - Print a list of PNG chunks that can be searched for messages - as_bytes/format?
//          pngme print ./dice.png

use clap::Subcommand;
use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print,
}
