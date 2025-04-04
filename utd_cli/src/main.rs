use clap::{Parser, Subcommand};

mod utils;

/// main cli structure
#[derive(Parser, Debug)]
#[command(name = "utd", about = "Ultimate to-do")]
#[command(styles=utils::get_styles())]
struct Cli {
    #[arg(short, long)]
    filters: Vec<String>,
    #[command(subcommand)]
    command: Option<Command>,
}

/// Available Commands
#[derive(Subcommand, Debug)]
enum Command {
    Add {
        #[arg(short = 'P', long)]
        priority: Option<u8>,
        #[arg(short = 'p', long)]
        project: Option<String>,
        misc: Vec<String>,
    },
    List {
        #[arg(short,long)]
        ordering: bool,
        misc: Vec<String>,
    },
    Done {
        misc: Vec<String>,
    },
    Annotate {
        misc: Vec<String>,
    },
    Edit {
        misc: Vec<String>,
    },
    Start {
        misc: Vec<String>,
    },
    Modify {
        misc: Vec<String>,
    },
    Undo {}
}

fn main() {
    let cli = Cli::parse();
    println!("filters {:?}", cli.filters);
    println!("filters {:?}", cli.command);
}
