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

    if let Err(e) = eval_op(&command)  {
        eprintln!("Calculator error: {e}");
        process::exit(1);
    }
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

fn mult(cmd: &Command) -> f32 {
    cmd.a * cmd.b
}

fn div(cmd: &Command) -> Result<f32, &'static str>{
    if cmd.b < 0.0 {
        return Err("Divide by zero");
    }else {
        Ok(cmd.a/cmd.b)
    }
}

fn subtract(cmd: &Command) -> f32 {
    cmd.a - cmd.b
}

fn add(cmd: &Command) -> f32 {
    cmd.a + cmd.b   
}

fn eval_op(cmd: &Command)-> Result<f32, &'static str> {
    
    let answer: f32;
    if cmd.op == "+"{
        answer = add(cmd); 
    }else if cmd.op == "-" {
        answer = subtract(cmd);
    }else if cmd.op == "/" {
        let temp:Result<f32, &'static str>= div(cmd);
        if temp.is_err(){
            return temp;
        }else{
            answer = temp.unwrap();
        }
            
    }else if cmd.op == "*" || cmd.op == "x" {
        answer = mult(cmd); 
    }else{
        return Err("Error finding operator");
    }


    Ok(answer)
}