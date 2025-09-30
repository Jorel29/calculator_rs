use std::{ io, process};

// Current To Do: 
// - make some integration tests

fn main() {
    

    println!("Please input your first number");
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap_or_else(|e|{
        eprintln!("Error: {e}, invalid input");
        process::exit(1);
    });

    println!("Please input your second number");
    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap_or_else(|e|{
        eprintln!("Error: {e} invalid input");
        process::exit(1);
    });

    println!("Please input a math operation");    
    let mut op = String::new();
    
    io::stdin().read_line(&mut op).unwrap_or_else(|e|{
        eprintln!("Error: {e} invalid input");
        process::exit(1);
    });

    let args:Vec<String> = vec![a, b, op];

    if args.len() < 3 {
        eprintln!("Error, too few arguements");
        process::exit(1);
    } 
    let command = Command::new(&args).unwrap_or_else(|err|{
        eprintln!("Error parsing arguements: {err}");
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
        
        println!("args: a{}a", args[0].trim());
        let a = args[0].trim().parse::<f32>();

        let b = args[1].trim().parse::<f32>();
        
        if a.is_err(){
            return Err("Cannot parse for number a arguement");
        }

        if b.is_err(){
            return Err("Cannot parse for number b arguement");
        }
        let op = args[2].trim().to_string().clone();
        
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