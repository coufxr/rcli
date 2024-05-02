// rcli csv -i input.csv -o output.json -header ,

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct CliOpts {
    #[command(subcommand)]
    cmd: CmdSubcommand,
}

#[derive(Debug, Parser)]
enum CmdSubcommand {
    #[command(name = "csv", about = "将csv转为其他格式")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts: CliOpts = CliOpts::parse();
    match opts.cmd {
        CmdSubcommand::Csv(opts) => {
            println!("{:?}", opts);
        }
    }
}
