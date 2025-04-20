use clap::Parser; // Import Parser
use serde::{Deserialize, Serialize}; // Import Serialize & Deserialize 

#[derive(Parser, Debug)] // Automatically create CLI argument parsing from the struct
#[command(name = "Todo")] // Name the app
#[command(about = "Obligatory new language ToDo app", long_about = None)] // Give a description
struct Cli {
    // Custom container for related data
    task: Option<String>, // Define the data type for a task
    #[arg(long)] // use -- for arg
    done: Option<usize>, // e.g. --done 1
    #[arg(long)]
    delete: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)] // Easily convert to and from JSON
struct Task {
    description: String, // Description of task
    done: bool,          // Complete or not
}

enum TaskAction {
    Done,
    Delete,
}

fn update_task(index: Option<usize>, action: TaskAction) {
    if let Some(index) = index {
        let mut tasks: Vec<Task> = match std::fs::read_to_string("todo.json") {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
            Err(_) => {
                println!("No tasks found");
                return;
            }
        };

        if index < tasks.len() {
            match action {
                TaskAction::Done => {
                    tasks[index].done = true;

                    println!("Marked task {} as done", index);
                }
                TaskAction::Delete => {
                    let removed = tasks.remove(index);

                    println!("Deleted task {}: {}", index, removed.description);
                }
            }

            let json = serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");

            std::fs::write("todo.json", json).expect("Failed to write to file");
        } else {
            println!("No task at index {}", index);
        }
    }
}

fn main() {
    let args = Cli::parse();

    if args.done.is_some() {
        update_task(args.done, TaskAction::Done);
        return;
    }

    if args.delete.is_some() {
        update_task(args.delete, TaskAction::Delete);
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
