use std::collections::HashMap;


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

    fn display_task(self, name:&str){
        if let Some(task) = self.tasks.get(name){
            println!(
                "{}: {:#?}\n",
                name, task
            );
        }else {
            println!("Task '{}' not found!", name);
        }
    }


    
}

fn main(){
    let mut tt1 = TaskTrucker::new();
    tt1.add_task("task", "description", "due_date", Status::Pending);
    tt1.add_task("task1", "description", "due_date", Status::Pending);
    tt1.add_task("task2", "description", "due_date", Status::Pending);
    tt1.update_task_status("task2", Status::Completed);
    tt1.display_task("task22");
    
    
}