use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'N', long)]
    numbers: u32,
    #[arg(short = 'F', long)]
    find: u32,
}

pub fn get_args() -> (u32, u32) {
    let args = Args::parse();
    return (args.numbers, args.find)
}
