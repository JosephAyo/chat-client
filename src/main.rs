use std::{
    io::{stdin, ErrorKind, Read, Write},
    net::TcpStream,
    sync::mpsc::channel,
    thread,
};

const LOCAL_ADDR: &str = "127.0.0.1:3422";
const MESSAGE_SIZE: usize = 32;

fn main() {
    let mut client =
        TcpStream::connect(LOCAL_ADDR).expect("Stream failed to connect on local address");
    client
        .set_nonblocking(true)
        .expect("server failed to set to non blocking");

    let (sender, receiver) = channel::<String>();

    loop {
        let mut user_input = String::new();

        println!("Enter your message: ");

        stdin()
            .read_line(&mut user_input)
            .expect("failed to read user input");

        sender
            .send(user_input)
            .expect("failed to send message over channel");
    }
}
