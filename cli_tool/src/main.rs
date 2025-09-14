use cli_tool::search;
use std::env;
use std::fs;
use std::io::Error;
use std::process;

struct Arguments {
    search_string: String,
    file_path: String,
}
impl Arguments {
    fn new(args: &Vec<String>) -> Arguments {
        let search_string: String = args[1].clone();
        let file_path: String = args[2].clone();

        return Arguments {
            search_string,
            file_path,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arguments: Arguments = Arguments::new(&args);

    if let Err(e) = read_file(arguments) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn read_file(arguments: Arguments) -> Result<String, std::io::Error> {
    let file_content: String = fs::read_to_string(arguments.file_path)?;
    
    for line in search(&arguments.search_string , &file_content){
        println!("{line}");
    }

    Ok(file_content)
}


//Ok with a string is valid or an Err is valid , we caqn retrun these 2 things 
