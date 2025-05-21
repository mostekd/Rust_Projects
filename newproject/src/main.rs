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

fn sum_odd_numbers(numbers: &Vec<i32>) -> i32 {
    numbers.iter().filter(|n| *n % 2 != 0).sum()
}

fn contains_number(numbers: &Vec<i32>, value: i32) -> bool {
    numbers.contains(&value)
}

fn multiply_by_two(numbers: &Vec<i32>) -> Vec<i32> {
    numbers.iter().map(|n| n * 2).collect()
}

fn print_greater_than(numbers: &Vec<i32>, threshold: i32) {
    println!("Liczby większe od {}:", threshold);
    for num in numbers {
        if *num > threshold {
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
    println!("Suma liczb nieparzystych w wektorze: {}", sum_odd_numbers(&numbers));
    let value = 3;
    println!("Czy liczba {} występuje w wektorze? {}", value, contains_number(&numbers, value));
    let doubled = multiply_by_two(&numbers);
    println!("Nowy wektor z liczbami pomnożonymi przez 2: {:?}", doubled);
    print_greater_than(&numbers, 3);
}
