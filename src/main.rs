// build it normal for now;
// without the TUI, and then when is mostly working.
// Add the TUI.

use std::env::args;
use std::fmt::Display;
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

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.done {
            return writeln!(f, "Created: {}\n[x] Task: {}\nFinished: {}", self.date_created, self.task, self.date_finished.as_ref().unwrap());
        }
        return writeln!(f, "Created: {}\n[ ] Task: {}", self.date_created, self.task);
    }
}

const FILE_PATH: &str = "tasks.json";

fn help() {
    println!("TODO is a tool to make To Do lists");
    println!("\n\ncargo run -- [tag]");
    println!("\n\nhelp\n");

    println!("--add \t\t\t add one task to be completed");
    println!("--edit [task]\t\t edit the task specified");
    println!("--delete [task]\t\t delete the task specified");
    println!("--view \t\t\t view all the task you have");
    println!("--done\t\t\t change the status of the task selected")
}

fn add() {
    let mut buf_tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err),
    };

    print!("Type the task you want to add: ");

    let user_input = match get_input() {
        Ok(v) => v,
        Err(err) => panic!("Could't get input out of stdin: {}", err)
    };

    // Make the date automatic
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

    for i in tasks.tasks {
        println!("{}", i);
    } 
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

fn edit() {
    let mut tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err)
    };

    println!("Type the index of the taks you want to edit: ");
    let index = match get_input() {
        Ok(v) => v.parse::<usize>().expect("Number"),
        Err(err) => panic!("Couldn't get the input: {}", err)
    };

    // possible to do with args().windows(2)
    for (tag, value) in args().skip(2).step_by(2).zip(args().skip(3).step_by(2)) {
        match tag.as_str() {
            "--task" => {
                match tasks.tasks.get_mut(index) {
                   Some(t) => t.task = value,
                   None => panic!("Dont have this task")
                }
            },
            "--done" => {
                let index = 2;
                match tasks.tasks.get_mut(index) {
                    Some(t) => { 
                        t.done = value.parse::<bool>().expect("Expect here to be either false or true");
                        match t.done {
                            true => {
                                t.date_finished = Some("2023-03-29".to_owned())
                            },
                            false => {
                                t.date_finished = None;
                            },
                        }
                    },
                    None => panic!("Dont have this task")
                }
            },
            s => panic!("Unknown command {}", s)
        }
    }
    // done with changin the task
    
    if let Err(err) = upload_data(tasks) {
        panic!("Couldn't upload the data to the file: {}", err)
    };
}

fn done() {
    let mut tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err)
    };

    println!("Type the index of the taks you want to edit: ");
    let index = match get_input() {
        Ok(v) => v.parse::<usize>().expect("Number"),
        Err(err) => panic!("Couldn't get the input: {}", err)
    };

    match tasks.tasks.get_mut(index) {
        Some(t) => { 
            match t.done {
                false => {
                    t.done = true;
                    t.date_finished = Some("2023-03-29".to_owned());
                },
                true => {
                    t.done = false;
                    t.date_finished = None;
                },
            }
        },
        None => panic!("Dont have this task")
    }
}

fn get_input() -> io::Result<String> {
    io::stdout().flush().expect("flushed failed");

    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input)?;

    let user_input = user_input.trim_end().to_string();

    return Ok(user_input);
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
    match args().nth(1) {
        Some(i) => {
            match i.as_str() {
                // refactor this to be a enum to be better
                "--help" => help(), 
                "--add" => add(), 
                "--delete" => delete(), 
                "--edit" => edit(), 
                "--view" => read_all(), 
                "--done" => done(),
                s => {
                    println!("TODO {}: unknown command.\nDo --help to see all the available commands.", s);
                },
            }
        },
        None => help()
    }
}
