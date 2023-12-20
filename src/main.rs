use clap::Parser;
use std::io;

// fn main() -> Result<(), io::Error> {
//     clap::clap_parse_demo();
//     tui::tui_demo()?;
//     Ok(())
// }

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    Ok(())
}
