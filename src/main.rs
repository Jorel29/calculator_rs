use std::{env, process};

// Current To Do: 
// - Evaluate proper operator function
// - Add, Sub, Div, Mul

fn main() {
    let args:Vec<String> = env::args().collect();
    
    let command = Command::new(&args).unwrap_or_else(|err|{
        eprintln!("Error parsing arguements {err}");
        process::exit(1);
    });

}

struct Command {
    a: f32,
    b: f32,
    op: String,
}

impl Command {
    fn new(args: &[String]) -> Result<Command, &'static str>{
        
        let a:Result<f32, std::num::ParseFloatError> = args[1].parse::<f32>();

        let b:Result<f32, std::num::ParseFloatError> = args[2].parse::<f32>();
        
        if b.is_err() || a.is_err(){
            return Err("Cannot parse for number arguement");
        }

        let op: String = args[3].clone();
        
        Ok(Command {a:a.unwrap(), b:b.unwrap(), op})
   }
}

fn eval_op(cmd: Command)-> Result<f32, &'static str> {
    
    let mut answer: f32;
    if cmd.op == "+"{
        
    }else if cmd.op == "-" {
        
    }else if cmd.op == "/" {
        
    }else if cmd.op == "*" || cmd.op == "x" {
        
    }else{
        return Err("Error finding operator");
    }


    Ok(32.0)
}