use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name.as_deref());
}