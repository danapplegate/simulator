use clap::{Parser, ValueEnum};
use simulator::output_adapter::{
    csv_adapter::CsvAdapter, stdout_adapter::StdoutAdapter, OutputAdapter,
};
use simulator::simulation::Simulation;
use std::path::PathBuf;
use std::{error::Error, fs};

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputType {
    /// Simulation steps are debug printed to stdout
    Stdout,
    /// Comma-separated values, with each step formatted as a row
    Csv,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename containing input simulation data
    #[arg(short, long)]
    infile: PathBuf,

    /// Format of the simulation's output
    #[arg(short, long, value_enum, default_value_t = OutputType::Csv)]
    output: OutputType,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let config = Args::parse();
    let input_yaml = fs::read_to_string(config.infile)?;
    let sim: Simulation<2> = serde_yaml::from_str(&input_yaml)?;

    match config.output {
        OutputType::Stdout => {
            let adapter = StdoutAdapter::new(&sim);
            adapter.output();
        }
        OutputType::Csv => {
            let adapter = CsvAdapter::new(&sim);
            adapter.output();
        }
    }

    Ok(())
}
