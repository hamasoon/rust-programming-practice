use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 1초마다 메시지를 보내는 함수 --- (*1)
pub fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    let whales = ["큰고래", "혹등고래", "향유고래", "남방큰돌고래", "북극고래"];

    for whale in whales {
        let msg = format!("{}: {}", name, whale);
        println!("[송신] {}", msg);
        sender.send(msg).unwrap(); // 송신
        thread::sleep(Duration::from_millis(1000));
    }
    
    sender.send("quit".to_string()).unwrap();
}
