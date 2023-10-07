use std::env;
use text_colorizer::*;
use std::fs;
use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]
struct Argumets {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{}",
        "Usage: text-colorizer [OPTIONS] PATTERN REPLACE INPUT_FILE OUTPUT_FILE"
            .red()
            .bold()
    );
}

fn read_and_write_file(arg :&Argumets){
    println!("In file {}", arg.input_file);
    let content:String = fs::read_to_string(&arg.input_file)
        .expect("Something went wrong reading the file");

    let replaced_data = match find_and_replace(&arg.pattern, &arg.replace, &content) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{} {}", "Error: ".red(), e);
            std::process::exit(1);
        }
    };

    fs::write(&arg.output_file, replaced_data)
        .expect("Something went wrong writing the file");
}

fn find_and_replace(target:&str,rep:&str,data:&str)->Result<String,regex::Error>{
    let reg = Regex::new(target)?;
    let result = reg.replace_all(data, rep);
    Ok(result.to_string())
}

fn parse_args()->Argumets{
    let arg: Vec<String> = env::args().skip(1).collect();
    if arg.len() < 4 {
        print_help();
        eprintln!("{} expected 4 but received {}", "Error: Invalid number of arguments".red(), arg.len().to_string().red());
        std::process::exit(1);
    }
    Argumets{
        pattern: arg[0].clone(),
        replace: arg[1].clone(),
        input_file: arg[2].clone(),
        output_file: arg[3].clone(),
    }
}


pub fn run() {
    let args: Argumets = parse_args();
    read_and_write_file(&args);
    println!("{:?}",args);
}
