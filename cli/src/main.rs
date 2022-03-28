use clap::Parser;
use mini_router::storage::write_file;

mod generate_file;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file (json intermediary format)
    #[clap(short, long)]
    input_file: String,

    /// Output file
    #[clap(short, long, default_value = "routing-data.br")]
    output_file: String,
}

fn main() {
    let args = Args::parse();

    let file = generate_file::generate_file(&args.input_file);

    println!("✍️\tWriting output file to {}", args.output_file);
    write_file(args.output_file, &file);
}
