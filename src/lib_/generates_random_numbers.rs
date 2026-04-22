
use std::{thread, time};
use std::sync::{mpsc};

use rand::RngExt;

pub fn generate_random_number_in_thread(time_in_millis: time::Duration) -> u8{
    //variables qui permettent d'envoyer des données entre les threads
    let (tx, rx) = mpsc::channel::<u8>();

        thread::spawn(move || {
            let mut random_number = rand::rng();
            let random_number: u8 =  random_number.random_range(0..255);
            tx.send(random_number).unwrap();
        });
        
        thread::sleep(time_in_millis);
        let received: u8 = rx.recv().unwrap();
        received
}
