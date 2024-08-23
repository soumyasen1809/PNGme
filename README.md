# PNGme
Implementation of the [PNGme challenge](https://picklenerd.github.io/pngme_book/introduction.html) in Rust

## What are we making?
We're making a command line program that lets you hide secret messages in PNG file. Your
program will have four commands:
- Encode a message into a PNG
- Decode a message stored in a PNG
- Remove a message from a PNG
- Print a list of PNG chunks that can be searched for messages

The part of the PNG spec we're tackling is surprisingly simple.