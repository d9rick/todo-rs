use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

use crate::task::Task;

// stores the task stuff
pub struct TaskList {
    filename: PathBuf,
    tasks: HashMap<i32, Task>,
    next_available_id: i32
}

impl TaskList {

    // takes a filename, and returns a hashmap of tasks mapped to their unique ids
    pub fn new(path: &Path) -> TaskList {
        // initialize basic member vars
        let mut tasks: HashMap<i32, Task> = HashMap::new();
        let filename: PathBuf = path.to_path_buf();

        // open the file and create the buffer reader...
        let file: File = File::open(&path).unwrap();
        let reader: io::BufReader<File> = io::BufReader::new(file);
        
        let mut next_available_id = 0;

        // and read in the program state line-by-line:
        for line in reader.lines() {

            // handle error with line reading
            let line = match line {
                Ok(line) => line,
                Err(e) => panic!("Failed to read line! {}", e),
            };
            
            // tasks are stored line by line like this:
            // - $TASK_ID, $TASK_NAME, $TASK_STATUS \n
            
            // so grab them in that order & assemble them
            let mut words = line.split(',');
            
            let id: i32 = match words.next().and_then(|id| id.parse().ok()) {
                Some(id) => id,
                None => {
                    println!("Failed to parse id");
                    continue;
                }
            };
        
            let name = match words.next() {
                Some(name) => name,
                None => {
                    println!("No name found");
                    continue;
                }
            };
        
            let status: bool = match words.next().and_then(|status| status.parse().ok()) {
                Some(status) => status,
                None => {
                    println!("Failed to parse status");
                    continue;
                }
            };

            let curr_task: Task = Task::new(name.to_string(), status);

            // and then add task to map and go to next one
            tasks.insert(id, curr_task);
            next_available_id = id + 1;
        }

        TaskList {
            filename,
            tasks,
            next_available_id
        }
    }

    // prints the tasks
    pub fn print_tasks(&self) {
        println!("Tasks:-------------------------------------------------");
        // Collect the tasks into a vector
        let mut tasks: Vec<(&i32, &Task)> = self.tasks.iter().collect();

        // Sort the vector by task ID
        tasks.sort_by_key(|&(id, _task)| *id);

        // Iterate over the sorted vector and print the tasks
        for (id, task) in tasks {
            print!("{} | ", id);
            task.print();
        }
        println!("-------------------------------------------------------");
    }
       
    pub fn add_task(&mut self, task: Task) {
        self.tasks.insert(self.next_available_id, task);
        self.next_available_id += 1;
    }

    pub fn update_task(&mut self, id: i32) {
        self.tasks.get_mut(&id).expect("Task not found!").flip_status();
    }

    pub fn remove_task(&mut self, id: i32) {
        self.tasks.remove(&id);
    }

    pub fn save(&self) {
        // open file, clear it, and write the new data
        let mut file = File::create(&self.filename).unwrap();
        for i in &self.tasks {
            let line = format!("{},{},{}\n", i.0, i.1.get_name(), i.1.get_status());
            file.write_all(line.as_bytes()).unwrap();
        }
    }
}
