use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }
}

struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> Self {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.insert(self.next_id, task);
        println!("Task added with ID {}", self.next_id);
        self.next_id += 1;
    }

    fn delete_task(&mut self, id: u32) {
        if self.tasks.remove(&id).is_some() {
            println!("Task {} removed", id);
        } else {
            println!("Task with ID {} not found", id);
        }
    }

    fn edit_task(&mut self, id: u32, new_description: String) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.description = new_description;
            println!("Task {} updated", id);
        } else {
            println!("Task with ID {} not found", id);
        }
    }

    fn mark_completed(&mut self, id: u32) {
        if let Some(task) = self.tasks.get_mut(&id) {
            task.completed = true;
            println!("Task {} marked as completed", id);
        } else {
            println!("Task with ID {} not found", id);
        }
    }

    fn list_tasks(&self) {
        for (id, task) in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                id,
                task.description,
                task.completed
            );
        }
    }

    fn save_to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("Unable to create file");
        for (id, task) in &self.tasks {
            writeln!(
                file,
                "{}|{}|{}",
                id, task.description, task.completed
            )
            .expect("Unable to write to file");
        }
        println!("Tasks saved to {}", filename);
    }

    fn load_from_file(&mut self, filename: &str) {
        let file = File::open(filename).expect("Unable to open file");
        let reader = BufReader::new(file);

        self.tasks.clear();
        self.next_id = 1;

        for line in reader.lines() {
            let line = line.expect("Unable to read line");
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 3 {
                let id: u32 = parts[0].parse().expect("Invalid ID");
                let description = parts[1].to_string();
                let completed: bool = parts[2].parse().expect("Invalid completion status");

                self.tasks.insert(
                    id,
                    Task {
                        description,
                        completed,
                    },
                );
                self.next_id = self.next_id.max(id + 1);
            }
        }
        println!("Tasks loaded from {}", filename);
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let filename = "tasks.txt";

    loop {
        println!("\nTODO LIST:");
        println!("1. Add Task");
        println!("2. Delete Task");
        println!("3. Edit Task");
        println!("4. Mark Task as Completed");
        println!("5. List Tasks");
        println!("6. Save Tasks");
        println!("7. Load Tasks");
        println!("8. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                todo_list.add_task(description.trim().to_string());
            }
            2 => {
                println!("Enter task ID to delete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read input");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                todo_list.delete_task(id);
            }
            3 => {
                println!("Enter task ID to edit:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read input");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };

                println!("Enter new description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read input");
                todo_list.edit_task(id, description.trim().to_string());
            }
            4 => {
                println!("Enter task ID to mark as completed:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read input");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID.");
                        continue;
                    }
                };
                todo_list.mark_completed(id);
            }
            5 => {
                todo_list.list_tasks();
            }
            6 => {
                todo_list.save_to_file(filename);
            }
            7 => {
                todo_list.load_from_file(filename);
            }
            8 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }
}