use std::io::stdin;

fn main() {
    menu();
    let mut tasks: Vec<String> = Vec::new();
    loop {
        let mut choice: String = String::new();
        if stdin().read_line(&mut choice).is_ok(){
            println!("{}", choice);

            match choice.trim() {
                "1" => {
                    println!("Add new task");
                    let mut task: String = String::new();
                    if stdin().read_line(&mut task).is_ok() {
                        tasks.push(task.trim().to_string());
                        println!("Successfully added task");
                    }
                }
                "2" => {
                    println!("List of tasks");
                    for (index, task) in tasks.iter().enumerate() {
                        println!("{}: {}", index + 1, task);
                    }
                }
                "3" => {
                    println!("Remove task");
                    let mut id = String::new();
                    if stdin().read_line(&mut id).is_ok() {
                        if let Ok(id) = id.trim().parse::<usize>() {
                            tasks.remove(id);
                            println!("Removed task {}", id);
                        }
                    }
                }
                "4" => {}
                "5" => {}
                _ => {}
            }
        }
    }
}

fn menu() {
    println!("Menu:");
    println!("1. Add Task");
    println!("2. List Task");
    println!("3. Remove Task");
    println!("4. Update Task");
    println!("5. Exit");
}