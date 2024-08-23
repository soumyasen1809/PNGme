use crate::{
    args::{Commands, DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs},
    png::Png,
};

pub fn execute_command(command: Commands) -> std::result::Result<(), clap::error::Error> {
    match command {
        Commands::Encode(encode_args) => execute_encode(encode_args),
        Commands::Decode(decode_args) => execute_decode(decode_args),
        Commands::Remove(remove_args) => execute_remove(remove_args),
        Commands::Print(print_args) => execute_print(print_args),
    }
}

fn execute_encode(encode_args: EncodeArgs) -> Result<(), clap::Error> {
    Ok(())
}

fn execute_decode(decode_args: DecodeArgs) -> Result<(), clap::Error> {
    Ok(())
}

fn execute_remove(remove_args: RemoveArgs) -> Result<(), clap::Error> {
    Ok(())
}

fn execute_print(print_args: PrintArgs) -> Result<(), clap::Error> {
    let file = std::fs::read(print_args.in_file_path)?;
    let png_image = Png::try_from(&file[..]).unwrap();
    for chunk in png_image.chunks() {
        println!("{:?}", chunk.data());
    }
    Ok(())
}
