use std::io::Write;
use std::{thread, time};

fn main() {
    let time_in_millis_before_erase = time::Duration::from_millis(32); //thread pause to see the print
    let time_in_millis_before_print = time::Duration::from_millis(16); // thread pause to avoid unpredictable fequency
   
    //RGB colors value
    let mut red:u8 =255; //value for red
    let green:u8 = 0; //value for green
    let blue:u8 = 0; //value for blue

    loop{
        if red >3{
            
            print_red(red, green, blue);
            
            //put the thread at sleep before erasing the print
            thread::sleep(time_in_millis_before_erase); 
            
            erase_line(); 
            
            red -=3; //remove value from red so it fade

            // put the thread at sleep to have homogeneous erase frenquency
            thread::sleep(time_in_millis_before_print);

        }
        else{ //while red value is 0
            
            thread::sleep(time::Duration::from_millis(500)); //pause the thread for half-second
            
            red = 255; //set red value to 255

        }
    }
}

fn print_red(red: u8, green: u8, blue: u8){

    print!("\x1b[38;2;{red};{green};{blue}m■"); //print RGB red neacause green and blue are set to 0

    //print is line-buffered by default flush method to ensure the output is emitted immediately
    std::io::stdout().flush().expect("error while flush");
}

fn erase_line(){
    print!("\x1bc\x1b[?25l"); //clear the console and hide cursor

    //print is line-buffered by default flush method to ensure the output is emitted immediately
    std::io::stdout().flush().expect("error while flush"); 
}