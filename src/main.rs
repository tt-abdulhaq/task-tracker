use std::{collections::HashMap, io::{Write}};


#[derive(Debug)]
enum Status {
    Pending,
    Completed
}

#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    due_date: String,
    status: Status,
}

struct TaskTrucker {
    tasks: HashMap<String, Task>
}

impl Task {
    fn new(name: &str, description:&str, due_date:&str, status:Status) -> Self {
        Task {
            name: name.to_string(),
            description: description.to_string(),
            due_date: due_date.to_string(),
            status
        }
    }
}

impl TaskTrucker {

    fn new() ->Self {
        TaskTrucker{
            tasks: HashMap::new()
        }
    }

    fn add_task(&mut self, name:&str, description:&str, due_date:&str, status:Status){
        let task = Task::new(name, description, due_date, status);
        self.tasks.insert(task.name.clone(), task);
        println!("Task '{}' added successfully!", name);
    }

    fn delete_task(&mut self, name:&str) {
        if let Some(_) = self.tasks.remove(name){
            println!("Task '{}' deleted successfully!", name);
        }
        else {
            println!("Task '{}' not found!", name);
        }
    }

    fn update_task_status(&mut self, name:&str, status:Status) {
        if let Some(task) = self.tasks.get_mut(name){
            task.status = status;
            println!("Task '{}' updated successfully!", name);
        }else {
            println!("Task '{}' not found!", name);
        }
    }

    fn display_tasks(&self) {
        for (name, task) in &self.tasks {
            println!(
                "{}: {:#?}\n",
                name, task
            );
        }
    }
}

fn main(){
    let mut task_tracker = TaskTrucker::new();
    hint();
    loop {
        println!();
        println!();
        println!("##############################################################");
        let choice = get_user_input("Enter your choice:");
        let choice = match choice.parse::<u8>(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                0
            }
        };
        handle_task(&mut task_tracker, choice)

    }   
}
fn hint(){
    println!("Task Tracker Menu:");
    println!("1. Add Task");
    println!("2. Update Task Status");
    println!("3. Delete Task");
    println!("4. Display Tasks");
    println!("5. Exit");
}

fn add_task(task_trucker:&mut TaskTrucker){
    let name = get_user_input("Enter task name: ");
    let description = get_user_input("Enter task description: ");
    let due_date = get_user_input("Enter task to do date: ");
    task_trucker.add_task(&name, &description, &due_date, Status::Pending);
}

fn update_task_status(task_trucker:&mut TaskTrucker){
    let name = get_user_input("Enter task name: ");
    let status = get_user_input("Enter new status (0 for Pending, 1 for Completed): ");
    let new_status = match status.parse::<u32>() {
        Ok(0) => Status::Pending,
        Ok(1) => Status::Completed,
        _ => {
            println!("Invalid status input. Setting status to Pending.");
            Status::Pending
        }
    };
    task_trucker.update_task_status(&name, new_status);
}

fn handle_task(task_trucker:&mut TaskTrucker, choice: u8){
    match  choice { 
        1 => {
            add_task(task_trucker);
        }
        2 => {
            update_task_status(task_trucker);
        }
        3 => {
            let name = get_user_input("Enter task name: ");
            task_trucker.delete_task(&name);
        }
        4 => {
            task_trucker.display_tasks();
        }
        5 => {
            std::process::exit(0);
        }
        _ => {
            println!("Please enter a correct number!");
            hint();
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}