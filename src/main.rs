// build it normal for now;
// without the TUI, and then when is mostly working.
// Add the TUI.

use std::env::args;
use std::io::{self, Write, BufReader, Read};
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    done: bool,
    task: String,
    date_created: String,
    date_finished: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    tasks: Vec<Task>
}

fn help() {
    println!("\nhelp\n");

    println!("--add \t\t\t add one task to be completed");
    println!("--edit [task]\t\t edit the task specified");
    println!("--delete [task]\t\t delete the task specified");
    println!("--view \t\t\t view all the task you have");
}

fn add() {
    let file_path = "tasks.json";

    let mut buf_tasks = match get_data(file_path) {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err),
    };

    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(err) => panic!("Couldn't open write-only file: {}", err),
    };

    print!("Type the task you want to add: ");
    // To make it possible to print before the scanf
    io::stdout().flush().expect("flushed failed");

    let mut user_input: String = String::new();
    if let Err(err) = io::stdin().read_line(&mut user_input) {
        println!("Not possible to add the task: {}", err);
        return;
    }

    let user_input = user_input.trim_end().to_string();

    let date = "1-1-2001".to_owned();

    let task = Task { done: false, task: user_input, date_created: date, date_finished: None }; 

    buf_tasks.tasks.push(task);
        
    let task = match serde_json::to_string(&buf_tasks) {
        Ok(v) => v,
        Err(err) => {
            println!("Couldn't parse to JSON: {}", err);
            return;
        }
    };

    if let Err(err) = file.write_all(task.as_bytes()) {
        panic!("Unable to write in the file: {}", err);
    }

//    if let Err(err) = std::fs::write(file_path, task) {
//        panic!("Unable to write in the file: {}", err);
//    }

//    unsafe {
//        if let Err(err) = file.write(any_as_u8_slice(&task)) {
//            println!("Unable to write in the file: {}", err);
//        }
//    }
}

fn read_all() {
    let file_path = "tasks.json";

    let tasks = match get_data(file_path) {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err)
    };

    println!("{:?}", tasks);
}

fn get_data(file_path: &str) -> io::Result<Tasks> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content: String = String::new();
    buf_reader.read_to_string(&mut content)?;

    let tasks: Tasks = serde_json::from_str(&content)?;
    
    return Ok(tasks);
}

//unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
//    ::core::slice::from_raw_parts(
//        (p as *const T) as *const u8,
//        ::core::mem::size_of::<T>(),
//    )
//}

fn main() {
    for i in args().skip(1) {
        match i.as_str() {
           "help" => help(), 
           "add" => add(), 
           "delete" => println!("delete function"), 
           "edit" => println!("edit function"), 
           "view" => read_all(), 
           s => {
               println!("TODO {}: unknown command.\nDo --help to see all the available commands.", s);
           },
        }
    }
}
