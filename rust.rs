Простой сканер портов на языке программирования Rust.

Создание сканера портов на Rust включает в себя использование стандартных библиотек для сетевого программирования. Ниже представлен пример простейшего сканера портов, который работает асинхронно, используя библиотеку `tokio`.

Перед началом Вам необходимо добавить зависимости в ваш `Cargo.toml` файл:

[dependencies]
tokio = { version = "1.0", features = ["full"] }

Теперь Вы можете написать код сканера портов следующим образом:

use tokio::net::TcpStream;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::time;
#[tokio::main]
async fn main() {
    // Получаем аргументы командной строки
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Использование: {} <IP-адрес> <начальный порт> <конечный порт>", args[0]);
        return;
    }
    let ip: IpAddr = args[1].parse().expect("Неверный IP-адрес");
    let start_port: u16 = args[2].parse().expect("Неверный начальный порт");
    let end_port: u16 = args[3].parse().expect("Неверный конечный порт");
    // Сканируем порты в заданном диапазоне
    scan_ports(ip, start_port, end_port).await;
}
async fn scan_ports(ip: IpAddr, start_port: u16, end_port: u16) {
    let mut open_ports: Vec<u16> = Vec::new();
    for port in start_port..=end_port {
        if let Ok(_) = time::timeout(Duration::from_millis(200), TcpStream::connect((ip, port))).await {
            println!("Порт открыт: {}", port);
            open_ports.push(port);
        }
    }
    println!("\nОткрытые порты:");
    if !open_ports.is_empty() {
        for port in open_ports {
            println!("{}", port);
        }
    } else {
        println!("Нет открытых портов в заданном диапазоне.");
    }
}

Этот код создает асинхронную функцию main, которая получает аргументы командной строки для IP-адреса и диапазона портов. Функция scan_ports асинхронно пытается подключиться к каждому порту в указанном диапазоне с таймаутом в 200 мс на каждую попытку. Если подключение успешно, порт считается открытым, и информация о нем выводится в консоль.

Чтобы использовать этот скрипт, скомпилируйте его с помощью cargo build, а затем запустите, передав IP-адрес и диапазон портов:

cargo run 192.168.1.1 1 1024

Перед использованием сканера портов убедитесь, что Вы имеете разрешение на сканирование целевого адреса, так как неавторизованное сканирование может быть незаконным в некоторых юрисдикциях и рассматриваться как вредоносная активность.
