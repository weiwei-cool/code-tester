use clap::{Parser, Subcommand};
mod test;
use crate::test::Test;

#[derive(Parser)]
#[command(name = "code-tester")]
#[command(author = "StelaHaveno")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A tool to check your algorithm.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands{
    /// Test if the algorithm is right.
    Test {
        /// The path of algorithm's executable file.
        file: String,
        #[arg(short, long)]
        /// The path of algorithm's input file.
        input_file: Option<String>,

        #[arg(short, long)]
        /// The correct output file path of the algorithm is also referred to as the answer.
        out_file: Option<String>,

        #[arg(short, long)]
        /// Upper bound of algorithm runtime.
        time_limit: Option<u128>,

        #[arg(long)]
        /// The algorithm has no input.
        no_input: bool,
    }
}



fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Test { file, input_file, out_file, time_limit, no_input } => {
            if *no_input && input_file.is_some() {
                panic!("Cannot use --input-file and --no-input together!")
            }
            let tester = Test::new(file, input_file, out_file, time_limit, no_input);
            tester.run();
        }
    }
}
