# PNGme
Implementation of the [PNGme challenge](https://picklenerd.github.io/pngme_book/introduction.html) in Rust


## What are we making?
We're making a command line program that lets you hide secret messages in PNG file.
The program will have four commands:
- Encode a message into a PNG
- Decode a message stored in a PNG
- Remove a message from a PNG
- Print a list of PNG chunks that can be searched for messages

The part of the PNG spec used is [here](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html).


## Installation

1. Clone the repository:

```bash
git clone git@github.com:soumyasen1809/PNGme.git
```

2. Change to the project directory:
```bash
cd png_me
```

3. Build the project using Cargo
```bash
cargo build --release
```

4. You can find the compiled binary in the `target/release` directory


## Usage
- To encode a message into a PNG image and save the result:
```bash
cargo run --release -- encode --in-file-path <input.png> --chunk-type <chunk_type> --message <message> --out-file-path <output.png>
```
- To decode a hidden message from a PNG image and print the message if one is found:
```bash
cargo run --release -- decode --in-file-path <input.png> --chunk-type <chunk_type>
```

- To remove a chunk from a PNG file and save the result:
```bash
cargo run --release -- remove --in-file-path <input.png> --chunk-type <chunk_type>
```

- To print all of the chunks in a PNG file:
```bash
cargo run --release -- print --in-file-path <input.png>
```


## License
This project is licensed under the GNU GENERAL PUBLIC LICENSE. See the [LICENSE](./LICENSE) file for details.
