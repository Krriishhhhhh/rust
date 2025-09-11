use std::env;
use std::fs;
use std::error::Error;

struct arguments {
    search_string: String,
    file_path: String,
}
impl arguments {
    fn new(args: &Vec<String>) -> arguments {
        
        let search_string: String = args[1].clone();
        let file_path: String = args[2].clone();

        return arguments{search_string , file_path};
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments: arguments = arguments::new(&args);

    if let Err(e) = remaining_logic(arguments) {
        println!("Application error: {e}");
        process::exit(1);
    }



    
}


fn remaining_logic(arguments : arguments)-> Result<() , Box<dyn Error>>  {
    
    let contents = match fs::read_to_string(arguments.file_path) {
        Ok(content)=> content,
        Err(error)=> return Err(Box::new(error))
    };
    //So we basically returning the func if an error occurs in the middle 
let contents = fs::read_to_string(arguments.file_path)?;

    Ok(()); //
   
}
