use std::net::UdpSocket;

fn main() -> std::io::Result<()>{
    {

        let socket = UdpSocket::bind("127.0.0.1:3615").expect("failed to create socket, couldn't bind to address");
        
        //socket.send_to(&[0,1,2,3,4,5,6,7,8,9], "127.0.0.1:3615").expect("Couldn't send data");
        
        let mut listen_buffer = [0;10];
        
        //destructuring socket.recv_from in 2 variable
        let (number_of_bytes,src_addr) = socket.recv_from(&mut listen_buffer).expect("couldn't write on the buffer, didn't receive data");
        
        let filled_buffer = &mut listen_buffer[..number_of_bytes];
        
        println!("{} {:?}", src_addr, filled_buffer);
        
    }
    Ok(())
}

