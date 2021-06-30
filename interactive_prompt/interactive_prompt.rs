// This can be improved with rustyline

use std::io;
use std::collections::HashMap;

type CliResult = Result<String,String>;

// D is any datatype with size
struct Cli<'a, D> {
    data: D,
    help: HashMap<String, String>,
    callbacks: HashMap<String, Box<dyn Fn(&mut D, &[&str]) -> CliResult + 'a>>
}

// Implementations for Cli struct
impl <'a, D: Sized> Cli<'a, D> {

    // Creates new Cli struct
    fn new(data: D) -> Cli<'a, D> {
        Cli{data: data, help: HashMap::new(), callbacks: HashMap::new()}
    }

    // Creates a new command by inserting a hashmap with a name (how we access the command)
    // and a closure (the command to execute)
    fn cmd<F>(&mut self, name: &str, help: &str, callback: F)
    where F: Fn(&mut D, &[&str])->CliResult + 'a {
        self.help.insert(name.to_string(), help.to_string());
        self.callbacks.insert(name.to_string(), Box::new(callback));
    }

    // Processes a command. Splits input up by whitespace. The part is
    // used to access the command name, the rest are then passed to the callback
    // as arguments
    fn process(&mut self, line: &str) -> CliResult {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 0 { return Ok("".to_string()); }
        
        // Check to see if there is space for a 'help' flag and
        // if the help flag exists
        if parts.len() > 1 && parts[1] == "help".to_string() { 
            match self.help.get(parts[0]) {
                Some(_) => {
                    println!("{}", self.help.get(parts[0]).unwrap());
                    Ok("".to_string())
                }
                None => Err("No such command".to_string())
            }
        } else { 
            match self.callbacks.get(parts[0]) {
                Some(callback) => callback(&mut self.data, &parts[1..]),
                None => Err("No such command".to_string())
            }
        }
    }

    // Processes user input to call commands
    fn go(&mut self) {
        let mut buff = String::new();
        while io::stdin().read_line(&mut buff).expect("Error") > 0 {
            let line = buff.trim_start();
            let res = self.process(line);
            println!("{:?}",res);
                
            buff.clear();
        }
    }
}

fn ok<T: ToString>(s: T) -> CliResult {
    Ok(s.to_string())
}

fn err<T: ToString>(s: T) -> CliResult {
    Err(s.to_string())
}

fn main() {
    println!("Welcome to the Interactive Prompt! ");

    struct Data {
        answer: i32
    }

    let mut cli = Cli::new(Data{answer: 42});

    cli.cmd("go", "'go' will call the go command and store the first argument",|data, args| {
        if args.len() == 0 { return err("Need at least 1 argument"); }
        data.answer = match args[0].parse::<i32>() {
            Ok(n) => n,
            Err(e) => return err(e)
        };
        println!("got {:?}", args);
        ok(data.answer)
    });

    cli.cmd("show","'show' will display the last stored argument", |data, _| {
        ok(data.answer)
    });

    cli.go();
}