use std::io;

fn main() {
    println!("Witaj w grze Kamień, Papier, Nożyce!");
    println!("Wybierz: 1 - Kamień, 2 - Papier, 3 - Nożyce");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Błąd odczytu");
    let user_choice: u32 = match user_input.trim().parse() {
        Ok(num) if num >= 1 && num <= 3 => num,
        _ => {
            println!("Nieprawidłowy wybór!");
            return;
        }
    };

    let computer_choice = rand::random::<u32>() % 3 + 1;
    let choices = ["Kamień", "Papier", "Nożyce"];
    println!("Twój wybór: {}", choices[(user_choice - 1) as usize]);
    println!("Wybór komputera: {}", choices[(computer_choice - 1) as usize]);

    match (user_choice, computer_choice) {
        (a, b) if a == b => println!("Remis!"),
        (1, 3) | (2, 1) | (3, 2) => println!("Wygrałeś!"),
        _ => println!("Przegrałeś!"),
    }
}
