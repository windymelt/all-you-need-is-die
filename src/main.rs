use std::process::Command;
use std::env;
use std::io::{self, Write};

fn main() {
    // get args
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
 
    //execute command
    let mut cmd = Command::new(&args[1]);
    cmd.args(&args[2..]);
    let mut output = cmd.output();
    while let Ok(o) = output {
        println!("status: {}", o.status);
        io::stdout().write_all(&o.stdout).unwrap();
        io::stderr().write_all(&o.stderr).unwrap();
        if o.status.code() != Some(0) { break; }
        output = cmd.output();
    }

    println!("dead!");
}
