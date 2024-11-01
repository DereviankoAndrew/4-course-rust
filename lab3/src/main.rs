use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct Task {
    description: String,
    is_done: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            is_done: false,
        }
    }

    fn mark_done(&mut self) {
        self.is_done = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.is_done { "✔" } else { "✘" };
        write!(f, "[{}] {}", status, self.description)
    }
}

struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err("Invalid task index".to_string())
        }
    }

    fn edit_task(&mut self, index: usize, new_description: String) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks[index].description = new_description;
            Ok(())
        } else {
            Err("Invalid task index".to_string())
        }
    }

    fn mark_done(&mut self, index: usize) -> Result<(), String> {
        if index < self.tasks.len() {
            self.tasks[index].mark_done();
            Ok(())
        } else {
            Err("Invalid task index".to_string())
        }
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task);
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(file, "{},{}", task.description, task.is_done)?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let path = Path::new(filename);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        self.tasks.clear();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                let description = parts[0].to_string();
                let is_done = parts[1] == "true";
                self.tasks.push(Task {
                    description,
                    is_done,
                });
            }
        }
        Ok(())
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    let filename = "todo_list.txt";

    // Спроба завантажити збережений список завдань
    match todo_list.load_from_file(filename) {
        Ok(_) => println!("Завантажено список завдань."),
        Err(_) => println!("Файл списку завдань не знайдено, починаємо з порожнього списку."),
    }

    loop {
        println!("\n1. Додати завдання\n2. Видалити завдання\n3. Редагувати завдання\n4. Позначити виконаним\n5. Показати список\n6. Зберегти та вийти");
        print!("Виберіть опцію: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                print!("Введіть опис завдання: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_string();
                todo_list.add_task(Task::new(description));
            }
            "2" => {
                print!("Введіть номер завдання для видалення: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                let index: usize = index_str.trim().parse().unwrap_or(0);
                if let Err(e) = todo_list.remove_task(index - 1) {
                    println!("{}", e);
                }
            }
            "3" => {
                print!("Введіть номер завдання для редагування: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                let index: usize = index_str.trim().parse().unwrap_or(0);
                print!("Введіть новий опис завдання: ");
                io::stdout().flush().unwrap();
                let mut new_description = String::new();
                io::stdin().read_line(&mut new_description).unwrap();
                let new_description = new_description.trim().to_string();
                if let Err(e) = todo_list.edit_task(index - 1, new_description) {
                    println!("{}", e);
                }
            }
            "4" => {
                print!("Введіть номер завдання для позначення як виконаного: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                let index: usize = index_str.trim().parse().unwrap_or(0);
                if let Err(e) = todo_list.mark_done(index - 1) {
                    println!("{}", e);
                }
            }
            "5" => {
                todo_list.list_tasks();
            }
            "6" => {
                if let Err(e) = todo_list.save_to_file(filename) {
                    println!("Помилка збереження: {}", e);
                } else {
                    println!("Список збережено.");
                }
                break;
            }
            _ => println!("Невірний вибір!"),
        }
    }
}
