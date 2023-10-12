use std::{fs::OpenOptions, io::Write, path::Path};

use clap::Parser;
use nox_asm::Assembler;

#[derive(Parser)]
struct Args {
    /// Input file
    #[arg(short)]
    input_file: String,

    /// Output file
    #[arg(short)]
    output_file: String,

    /// Verbose
    #[arg(short)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let input_path = Path::new(&args.input_file);
    let output_path = Path::new(&args.output_file);
    let verbose = args.verbose;

    let mut assembler = Assembler::new(input_path);

    println!("> Assembling {:?}...", input_path);
    let bytes = assembler.assemble(verbose).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(output_path)
        .unwrap();

    file.write_all(&bytes).unwrap();
    println!("> {:?} assembled to {:?}", input_path, output_path);
}
