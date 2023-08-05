const SERVER_ADDR: &str = "127.0.0.1:8888";

use std::sync::mpsc;
use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(SERVER_ADDR).expect("Adress binding err.");
    server.set_nonblocking(true).expect("Block set err.");
    println!("Server start at {}", SERVER_ADDR);

    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("New client connect at {}", addr);
            clients.push(client.try_clone().expect("Cloning failure"));
            start_thread(client, tx.clone());
        }

        if let Ok(msg) = rx.recv() {
            println!("Receive message. Broadcast to all clients : {}", msg);
            clients = send_all(clients, &msg);
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut messasge: String = String::new();
        if let Ok(n) = reader.read_line(&mut messasge) {
            if n > 0 {
                tx.send(messasge).expect("Thread tranfer err.");
            }
        }
    });
}

fn send_all(clients: Vec<TcpStream>, msg: &str) -> Vec<TcpStream> {
    let mut collector: Vec<TcpStream> = Vec::new();
    let data = String::from(msg).into_bytes();

    for mut socket in clients.into_iter() {
        if let Err(_) = socket.write_all(&data) {
            println!("Message send err. End connection.");
            continue;
        }

        collector.push(socket);
    }

    collector
}