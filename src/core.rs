use std::path::Path;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub cmd: CmdSubcommand,
}

#[derive(Debug, Parser)]
pub enum CmdSubcommand {
    #[command(name = "csv", about = "将csv转为其他格式")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file_path)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file_path(path: &str) -> Result<String, String> {
    if !path.ends_with(".csv") {
        return Err(format!("{} is not a csv file", path));
    } else if !Path::new(path).exists() {
        return Err(format!("{} does not exist", path));
    }
    Ok(path.to_string())
}
