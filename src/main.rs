use clap::Parser;
mod quiz_01_04;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author = "O6lvl4", version = "0.0.1", about = "Rust CLI sandbox app", long_about = None)]
struct Args {
    #[clap(short, long)]
    select_quiz: String,
}

fn main() {
    let args = Args::parse();
    let select_quiz = args.select_quiz.to_string();
    match &*select_quiz {
        "01-04" => println!("quiz_01_04 => {}", quiz_01_04::hello_world()),
        _ => println!("no select"),
    }
}
