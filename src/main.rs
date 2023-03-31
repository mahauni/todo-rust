// build it normal for now;
// without the TUI, and then when is mostly working.
// Add the TUI.

use std::collections::HashMap;
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

fn help_main() {
    println!("TODO is a tool to make To Do lists");
    println!("\n\ncargo run -- [tag]");
    println!("\n\nhelp\n");

    println!("--add \t\t\t add one task to be completed");
    println!("--edit [task]\t\t edit the task specified");
    println!("--delete [task]\t\t delete the task specified");
    println!("--view \t\t\t view all the task you have");
    println!("--done\t\t\t change the status of the task selected")
}

fn add_help() {
}

fn help_edit() {
    println!("cargo run -- --id [id] [OPTIONS]");

    println!("--id [id] \t\t\t choose the task with the selected id starting with 0");
    println!("--task prompt \t\t\t show a prompt in the stdin to type the task to be replaced");
    println!("--done [bool]\t\t\t change the status of the selected taks by the id");
}

fn help_delete() {
    println!("cargo run -- --id [id]");

    println!("--id [id] \t\t\t choose the task with the selected id starting with 0");
}

fn help_done() {
    println!("cargo run -- --id [id]");
}

// make this if there is no --id, it shows all, and if it has is a filter
fn help_view() {
}

fn add() {
    let mut buf_tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err),
    };

    let task: Task;

    // use --task to add another task

    let opt = get_flags();

    match opt.get("--task") {
        Some(v) => {
            if v != "prompt" {
                add_help();
                return;
            }
            println!("Type the task you want to replace with:");
            let t = match get_input() {
                Ok(v) => v,
                Err(err) => panic!("Couldn't get user input from stdin: {}", err)
            };

            task = Task { task: t, done: false, date_created: get_date(), date_finished: None };
        },
        None => {
            add_help();
            return;
        }
    };

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

    let opt = get_flags(); 

    let user_input = match opt.get("--id") {
        Some(v) => v,
        None => {
            help_delete();
            return;
        },
    };

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

    let opt = get_flags();
    
    // --help first
    if let Some(_) = opt.get("--help") {
        help_edit();
        return;
    }

    let index = match opt.get("--id") {
        Some(v) => v.parse::<usize>().expect("Expected a number"),
        None => {
            println!("Command with few flags");
            help_edit();
            // panic
            return;
        }
    };


    // task it need to be without, because space makes the args to be another one and that is not
    // possible, so we need to have the user to input in the terminal
    let task = match tasks.tasks.get_mut(index) {
        Some(t) => t,
        None => panic!("Task does not exist")
    };

    // this one are optional, so we need to make the id obrigatory, but the task and done optional
    // so see if map has more than 1 and if we find --id, if we get both we continue, if not, we
    // panic

    if let Some(v) = opt.get("--task") {
        if v != "prompt" {
            help_edit();
            return;
        }
        println!("Type the task you want to replace with:");
        let t = match get_input() {
            Ok(v) => v,
            Err(err) => panic!("Couldn't get user input from stdin: {}", err)
        };

        task.task = t;
    };

    if let Some(status) = opt.get("--done") {
        match status.parse::<bool>().expect("Expected to be a boolean (true or false)") {
            true => {
                task.done = true;
                task.date_finished = Some(get_date());
            },
            false => {
                task.done = false;
                task.date_finished = None;
            },
        }
    };

    if let Err(err) = upload_data(tasks) {
        panic!("Couldn't upload the data to the file: {}", err)
    };
}

fn done() {
    let mut tasks = match get_data() {
        Ok(v) => v,
        Err(err) => panic!("Couldn't get the data in the file: {}", err)
    };

    let opt = get_flags();

    let index = match opt.get("--id") {
        Some(i) => i.parse::<usize>().expect("Expected to be usize number"),
        None => {
            help_done();
            return;
        },
    };

    let task = match tasks.tasks.get_mut(index) {
        Some(t) => t,
        None => panic!("Dont have this task")
    };


    // here we are fliping the done task, so if is false, will be true and vice-versa
    match task.done {
        false => {
            task.done = true;
            task.date_finished = Some(get_date());
        },
        true => {
            task.done = false;
            task.date_finished = None;
        },
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

fn get_flags() -> HashMap<String, String> {
    let mut flags: HashMap<String, String> = HashMap::new();
     
    for (tag, value) in args().skip(2).step_by(2).zip(args().skip(3).step_by(2)) {
        flags.insert(tag, value);
    }
    
    return flags;
}

fn get_date() -> String {


    "2023-03-31".to_owned()
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
                "--help" => help_main(), 
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
        None => help_main()
    }
}
