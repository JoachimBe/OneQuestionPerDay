
use std::time::Duration;
use std::{thread, time};
use std::io;
use std::sync::{mpsc};

use crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};
use rand::RngExt;

fn main() -> io::Result<()> {
    
    let time_in_millis = time::Duration::from_millis(50); 
    loop{
        let mut vec_of_bytes: Vec<u8> = vec![];
            println!("numbers are generated please press a key to show the range");

        loop{
            vec_of_bytes.push(generate_random_number_in_thread(time_in_millis));
            
            //event  poll permet d'attendre un evenement sans bloquer le thread 
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event_kind) = event::read().unwrap() {// si l'event est lu correctement on effectue le block
                    if key_event_kind.is_release()==true{
                        continue;
                    }else{
                        break;
                    }                    
                }
            }
        };
        println!(" {vec_of_bytes:?}");
    }
}


fn generate_random_number_in_thread(time_in_millis: time::Duration) -> u8{

        let (tx, rx) = mpsc::channel::<u8>();


        thread::spawn(move || {
            let mut random_number = rand::rng();
            let random_number: u8 =  random_number.random_range(0..255);
            tx.send(random_number).unwrap();
            
        });

        thread::sleep(time_in_millis);
        let received = rx.recv().unwrap();
        received
}
