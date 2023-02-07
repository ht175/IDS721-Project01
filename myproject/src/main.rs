use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Rachel", about = "Analyze movie dataset")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Rachel")]
    MovieAnalyze {
        #[clap(short, long)]
        file_name: String,
    },
}

fn main() {
    let args = Cli::parse();
    // parse the command line arguments as the parameter of the function
    match args.command {
        Some(Commands::MovieAnalyze { file_name }) => {
            let df = myproject::read_csv(&file_name);
            println!("The whole dataset is below");
            println!("{:?}", df);
            println!("The static information is below");
            let sum = myproject::groupby_sum(&df);
            println!("{:?}", sum);
        }
        None => {
            println!("No command given");
        }
    }
}
