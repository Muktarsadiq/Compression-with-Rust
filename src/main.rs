/*
        FILE COMPRESSION IN RUST
*is a command-line utility that compresses a file using gzip compression.
It utilizes the flate2 crate to handle the compression
*/

// IMPORTS //

//Provides functionality for compression and decompression
extern crate flate2;

// A structure used to encode/compress data using the gzip format.
use flate2::write::GzEncoder;

// Represents compression levels (e.g., default, fastest, etc.).
use flate2::Compression;

//Gets command-line arguments passed when running the program.
use std::env::args;

//Used for file input and output operations
use std::fs::File;

//A function to copy data from one I/O stream to another.
use std::io::copy;

//A buffered reader that wraps a reader (in this case, a file) to improve performance when reading data.
use std::io::BufReader;

//Used for timing how long the compression process takes.
use std::time::Instant;

//

fn main() {
    /*
    * @dev: Argument Handling
    * The program expects exactly two command-line arguments: a source file and a target file.
    If the number of arguments is not 3 (the first is the program's name, the second is the source file,
    and the third is the target file), it prints an error message and exits
    */
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    /*
    *@dev: Opening Files
     The program opens the source file using File::open() and wraps it in a BufReader to optimize reading performance.
    */
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    //The target file is created (or overwritten) using File::create().
    let output = File::create(args().nth(2).unwrap()).unwrap();

    /*
     *@dev: Setting Up Compression
     *GzEncoder is created to handle the compression. It wraps the target file and compresses data using the default compression level.
     */
    let mut encoder = GzEncoder::new(output, Compression::default());

    /*
     *@dev: Timing the Compression
     *The program stores the current time in the start variable to calculate how long the compression process takes.
     */
    let start = Instant::now();

    /*
     * @dev: Copying and Compressing Data
     *
     */
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
