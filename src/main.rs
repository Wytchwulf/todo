use clap::Parser; // Import Parser
use serde::{Deserialize, Serialize}; // Import Serialize & Deserialize 

#[derive(Parser, Debug)] // Automatically create CLI argument parsing from the struct
#[command(name = "Todo")] // Name the app
#[command(about = "Obligatory new language ToDo app", long_about = None)] // Give a description
struct Cli {
    // Custom container for related data
    task: Option<String>, // Define the data type for a task
    #[arg(long)]
    done: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)] // Easily convert to and from JSON
struct Task {
    description: String, // Description of task
    done: bool,          // Complete or not
}

fn main() {
    let args = Cli::parse(); // Parses command line input

    if let Some(index) = args.done {
        // If a number is provided with arg --done bind it to index and run the following block
        let mut tasks: Vec<Task> = match std::fs::read_to_string("todo.json") {
            // read todo.json and convert to string
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]), // content holds raw json string, serde_json will parse to list else fallback to empty list
            Err(_) => {
                println!("No tasks found");
                return; // If file doesnt exist print to command line and exit 
            }
        };

        if index < tasks.len() {
            // if the index number is less than the total number of tasks
            tasks[index].done = true; // mark the associated task as done

            let json = serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks"); // Convert back to JSON else crash and print

            std::fs::write("todo.json", json).expect("Failed to write to file"); // overwrites todo.json saves to disk else crash and print

            println!("Marked task {} as done", index);
        } else {
            println!("No task at index {}", index);
        }

        return;
    }

    match args.task {
        // Pattern matching
        Some(task_desc) => {
            // If not None the following block is run
            let task = Task {
                description: task_desc, // Assigns the argument to the description
                done: false,
            };

            let mut tasks: Vec<Task> = match std::fs::read_to_string("todo.json") {
                // Try to read todo.json and parse it into a list of tasks
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]), // If parsing fails fallback to empty list
                Err(_) => vec![], // If file doesnt exsist fallback to empty list
            };

            tasks.push(task); // Add task to list

            let json = serde_json::to_string_pretty(&tasks) // Turn the entire task list into JSON
                .expect("Failed to serialize task");

            std::fs::write("todo.json", json) // Write to todo.json
                .expect("Failed to write to file");

            println!("Task saved to todo.json");
        }
        None => {
            let tasks: Vec<Task> = match std::fs::read_to_string("todo.json") {
                // Try to read todo.json and parse it to a list of tasks
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]), // If parsing fails fallback to empty list
                Err(_) => {
                    println!("No tasks found"); // If file doesnt exist print to command line
                    return;
                }
            };

            if tasks.is_empty() {
                println!("No tasks to show");
            } else {
                for (i, task) in tasks.iter().enumerate() {
                    // For each task in list...
                    let status = if task.done { "[x]" } else { "[ ]" }; // If the task is complete mark with an X or else a blank box
                    println!("{} {} {}", i, status, task.description); // Print to command line task index, completion status and description
                }
            }
        }
    }
}
