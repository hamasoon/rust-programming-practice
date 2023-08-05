mod threadtest;

use std::sync::mpsc;
use std::thread;
#[allow(unused)]
use threadtest::{sleeptest, mpsctest};

fn main() {
    // 스레드간 통신용 채널 --- (*2)
    let (tx, rx) = mpsc::channel::<String>();
        
    // 스레드 1 생성 --- (*3)
    let sender = tx.clone();
    thread::spawn(|| {
        mpsctest::sleep_sender("우영우", sender)
    });
    // 스레드 2 생성
    let sender = tx.clone();
    thread::spawn(|| {
        mpsctest::sleep_sender("이준호", sender)
    });
    // 스레드로부터 메시지를 반복해서 받음 --- (*4)
    loop {
        let buf = rx.recv().unwrap();
        println!("[수신] {}", buf);
        if buf == "quit" { break; }
    }
}
