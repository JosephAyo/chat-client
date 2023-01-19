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

    thread::spawn(move || loop {
        let mut message_buff = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut message_buff) {
            Ok(_) => {
                let message = String::from_utf8(message_buff).expect("buffer not valid utf8");
                println!("received message: {}", message);
            }
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("lost connection to server");
                break;
            }
        }

        match receiver.try_recv() {
            Ok(message) => {
                let mut message_buff = message.clone().into_bytes();
                message_buff.resize(MESSAGE_SIZE, 0);
                client
                    .write_all(&message_buff)
                    .expect("failed to message send to socket");
                println!("Message sent {:?}", message);
            },
            Err(_) => (),
        }
    });

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
