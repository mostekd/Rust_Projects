use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    loop {
        println!("\nWybierz opcję:");
        println!("1. Stoper");
        println!("2. Minutnik");
        println!("3. Pomodoro");
        println!("4. Wyjście");
        print!("Twój wybór: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => run_stopwatch(),
            "2" => run_timer(),
            "3" => run_pomodoro(),
            "4" => break,
            _ => println!("Nieprawidłowy wybór!"),
        }
    }
}

fn run_stopwatch() {
    println!("\nStoper: Naciśnij Enter, aby rozpocząć. Potem Enter, aby zatrzymać.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let start = Instant::now();
    println!("Stoper uruchomiony... Naciśnij Enter, aby zatrzymać.");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let elapsed = start.elapsed();
    println!(
        "Czas: {:02}:{:02}:{:02}.{:03}",
        elapsed.as_secs() / 3600,
        (elapsed.as_secs() % 3600) / 60,
        elapsed.as_secs() % 60,
        elapsed.subsec_millis()
    );
}

fn run_timer() {
    println!("\nMinutnik: Podaj czas w sekundach:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let secs: u64 = match input.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("Nieprawidłowa liczba!");
            return;
        }
    };
    println!("Minutnik uruchomiony na {} sekund...", secs);
    for i in (1..=secs).rev() {
        print!("\rPozostało: {:02}:{:02}", i / 60, i % 60);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
    println!("\nCzas minął!");
}

fn run_pomodoro() {
    println!("\nPomodoro: 25 minut pracy, 5 minut przerwy. Ile cykli chcesz wykonać?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cycles: u32 = match input.trim().parse() {
        Ok(c) => c,
        Err(_) => {
            println!("Nieprawidłowa liczba!");
            return;
        }
    };
    for i in 1..=cycles {
        println!("\nCykl {}: Praca przez 25 minut...", i);
        countdown_minutes(25);
        println!("Czas na przerwę: 5 minut...");
        countdown_minutes(5);
    }
    println!("\nPomodoro zakończone!");
}

fn countdown_minutes(mins: u64) {
    for m in (0..mins).rev() {
        for s in (0..60).rev() {
            print!("\rPozostało: {:02}:{:02}", m, s);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
    println!("");
}
