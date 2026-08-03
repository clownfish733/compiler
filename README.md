# Fully custom compiler of custom language to X86_64 assembly
## Current restrictions:
- only works with u32
- has no heap based datastructures

## To run:
'''bash
cargo build --release
build/release/compiler $filepath
'''
