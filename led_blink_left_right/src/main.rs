use std::io::Write;
use std::{thread, time};

fn main() {
    //time for each loop
    let time_in_millis_for_each_loop = time::Duration::from_millis(500);
    loop{
        print_stripe_led(1);

        thread::sleep(time_in_millis_for_each_loop);
        clear_line();
        //move cursor to right
        print_stripe_led(10);
        std::io::stdout().flush().expect("error while flush"); // on flush le buffer de stdout (voir lien )

        thread::sleep(time_in_millis_for_each_loop);
        clear_line();
    }
}

pub fn clear_line(){
    print!("\x1bc");
    std::io::stdout().flush().expect("error while flush"); // on flush le buffer de stdout (voir lien )
}

pub fn print_stripe_led(column: u32){
    let red= 255;
    let green = 0;
    let blue = 0;
    let mut i = 1;
    while i< 10{
        //48;2;{255};{0};{0}m
        print!("\x1b[{};{}H\x1b[31m\x1b[48;2;{red};{green};{blue}m \x1b[?25l\x1b[0m", i, column);
        std::io::stdout().flush().expect("error while flush"); // on flush le buffer de stdout (voir lien )
        i+=1;
    }

}