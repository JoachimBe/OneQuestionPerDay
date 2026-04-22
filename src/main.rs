use std::collections::VecDeque;
use std::thread;

mod lib_;
use lib_::generates_bytes_of_random_numbers;
use lib_::udp_sockets;


fn main() {
    let mut red: Option<u8> = None;
    let mut green: Option<u8>= None;
    let mut blue: Option<u8>= None;

    thread::spawn(||{
        udp_sockets::create_sender_udp_port(generates_bytes_of_random_numbers::gen_array());
    });

    let mut received_byte = VecDeque::from(udp_sockets::create_listen_udp_port_3615());
    while red.is_none() || green.is_none() ||  blue.is_none(){
        if let Some(byte) = received_byte.pop_front(){
            if red.is_none(){
                red = Some(byte);
            } else if green.is_none(){
                green = Some(byte);
            } else if blue.is_none(){
                blue = Some(byte);
            }
        } else{
            break;
        }
    }
    print!("\x1b[38;2;{};{};{}m■", red.unwrap(), green.unwrap(), blue.unwrap()); 

}
