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
}
