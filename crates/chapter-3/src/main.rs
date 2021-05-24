fn main() {
    println!("92°C in is {}°F", to_fahrenheit(92.0));
    println!("92°F in is {}°C", to_celcius(92.0));
    println!("{}", fibonacci(92));
    print_lyrics();
}

fn to_fahrenheit(deg: f32) -> f32 {
    deg * 1.8 + 32.0
}

fn to_celcius(deg: f32) -> f32 {
    (deg - 32.0) / 1.8
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn print_lyrics() {}
