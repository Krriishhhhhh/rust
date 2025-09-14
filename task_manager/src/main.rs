use std::fmt::Debug;
use std::io::{Error, Read, Write, stdin, stdout};
use std::process;

//This is the creation of task
#[derive(Debug)]
enum Status {
    Pending,
    Done,
}
#[derive(Debug)]
struct Task {
    id: i64,
    description: String,
    status: Status,
}

impl Task {
    fn new(id: i64, description: String) -> Task {
        Task {
            id,
            description,
            status: Status::Pending,
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    println!("1.Add a Task");
    println!("2.List Tasks");
    println!("3.Mark Task Done");
    println!("4.Exit Program\n\n");

    loop {
        let input: i32 = starter();

    match input{
        1=> add_task(&mut tasks),
        2=>list_tasks(&tasks),
        3=>status_done(&mut tasks),
        4=> process::exit(0),
        _=> panic!("Out of scope input")
    };
    }

    
}

//There is a diff between command line arguments and taking input from the user when the program is runnung , for now lets implement the second one .

fn starter() -> i32 {
    

    print!("Enter the Choice :");
    stdout().flush().unwrap();

    let mut input: String = String::new();
    stdin().read_line(&mut input);
    let input: i32 = input.trim().parse().unwrap();

    return input;
}

fn add_task(tasks: &mut Vec<Task>) {
    print!("Enter the Task ID : ");
    stdout().flush().unwrap();
    let mut id = String::new();
    stdin().read_line(&mut id).unwrap();
    let id: i64 = id.trim().parse().unwrap();

    print!("Enter the Task Description : ");
    stdout().flush().unwrap();
    let mut description = String::new();
    stdin().read_line(&mut description).unwrap();

    println!("\n\n");

    let task = Task::new(id, description);
    tasks.push(task);
}

fn list_tasks(tasks : &Vec<Task>){
    //loop over all tasks and print them 
    for task in tasks{
        println!("{:?}" , *task);
    }

    println!("\n\n");
}


fn status_done(tasks : &mut Vec<Task>){
    
    print!("Enter task Id to mark done : ");
    stdout().flush().unwrap();

    let mut id: String = String::new();
    stdin().read_line(&mut id).unwrap();
    let id : i64 = id.trim().parse().unwrap();

    //now we have to access the task with the given id 
    for task in tasks{
        if task.id == id {
            task.status = Status::Done;
        }
    }

    println!("Task {} mark as done" , id);

}

