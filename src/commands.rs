use std::str::FromStr;

use crate::{
    args::{Commands, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
};

pub fn execute_command(command: Commands) -> std::result::Result<(), Box<dyn std::error::Error>> {
    match command {
        Commands::Encode(encode_args) => execute_encode(encode_args),
        Commands::Decode(decode_args) => execute_decode(decode_args),
        Commands::Remove(remove_args) => execute_remove(remove_args),
        Commands::Print(print_args) => execute_print(print_args),
    }
}

/// cargo run --release -- encode --in-file-path assests/dice.png --chunk-type tEXt --message Hello --out-file-path assests/newdice.png
fn execute_encode(encode_args: EncodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read(encode_args.in_file_path)?;
    let mut png_image = Png::try_from(&file[..])?;

    let chunk_type_to_add = ChunkType::from_str(&encode_args.chunk_type)?;
    let chunk_to_append = Chunk::new(chunk_type_to_add, encode_args.message.into());

    png_image.append_chunk(chunk_to_append.clone());

    Ok(std::fs::write(
        encode_args.out_file_path,
        png_image.as_bytes(),
    )?)
}

/// cargo run --release -- decode --in-file-path assests/dice.png --chunk-type tEXt
fn execute_decode(decode_args: DecodeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read(decode_args.in_file_path)?;
    let png_image = Png::try_from(&file[..])?;

    let chunk = png_image.chunk_by_type(&decode_args.chunk_type.as_str());
    println!("Decoded chunk: {:?}", chunk);

    Ok(())
}

///cargo run --release -- remove --in-file-path assests/dice.png --chunk-type tEXt
fn execute_remove(remove_args: RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read(remove_args.in_file_path)?;
    let mut png_image = Png::try_from(&file[..])?;

    let chunk = png_image.remove_first_chunk(&remove_args.chunk_type.as_str())?;
    println!("Chunk removed: {:?}", chunk);

    Ok(())
}

/// cargo run --release -- print --in-file-path assests/dice.png
fn execute_print(print_args: PrintArgs) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read(print_args.in_file_path)?;
    let png_image = Png::try_from(&file[..]).unwrap();
    for chunk in png_image.chunks() {
        println!("{:?}", chunk.data());
    }
    Ok(())
}
