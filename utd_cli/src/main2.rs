use clap::Parser;

// main cli structure
#[derive(Parser, Debug)]
#[command( name= "utd", about = "Ultimate to-do")]
struct Cli{
    misc: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.misc)
}
