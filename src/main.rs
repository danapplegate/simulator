use clap::{Parser, ValueEnum};
use miniquad;
use simulator::config::Config;
use simulator::graphics::{self, Stage};
use simulator::output_adapter::{
    csv_adapter::CsvAdapter, stdout_adapter::StdoutAdapter, OutputAdapter,
};
use std::path::PathBuf;
use std::{error::Error, fs};

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputType {
    /// Simulation steps are debug printed to stdout
    Stdout,
    /// Comma-separated values, with each step formatted as a row
    Csv,
    /// Render the simulation graphically in a window
    Graphical,
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
    let args = Args::parse();
    let input_yaml = fs::read_to_string(&args.infile)?;
    let config: Config<3> = serde_yaml::from_str(&input_yaml)?;
    let sim = config.simulation;

    match args.output {
        OutputType::Stdout => {
            let adapter = StdoutAdapter::new(&sim);
            adapter.output();
        }
        OutputType::Csv => {
            let adapter = CsvAdapter::new(&sim);
            adapter.output();
        }
        OutputType::Graphical => {
            let graphics_conf = graphics::new_conf();
            let config_root = args.infile.parent().unwrap().to_path_buf();
            miniquad::start(graphics_conf, move |ctx| {
                Box::new(Stage::new(ctx, sim, config.models, config_root))
            });
        }
    }

    Ok(())
}
