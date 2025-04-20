use clap::Parser; // Import Parser
use colored::*;
use serde::{Deserialize, Serialize}; // Import Serialize & Deserialize
use std::path::PathBuf;

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
    #[arg(long)]
    show_done: bool,
    #[arg(long)]
    show_todo: bool,
    #[arg(long)]
    edit: Option<usize>,
    #[arg(long)]
    message: Option<String>,
    #[arg(long)]
    toggle: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)] // Easily convert to and from JSON
struct Task {
    description: String, // Description of task
    done: bool,          // Complete or not
}

enum TaskAction {
    // A data type that can be one of many variants.
    // If a struct is "AND" enum is "OR"
    Done,
    Delete,
    Edit(String),
    Toggle,
}

fn update_task(index: Option<usize>, action: TaskAction) {
    if let Some(index) = index {
        // If there is an index value, bind it to the variable `index`

        let mut tasks: Vec<Task> = match std::fs::read_to_string(&get_todo_file_path()) {
            // Assign to `tasks` the result of the match expression

            // Try to read todo.json as a String (raw JSON from disk)
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
            // If file read succeeds, attempt to parse its contents from JSON
            // If parsing fails, fall back to an empty list
            Err(_) => {
                println!("No tasks found");
                return;
                // If the file doesn't exist or can't be read, print a message and exit
            }
        };

        if index < tasks.len() {
            // If index within range of task list
            match action {
                // Perform the specified action
                TaskAction::Done => {
                    tasks[index].done = true;
                    // Mark the task at the given index as complete

                    println!("Marked task {} as done", index);
                }
                TaskAction::Delete => {
                    let removed = tasks.remove(index);
                    // Remove task at given index

                    println!("Deleted task {}: {}", index, removed.description);
                }
                TaskAction::Edit(new_desc) => {
                    let old_desc = tasks[index].description.clone();
                    tasks[index].description = new_desc;
                    println!(
                        "Edited task {}:\n- Before: {}\n- After: {}",
                        index, old_desc, tasks[index].description
                    );
                }
                TaskAction::Toggle => {
                    tasks[index].done = !tasks[index].done;
                    // Invert status of selected task
                    let status = if tasks[index].done {
                        "Complete"
                    } else {
                        "Incomplete"
                    };
                    println!("Toggled task {} → {}", index, status);
                }
            }

            let json = serde_json::to_string_pretty(&tasks).expect("Failed to serialize tasks");
            // Convert list to pretty json and add to variable json else print error message

            std::fs::write("todo.json", json).expect("Failed to write to file");
            // Write the contents of variable json to todo.json file else print error message
        } else {
            println!("No task at index {}", index);
            // If index out of bounds inform user
        }
    }
}

fn get_todo_file_path() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| {
        eprintln!("Could not find local data directory");
        std::process::exit(1);
    });

    let todo_dir = base.join("todo-rs");

    std::fs::create_dir_all(&todo_dir).expect("Failed to create todo data directory");

    todo_dir.join("todo.json")
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

    if let Some(index) = args.edit {
        // If --edit was provided with an index
        if let Some(new_message) = args.message.clone() {
            // and if --message is also provided
            update_task(Some(index), TaskAction::Edit(new_message));
            return;
            // Update selected task with new message and exit
        } else {
            println!("No message provided for --edit");
            return;
            // else if --message missing advise user & exit
        }
    }

    if let Some(index) = args.toggle {
        //if --toggle was provided with an index
        update_task(Some(index), TaskAction::Toggle);
        return;
        // Update task with result of Toggle and exit
    }

    match args.task {
        // Pattern matching
        Some(task_desc) => {
            // If not None the following block is run
            let task = Task {
                description: task_desc, // Assigns the argument to the description
                done: false,
            };

            let mut tasks: Vec<Task> = match std::fs::read_to_string(&get_todo_file_path()) {
                // Try to read todo.json and parse it into a list of tasks
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]), // If parsing fails fallback to empty list
                Err(_) => vec![], // If file doesnt exist fallback to empty list
            };

            tasks.push(task); // Add task to list

            let json = serde_json::to_string_pretty(&tasks) // Turn the entire task list into JSON
                .expect("Failed to serialize task");

            std::fs::write(get_todo_file_path(), json)
                // Write to todo.json
                .expect("Failed to write to file");

            println!("Task saved to todo.json");
        }
        None => {
            let tasks: Vec<Task> = match std::fs::read_to_string(&get_todo_file_path()) {
                // Try to read todo.json and parse it to a list of tasks
                Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]), // If parsing fails fallback to empty list
                Err(_) => {
                    println!("No tasks found"); // If file doesnt exist print to command line
                    return;
                }
            };

            if tasks.is_empty() {
                println!("No tasks to show");
                return;
            }

            let filtered: Vec<(usize, &Task)> = tasks
                .iter()
                .enumerate()
                .filter(|(_, task)| {
                    if args.show_done {
                        task.done
                    } else if args.show_todo {
                        !task.done
                    } else {
                        true
                    }
                })
                .collect();

            if filtered.is_empty() {
                println!("No matching tasks found");
            } else {
                for (i, task) in filtered {
                    // For each task in list...
                    let status = if task.done {
                        "[x]".green()
                    } else {
                        "[ ]".red()
                    }; // If the task is complete mark with an X or else a blank box
                    println!("{} {} {}", i, status, task.description); // Print to command line task index, completion status and description
                }
            }
        }
    }
}
