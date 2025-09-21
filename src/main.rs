use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
mod ast;
extern crate prettytable;


struct Cli {
    path: String,
}

fn read_in_source_file(filename: String) -> anyhow::Result<String, anyhow::Error> {
    /*
     Function to read in our source file with .zo extension
     args :
        filename (String): string representation of a .zo filename
    Returns:
        Result (String , io:Error): Either returns String contents or a related error
    */

    let file_extension = Path::new(&filename).extension().and_then(OsStr::to_str);

    if file_extension != Some("zo") {
        anyhow::bail!("Incorrect usage: <filename> should end with .zo extension")
    }

    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let path = std::env::args().nth(1).expect("no path given");
    let args = Cli { path };

    match read_in_source_file(args.path) {
        //if we have a successful result from read_source_file we need to call the tokenizer.
        Ok(contents) => {
           
           ast::ast::parse_input(&contents);
           
        }

        Err(e) => eprintln!("Error Reading file: {}", e),
    };
}
