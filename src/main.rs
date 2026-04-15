use std::time::Duration;
use std::{thread, time};
use std::io::{Write, stdout};
use std::io;
use std::sync::{mpsc, Arc, Mutex};

use std::future::Future;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, poll};
//use raw_input::{Core,Event,Key, Listen};
use rand::RngExt;

fn main() -> io::Result<()> {
    
    let time_in_millis = time::Duration::from_millis(50); 
    let mut vec_of_bytes: Vec<u8> = vec![];
    //let new_event = Event{KeyDown{key: }};
    //let mut arc_is_pressed= Arc::new(Mutex::new(false));
    let mut is_pressed = false;

    while is_pressed!=true{
        
        vec_of_bytes.push(generate_random_number_in_thread(time_in_millis));
        println!("test {vec_of_bytes:?}");
        //thread::sleep(time::Duration::from_millis(500));
        is_pressed = listen_to_key_stroke_cross();
       
       //is_pressed = listen_to_key_stroke(vec_of_bytes.clone());
       
       
    };
    Ok(())
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

 fn listen_to_key_stroke_cross() -> bool {
         thread::spawn( move || {
            
           if let  Event::Key(key_event) = event::read().expect("failed to read event"){
                match key_event {
                    key_event_kind =>{
                        if key_event_kind.is_release()==true{}else{println!("key {key_event_kind:?}");}},
                }
            } 
        }); 
        false
        //poll(Duration::from_secs(0))
            
}

/*fn listen_to_key_stroke(mut vec_of_bytes: Vec<u8>) -> bool{
        
        let (tx, rx) = mpsc::channel::<bool>();
        thread::spawn(|| {
            Core::start().expect("Failed to start raw-input core");
        });
        let new_vec  =vec_of_bytes.clone();
        Listen::start();
        Listen::subscribe( move|event| {
            match event {
                Event::KeyDown { key } => {tx.send(true).expect("sender keydown failed") ; println!("key: {key:?} vec {:?}", new_vec); },
                _ => {},
            }
        });
        
        let received:bool = rx.recv().expect("receiver failed");
        Listen::stop(); // Stop Listen
        Core::stop();
    received
}*/
