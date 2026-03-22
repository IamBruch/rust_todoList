use std::fs;
use std::io::Write;
use std::io;

enum Status{
    Done,
    Todo,
}


struct Task {
    id: u32,
    name: String,
    status: Status,
}

fn load_todo_list(file_name: String) -> Vec<Task> {
    let content = fs::read_to_string(file_name).unwrap();

    let mut tasks = Vec::new();
    for line in content.lines(){
        tasks.push(parse_line(line));
    }

    tasks
}

fn parse_line(line: &str) -> Task {
    let parsed_line: Vec<&str> = line.split(',').collect();

    let id = parsed_line[0].parse::<u32>().unwrap();
    let name = parsed_line[1].to_string();
    let status = match parsed_line[2] {
        "todo" => Status::Todo,
        "done" => Status::Done,
        _ => Status::Todo,
    };


    Task {id, name, status}
}

fn show_todo_list(tasks: &Vec<Task>){
    for task in tasks{
        let status_str = match task.status{
            Status::Todo => "todo",
            Status::Done => "done",
        };

        println!("{} {} {}", task.id, task.name, status_str);
    }

    println!("--------------------------------------------------------------");
}

fn delete_task(tasks: &mut Vec<Task>) -> bool{
    println!("Enter task ID to delete:");

    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();

    let id: u32 = id.trim().parse().unwrap();

    match tasks.iter().position(|t| t.id == id){
        Some(index) => {
            tasks.remove(index);
            println!("Task {} zmazany", id.to_string());
        },
        None => println!("Task s id {} neexistuje", id.to_string()),
    }
    println!("--------------------------------------------------------------");
    true
}

fn update_status(tasks: &mut Vec<Task>) -> bool {
    println!("Enter task ID u want to Change: ");

    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();    
    let id: u32 = id.trim().parse().unwrap();

    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            let status_str = match task.status{
                Status::Todo => "todo",
                Status::Done => "done",
            };
            println!("Task has status {} ", status_str);
            println!("What do u want to change it into? (todo/done)");

            let mut new_status = String::new();
            io::stdin().read_line(&mut new_status).unwrap();

            task.status = match new_status.trim() {
                "todo" => Status::Todo,
                "done" => Status::Done,
                _ => {
                    println!("Invalid status");
                    return true;
                }
            };
        },
        None => println!    ("Task with id {} doesnt exist", id),
    }
    println!("--------------------------------------------------------------");
    true
}

fn update_name(tasks: &mut Vec<Task>) -> bool {
    println!("Enter task ID u want to Change: ");

    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();    
    let id: u32 = id.trim().parse().unwrap();

    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            println!("Task has name {} ", task.name);
            println!("Type new name for this task: ");

            let mut new_name = String::new();
            io::stdin().read_line(&mut new_name).unwrap();

            println!("The name was changed from '{}' into '{}' ", task.name, new_name.trim());
            task.name = new_name.trim().to_string();
        },
        None => println!    ("Task with id {} doesnt exist", id),
    }
    println!("--------------------------------------------------------------");
    true
}

fn add_new_task(tasks: &mut Vec<Task>) -> bool {
    println!("Enter task name:");
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let status = Status::Todo;

    let new_task = Task {id, name: name.trim().to_string(), status};
    tasks.push(new_task);

    println!("task {} added in todo_list", name.trim());
    println!("--------------------------------------------------------------");
    true
}

fn get_input(mut tasks: &mut Vec<Task>) -> bool{
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    match input.trim() {
        "list" => {
            show_todo_list(tasks);
            true
        }
        "add" => {
            add_new_task(&mut tasks);
            true
        }
        "delete" => {
            delete_task(&mut tasks);
            true
        }
        "change status" => {
            update_status(&mut tasks);
            true
        }

        "change name" => {
            update_name(&mut tasks);
            true
        }

        "exit" => {
            println!("you are exiting todo list");
            false
        }
        _ => {
            println!("Invalid input, type help for list of commands");
            true
        }
    }
}

fn update_todo_list(tasks: &Vec<Task>, file_name: String){
    let mut content = String::new();

    for task in tasks{
        let status_str = match task.status{
            Status::Todo => "todo",
            Status::Done => "done",
        };

        content.push_str(&format!("{},{},{}\n", task.id, task.name, status_str));
    }

    fs::write(file_name, content).unwrap();
}

fn main() {

    let mut tasks = Vec::new(); 
    tasks = load_todo_list("list.txt".to_string());

    println!("Todo list is running! yay");

    let file_name = "list.txt";

    loop{
        update_todo_list(&tasks, file_name.to_string());
        if !get_input(&mut tasks){
            break;
        }
    }
}
