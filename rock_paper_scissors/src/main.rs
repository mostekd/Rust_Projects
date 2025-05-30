use std::io;
use rand::Rng;

fn main() {
    println!("Witaj w grze Kamień, Papier, Nożyce!");
    let choices = ["Kamień", "Papier", "Nożyce"];
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;

    loop {
        println!("\nWybierz: 1 - Kamień, 2 - Papier, 3 - Nożyce, 0 - Wyjście");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Błąd odczytu");
        let user_choice: u32 = match user_input.trim().parse() {
            Ok(0) => {
                println!("Koniec gry!");
                break;
            },
            Ok(num) if num >= 1 && num <= 3 => num,
            _ => {
                println!("Nieprawidłowy wybór!");
                continue;
            }
        };

        let computer_choice = rand::thread_rng().gen_range(1..=3);
        println!("Twój wybór: {}", choices[(user_choice - 1) as usize]);
        println!("Wybór komputera: {}", choices[(computer_choice - 1) as usize]);

        match (user_choice, computer_choice) {
            (a, b) if a == b => {
                println!("Remis!");
                draws += 1;
            },
            (1, 3) | (2, 1) | (3, 2) => {
                println!("Wygrałeś!");
                wins += 1;
            },
            _ => {
                println!("Przegrałeś!");
                losses += 1;
            },
        }
        println!("Statystyki: Wygrane: {}, Przegrane: {}, Remisy: {}", wins, losses, draws);
    }
}

// Dodatkowa funkcja do pobierania wyboru użytkownika (można rozbudować o obsługę błędów)
fn get_user_choice() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}
