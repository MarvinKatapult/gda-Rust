use clap::Parser;

#[derive(Parser)]
struct Cli {
    // this is a cross plattform string for file paths
    path: std::path::PathBuf,
    pattern: String,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("du lutscher die file existiert nich");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
