use clap::Clap;
use pika::formatter::{format, FormatOption};

#[derive(Clap)]
#[clap(name = "pika", version = "0.1", author = "Eric Groom")]
struct Opts {
    #[clap(subcommand)]
    format: FormatSelection,
}

#[derive(Clap)]
enum FormatSelection {
    #[clap(name = "sponge")]
    Sponge(FormatData),
    #[clap(name = "usa")]
    Usa(FormatData),
}

#[derive(Clap)]
struct FormatData {
    #[clap()]
    input: String,
}

fn main() {
    let opts = Opts::parse();
    let formatted_text = match opts.format {
        FormatSelection::Sponge(data) => format(&data.input, FormatOption::Sponge),
        FormatSelection::Usa(data) => format(&data.input, FormatOption::Usa),
    };
    println!("{}", formatted_text);
}
