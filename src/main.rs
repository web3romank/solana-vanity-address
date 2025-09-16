// src/main.rs
use solana_sdk::signature::{Keypair, Signer};
use bs58;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::time::{Instant, Duration};
use std::thread;
use std::env;

fn main() {
    // читаем префиксы из аргументов: cargo run --release -- f fa moon
    let args: Vec<String> = env::args().skip(1).collect();
    let prefixes: Vec<String> = if args.is_empty() {
        vec!["f".to_string()] // дефолтный префикс
    } else {
        args
    };

    println!("Searching for prefixes: {:?}", prefixes);

    let start_time = Instant::now();
    let counter = Arc::new(AtomicU64::new(0));
    let found_flag = Arc::new(AtomicBool::new(false));

    // поток-репортер: печатает статистику каждую 1 секунду, пока флаг не установлен
    {
        let counter_r = Arc::clone(&counter);
        let found_r = Arc::clone(&found_flag);
        let start_r = start_time.clone();
        thread::spawn(move || {
            let mut last_print = Instant::now();
            while !found_r.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(200)); // легкая пауза
                if last_print.elapsed() >= Duration::from_secs(1) {
                    let checked = counter_r.load(Ordering::Relaxed);
                    let dur = start_r.elapsed().as_secs_f64().max(1e-9);
                    let speed = (checked as f64) / dur;
                    println!("Checked addresses: {} | {:.2} addr/s", checked, speed);
                    last_print = Instant::now();
                }
            }
        });
    }

    // параллельный поиск
    let n_threads = num_cpus::get();
    let prefixes_arc = Arc::new(prefixes); // шарим в потоки

    let result = (0..n_threads)
        .into_par_iter()
        .find_map_any(|_| {
            // локальная переменная для сокращения обращений к Arc
            let prefixes = Arc::clone(&prefixes_arc);
            // каждый поток выполняет бесконечный цикл, пока кто-то не найдет результат
            loop {
                // можно заранее прервать, если кто-то уже нашел
                if found_flag.load(Ordering::Relaxed) {
                    return None;
                }

                let keypair = Keypair::new();
                // encode pubkey to base58 string
                let pubkey_b58 = bs58::encode(keypair.pubkey()).into_string();

                // увеличиваем атомарный счетчик
                counter.fetch_add(1, Ordering::Relaxed);

                // проверка по списку префиксов
                for p in prefixes.iter() {
                    if pubkey_b58.starts_with(p) {
                        // пометим, что нашли (репортер остановится)
                        // (необязательно для корректности, но полезно для быстрого завершения)
                        found_flag.store(true, Ordering::Relaxed);

                        let priv_b58 = bs58::encode(keypair.to_bytes()).into_string();
                        return Some((pubkey_b58, priv_b58));
                    }
                }
            }
        })
        .expect("Failed to generate vanity address");

    // гарантируем, что репортер увидит флаг и остановится
    found_flag.store(true, Ordering::Relaxed);

    let elapsed = start_time.elapsed().as_secs_f64().max(1e-9);
    let checked = counter.load(Ordering::Relaxed);
    println!("Done. Time: {:.2}s, Checked: {}, avg speed: {:.2} addr/s", elapsed, checked, checked as f64 / elapsed);
    println!("Found address: {}", result.0);
    println!("Private key (base58): {}", result.1);
}