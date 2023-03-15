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

const FILE_PATH: &str = "tasks.json";

fn help() {
    println!("\nhelp\n");

    println!("--add \t\t\t add one task to be completed");
    println!("--edit [task]\t\t edit the task specified");
    println!("--delete [task]\t\t delete the task specified");
    println!("--view \t\t\t view all the task you have");
}

fn add() {
    let mut buf_tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err),
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
        
    if let Err(err) = upload_data(buf_tasks) {
        panic!("Couldn't upload the data: {}", err);
    };

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
    let tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err)
    };

    println!("{:?}", tasks);
}

fn delete() {
    let mut tasks = match get_data() {
        Ok(f) => f,
        Err(err) => panic!("Couldn't get the data in the file: {}", err)
    };

    
    print!("Type the number of the task you want to delete: ");
    io::stdout().flush().expect("flushed failed");

    let mut user_input: String = String::new();
    if let Err(err) = io::stdin().read_line(&mut user_input) {
        println!("Not possible to add the task: {}", err);
        return;
    }

    let user_input = match user_input.trim_end().to_string().parse::<i32>() {
        Ok(v) => v,
        Err(err) => panic!("You should only input numbers: {}", err),
    };

    let elem_rem: Task;

    if user_input < 0 && user_input >= tasks.tasks.len() as i32 {
        panic!("Index out of bound");
    } else {
        elem_rem = tasks.tasks.remove((user_input - 1) as usize);
    }

    println!("Removed: {:?}", elem_rem);
}

fn upload_data(tasks: Tasks) -> io::Result<()> {

    let mut file = File::create(FILE_PATH)?;
    let task = serde_json::to_string(&tasks)?;
    file.write_all(task.as_bytes())?;

    return Ok(());
}

fn get_data() -> io::Result<Tasks> {
    let file = File::open(FILE_PATH)?;
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
           "delete" => delete(), 
           "edit" => println!("edit function"), 
           "view" => read_all(), 
           s => {
               println!("TODO {}: unknown command.\nDo --help to see all the available commands.", s);
           },
        }
    }
}
