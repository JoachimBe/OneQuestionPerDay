use std::net::UdpSocket;

pub fn create_sender_udp_port(vec_of_bytes: Vec<u8>){
        let sender_socket = UdpSocket::bind("127.0.0.1:30615").expect("Couldn't bind to address");
        sender_socket.send_to(&vec_of_bytes, "127.0.0.1:3615").expect("couldn't send to address");
}

pub fn create_listen_udp_port_3615()-> [u8;200]{

        let socket = UdpSocket::bind("127.0.0.1:3615").expect("failed to create socket, couldn't bind to address");
        let mut listen_buffer:[u8;200]= [0;200]; //buffer of 200 bytes maximum

        //destructuring  response
        let (number_of_bytes, src_addr) = socket.recv_from(&mut listen_buffer).expect("couldn't write on the buffer");
        
        let filled_buffer =&mut listen_buffer[..number_of_bytes];
        println!("{} {:?}", src_addr, filled_buffer);
        listen_buffer
}

