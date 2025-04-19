use clap::Parser; // Import Parser

#[derive(Parser, Debug)] // Automatically create CLI argument parsing from the struct
#[command(name = "Todo")] // Name the app
#[command(about = "Obligatory new language ToDo app", long_about = None)] // Give a description
struct Cli { // Custom container for related data
    task: Option<String>, // Define the data type for a task
}

fn main() {
    let args = Cli::parse(); // Parses command line input

    match args.task {
        Some(task) => println!("You said: {}", task), // If user gives a taskj, print it
        None => println!("No task given",) // If not give a default message
    }
}