use std::{thread, time};
use std::io::Write;

fn main() {
    let time_in_millis =  time::Duration::from_millis(750); 
    loop{
          
        print!("\x1b[31m■\x1b[0m");
        std::io::stdout().flush().expect("error while flush");
        
        thread::sleep(time_in_millis); 
        print!("\x1bc"); 
        std::io::stdout().flush().expect("error while flush"); 

        print!("\x1b[30m□\x1b[0m");
        std::io::stdout().flush().expect("error while flush");
        thread::sleep(time_in_millis);
        
        print!("\x1bc"); 
        std::io::stdout().flush().expect("error while flush"); 
    }
}
