use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the input program
    #[arg(value_name = "PROGRAM")]
    program: String,
}

fn main() {
    let parsed_args: Args = Args::parse();
    println!("Hello, world!");
}
