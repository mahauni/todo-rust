// build it normal for now;
// without the TUI, and then when is mostly working.
// Add the TUI.

use std::env::args;
use std::io::{self, Write, BufReader, Read};
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    done: bool,
    task: String,
    date_created: String,
    date_finished: Option<String>
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

    let task = match serde_json::to_string_pretty(&task) {
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
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => {
            panic!("Unable to open the flie: {}", err);
        }
    };

    let mut buf_reader = BufReader::new(file);
    
    let mut content: Vec<u8> = Vec::new();

    if let Err(err) = buf_reader.read_to_end(&mut content) {
        panic!("Error when buffering the bytes in file: {}", err);
    }

    let s = String::from_utf8_lossy(&content);

    let tasks = serde_json::from_str::<Value>(&s).unwrap();

    println!("{}", tasks);
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
           _ => (),
        }
    }
}
