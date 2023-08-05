const SERVER_ADDR: &str = "127.0.0.1:8888";

use std::sync::mpsc;
use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let (fx, tx) = mpsc::channel::<String>();
}