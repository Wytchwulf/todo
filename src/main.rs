use clap::Parser; // Import Parser
use serde::{Deserialize, Serialize}; // Import Serialize & Deserialize 

#[derive(Parser, Debug)] // Automatically create CLI argument parsing from the struct
#[command(name = "Todo")] // Name the app
#[command(about = "Obligatory new language ToDo app", long_about = None)] // Give a description
struct Cli {
    // Custom container for related data
    task: Option<String>, // Define the data type for a task
}

#[derive(Serialize, Deserialize, Debug)] // Easily convert to and from JSON
struct Task {
    description: String, // Description of task
    done: bool,          // Complete or not
}

fn main() {
    let args = Cli::parse(); // Parses command line input

    match args.task {
        // Pattern matching
        Some(task_desc) => {
            // If not None the following block is run
            let task = Task {
                // Assign Task struct to task variable
                description: task_desc, // Assigns the argument to the description
                done: false,
            };

            let mut tasks: Vec<Task> = match std::fs::read_to_string("todo.json") {
                // Try to read todo.json and parse it into a list of tasks
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]), // If parsing fails fallback to empty list
                Err(_) => vec![], // If file doesnt exsist fallback to empty list
            };

            tasks.push(task); // Add task to list

            let json = serde_json::to_string_pretty(&tasks) // Turn the Task into JSON
                .expect("Failed to serialize task");

            std::fs::write("todo.json", json) // Write to todo.json
                .expect("Failed to write to file");

            println!("Task saved to todo.json");
        }
        None => println!("No task given"),
    }
}
