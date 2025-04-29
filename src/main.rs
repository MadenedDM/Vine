use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// Initialize a git repo and create the core enviroment
    Init,
    Build,
    Run,
    /// Run avaliable tests in the current enviorment
    Test,
    /// Install a package to the current enviroment
    Install, 
    /// Update a portion of your toolchain
    Update,
}

fn main() {
    let args = Args::parse();
  
    println!("{:?}", args);
}
