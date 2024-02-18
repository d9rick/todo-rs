pub struct Task {
    name: String,
    status: bool, // true if done, false if not done
}

impl Task {
    pub fn new(name: String, status: bool) -> Task {
        Task { name, status, }
    }
    
    // prints the full task along with checkbox to screen:
    // [ ] Go to the grocery store.
    // [X] Go to the doctor.
    pub fn print(&self) {
        // assemble the output string...
        let mut output : String = "".to_owned();
    
        match self.get_status() {
            false => output.push_str("[ ] "),
            true =>  output.push_str("[X] ")
        }
    
        output.push_str(&self.get_name());
    
        // now print it!
        println!("{}", output);
    
    }
    
    pub fn get_name(&self) -> &String{
        &self.name
    }

    pub fn get_status(&self) -> bool {
        self.status
    }

    pub fn flip_status(&mut self) {
        self.status = !self.status;
    }

}