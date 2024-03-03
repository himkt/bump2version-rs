use clap::{Parser,ValueEnum};


#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Action {
    Major,
    Minor,
    Patch,
}


#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum)]
    action: Action,
}


fn main() {
    let opt = Cli::parse();
    println!("{:?}", opt);
}
