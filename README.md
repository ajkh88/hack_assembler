# Hack Assembler

This is an implemetation of an assembler that converts Hack Assembly Code which is written in files with an .asm extension, to Hack Machine Language which is denoted by the .hack file extension.

The languages are defined as part of the NAND to Tetris course and in the book "The Elements of Computer Systems" by Noam Nisan and Shimon Schocken 

I wrote the assembler using Rust as I wanted to gain more experience using the language, and I felt as a fairly simple project it was ideal for a learning experience. I therefore tried to use as few external crates as possible, to familiarise myself with the standard library, and except for in the tests which warranted a littel heavy lifting. 

## Instructions

* `cargo build` in the project root directory will build the project in debug more, and then `./target/debug/hack_assember {FILE}` (`.\target\debug\hack_assember.exe {FILE}` on Windows) where `FILE` is a valid Hack assembly file in order to produce the output. The output will be a file with the same name as the input file, but with the .hack extension. It will contain a series of 16 bit instructions. 

* `cargo build --release` will produce an optimised binary, which will run faster but takes longer to compile. The binary will be located at `./target/release/hack_assember` (`.\target\release\hack_assember.exe` on Windows). This can then be used as above.

* `cargo run FILE` where `FILE` is a valid Hack assembly file will compile and run the code in one command. Useful during development!

* `cargo test` will run the integration tests. These tests build the binary, generate .hack files from the .asm files included in the `test/files` directory, and compare the files with the expected output files, denoted by `*_example.hack`. These example files were generated using the Assembler provided with the course software downloads, available at https://www.nand2tetris.org/software