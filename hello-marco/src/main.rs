use std::process::Command;
use clap::Parser;


#[derive(Parser)]
#[clap(version= "1.0", author="Duany Baro", about= "A Marco Polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Parser)]
enum Commands{
    #[clap(version= "1.0", author="Duany Baro")]
    Play {
        #[clap(show, long)]
        name: String
    }
}

fn main() {
    let args: Cli = Cli::parse();

    match args.command {
        Some(Commands::Play{name}) => {
            if name=="Marco" {
                println!("Polo");
            }
            else {
                println!( "What 's your name?")
            }
        }
        None => println!(N"No command given")
    }
}
