// import created classes
mod task;
use task::Task;
mod task_list;
use task_list::TaskList;

// import std classes
use std::path::Path;
use std::io;

fn print_menu() {
    println!("Menu:--------------------------------------------------");
    println!("\t-(1) : Add a task");
    println!("\t-(2) : Remove a task");
    println!("\t-(3) : Change the status of a task");
    println!("\t-(q) : Save and quit the program");
    println!("-------------------------------------------------------");
}

fn add_task_wrapper(list: &mut TaskList) {
    // get task name
    let mut input: String = String::new();
    println!("Task name: ");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let input: &str = input.trim();
    println!();

    // then create and add the task to the list
    let new_task = Task::new(input.to_string(), false);
    list.add_task(new_task);
}

fn remove_task_wrapper(list: &mut TaskList) {
    // get task id
    let mut input: String = String::new();
    println!("Which task do you want to remove? ");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let input: &str = input.trim();
    println!();

    // ... and remove it
    list.remove_task(input.parse().unwrap());
}

fn update_task_wrapper(list: &mut TaskList) {
    // get task id
    let mut input: String = String::new();
    println!("Which task do you want to update? ");
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    let input: &str = input.trim();
    println!();

    // ... and update it
    list.update_task(input.parse().unwrap());
}

fn main() {
    // initialize values from saved file
    let mut list: TaskList = TaskList::new(Path::new("./tasks.txt"));
    let mut done : bool = false;

    // print menu, poll for user input, and process input
    while !done {
        // print tasks and menu
        list.print_tasks();
        println!();
        print_menu();

        // read input
        println!("Enter your choice: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        println!();

        // match input to menu options
        match input.trim() {
            "1" => add_task_wrapper(&mut list),
            "2" => remove_task_wrapper(&mut list),
            "3" => update_task_wrapper(&mut list),
            "q" => { done = true; }
            _   => { println!("Not a valid input. Try again.\n"); continue; }
        }
    }

    // save the list to file
    println!("Saving...");
    list.save();
    println!("Successfully saved!\nThank you for using my program.")

}


