use super::generates_random_numbers::generate_random_number_in_thread;

use crossterm::event::{self, Event};
use std::time;
pub fn gen_array()-> Vec<u8>  {
    
    let time_in_millis = time::Duration::from_millis(50); 
        let mut vec_of_bytes: Vec<u8> = vec![];
            println!("numbers are generated please press a key to show the range");

        loop{
            vec_of_bytes.push(generate_random_number_in_thread(time_in_millis));
            
            //event::poll permet d'attendre un evenement sans bloquer le thread
            // ici on attend qu'un évenment key soit pressé 
            if event::poll(time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event_kind) = event::read().unwrap() {    // si l'event est lu correctement on effectue le block{ }
                    if key_event_kind.is_press()==true{
                        break;
                    }else{
                        continue;
                    }                    
                }
            }
        };
        vec_of_bytes
}