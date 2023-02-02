mod util {
    pub mod ecco_error;
}

mod scanning {
    pub mod ecco_scanner;
    pub mod ecco_token;
}

use anyhow::Result;
use clap::Parser;
use scanning::ecco_scanner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the input program
    #[arg(value_name = "PROGRAM")]
    program: String,
}

fn main() -> Result<()> {
    let parsed_args: Args = Args::parse();
    ecco_scanner::Scanner::new(parsed_args.program)?.scan_file()?;
    Ok(())
}
