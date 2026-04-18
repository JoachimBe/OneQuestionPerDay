//for UDP
use std::net::UdpSocket;

//for generating the byte array
use std::time::Duration;
use std::{thread, time};
use std::sync::{mpsc};

use crossterm::event::{self, Event};
use rand::RngExt;

fn main() {
    loop{

        thread::spawn(||{
            gen_array();
        });
        
        listen_udp_port_3615();
    }

}

 fn listen_udp_port_3615(){

        let socket = UdpSocket::bind("127.0.0.1:3615").expect("failed to create socket, couldn't bind to address");
        let mut listen_buffer:[u8;200]= [0;200]; //buffer of 200 bytes maximum

        //destructuring  response
        let (number_of_bytes, src_addr) = socket.recv_from(&mut listen_buffer).expect("couldn't write on the buffer");
        
        let filled_buffer =&mut listen_buffer[..number_of_bytes];
        println!("{} {:?}", src_addr, filled_buffer);
        
}


fn gen_array() {
    let sender_socket = UdpSocket::bind("127.0.0.1:30615").expect("Couldn't bind to address");
    let time_in_millis = time::Duration::from_millis(50); 
        let mut vec_of_bytes: Vec<u8> = vec![];
            println!("numbers are generated please press a key to show the range");

        loop{
            vec_of_bytes.push(generate_random_number_in_thread(time_in_millis));
            
            //event::poll permet d'attendre un evenement sans bloquer le thread
            // ici on attend qu'un évenment key soit pressé 
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Event::Key(key_event_kind) = event::read().unwrap() {    // si l'event est lu correctement on effectue le block{ }
                    if key_event_kind.is_press()==true{
                        break;
                    }else{
                        continue;
                    }                    
                }
            }
        };
        sender_socket.send_to(&vec_of_bytes, "127.0.0.1:3615").expect("couldn't send to address");
}



fn generate_random_number_in_thread(time_in_millis: time::Duration) -> u8{
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
