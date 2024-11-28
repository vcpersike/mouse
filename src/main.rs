use enigo::{Enigo, MouseControllable};
use rdev::{listen, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("Digite o tempo (em segundos) para o movimento do mouse:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let time_limit: u64 = input.trim().parse().expect("Digite um número válido!");

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                if format!("{:?}", key).to_lowercase() == "keyb" {
                    println!("Tecla 'B' pressionada. Parando o programa...");
                    running_clone.store(false, Ordering::SeqCst);
                }
            }
        }) {
            eprintln!("Erro ao escutar eventos do teclado: {:?}", error);
        }
    });

    let mut enigo = Enigo::new();
    let mut x = 500;
    let mut direction = 1;
    let step = 10; 
    let interval = Duration::from_millis(100);

    let start_time = Instant::now();

    while running.load(Ordering::SeqCst) && start_time.elapsed().as_secs() < time_limit {
        x += direction * step;
        if x > 1500 || x < 500 {
            direction *= -1; 
        }
        enigo.mouse_move_to(x, 500); 
        thread::sleep(interval);
    }

    println!("Movimento automático interrompido.");
}
