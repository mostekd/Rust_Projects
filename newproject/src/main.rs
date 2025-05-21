fn average(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

fn max_number(numbers: &Vec<i32>) -> i32 {
    *numbers.iter().max().unwrap()
}

fn print_even_numbers(numbers: &Vec<i32>) {
    println!("Liczby parzyste w wektorze:");
    for num in numbers {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}

fn main() {
    println!("Hello, world!");
    for i in 0..5 {
        println!("Liczba: {}", i);
    }
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    for num in &numbers {
        sum += num;
        println!("Dodaję {} do sumy, aktualna suma: {}", num, sum);
    }
    println!("Suma wszystkich liczb w wektorze: {}", sum);
    println!("Średnia liczb w wektorze: {:.2}", average(&numbers));
    println!("Największa liczba w wektorze: {}", max_number(&numbers));
    print_even_numbers(&numbers);
}
