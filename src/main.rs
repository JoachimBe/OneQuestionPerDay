use std::{thread, time};
use std::sync::mpsc;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::RngExt;

fn main() -> std::io::Result<()> {
    
    crossterm::terminal::enable_raw_mode().expect("enable raw mode failed");
    
    let time_in_millis = time::Duration::from_millis(1000); 
    let mut vec_of_bytes = vec![];
    loop{
        let (tx, rx) = mpsc::channel::<u8>();
        thread::spawn(move || {
            let mut random_number = rand::rng();
            let random_number: u8 =  random_number.random_range(0..255);
            tx.send(random_number).unwrap();
            
        });
        //thread::sleep(time_in_millis);
        let received = rx.recv().unwrap();
        vec_of_bytes.push(received);

        loop{

            if let Event::Key(key) = event::read().expect("failed to read key"){
                if key.kind == KeyEventKind::Press{  
                    match key.code{
                        KeyCode::Char(c) => {println!("You pressed {:?}", vec_of_bytes); break}
                        _ =>{}
                    }
                }
            }
        }
            

    };

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

