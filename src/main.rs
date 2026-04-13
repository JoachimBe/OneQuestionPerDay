use std::{thread, time};
use rand::RngExt;

fn main() {
    random_number_generator_thread()
}

fn random_number_generator_thread(){
    let time_in_millis = time::Duration::from_millis(1000); 
    loop{
        thread::spawn(move || {
            let mut random_number = rand::rng();
            let random_number: u8 =  random_number.random_range(0..255);
            thread::sleep(time_in_millis);
        });
    };
}