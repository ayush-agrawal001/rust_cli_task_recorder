use std::{fs::OpenOptions, io::{self, Write}};
use figlet_rs::FIGfont;
use dialoguer::{theme::ColorfulTheme, Select};
use serde::{Serialize, Deserialize};

fn main() {
    let title = FIGfont::standard().expect("CLI TODO List");
    let title1 = title.convert("CLI TODO List").unwrap();

    println!("Welcome to ->");
    println!("{} \n \n", title1); 

    let option_set_1 = &["1. Add a task file", "2. Delete a task file", "3. List all task files", "4. Exit"];

    let selection = Select::with_theme(&ColorfulTheme::default()).with_prompt("Please Select One of the options below. (Add is default)")
    .default(0).items(option_set_1).interact().expect("err reading selection");

    match selection {
        0 => add_task(),
        1 => delete_task(),
        2 => list_task(),
        3 => exit(),
        _ => unreachable!(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    task: String,
    is_done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct File<'a> {
    file_name: &'a str,
    task: Vec<Task>,
}

fn add_task () {
    let mut file_name_in = String::new();
    let mut task_container = String::new();
    let mut task_list = Vec::new();
      
    // Asking for the task file title //
    println!("Please enter the task file title");

    io::stdin()
        .read_line(&mut file_name_in)
        .expect("Failed to read line");
      
    println!("\nTask file title is {}", &file_name_in.trim());
    
    // Asking for the task to add //
    
    let mut is_task_adding_completed = false;
    let mut x = 0;
    while !is_task_adding_completed {
        x += 1;
        println!("Now add task {}. (write '/exit' or /e to exit)", x);
        let mut  task_input = String::new();
        io::stdin()
            .read_line(&mut task_input)
            .expect("Failed to read line");

        if task_input.trim() == "/exit" || task_input.trim() == "/e" {
            is_task_adding_completed = true;
            print!("exited the task adding");
            continue;
        }

        task_container = task_input.trim().to_string();
        
        let task_to_add = Task {
            task: task_container,
            is_done: false,
        };
      
        task_list.push(task_to_add);
    }


    println!("\nTasks added {:?}", &task_list);

    let file = File {
        file_name: file_name_in.trim(),
        task: task_list,
    };
      
    println!("{:?}", file);
    adding_to_file(file);
}

fn adding_to_file (struct_to_add : File<'_>) {

    let mut json_file_str = serde_json::to_string(&struct_to_add).unwrap();

    let mut file_safe = OpenOptions::new()
        .append(false)
        .read(true)
        .write(true)
        .create(true)
        .open("data.txt")
        .expect("Error while opening the file.");

    file_safe.write_all(&json_file_str.as_bytes()).expect("Error while writing to the file.");

    println!("Task added successfully");
}

fn delete_task () {
    println!("Delete a task");
}

fn list_task () {
    println!("List all task");
}

fn exit () {
    println!("Exit");
}