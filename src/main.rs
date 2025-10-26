// Intentionally poorly formatted Rust code to trigger rustfmt

fn main() {
    println!("Hello, CI fmt test!");
    let mut numbers = vec![3, 1, 2];
    for n in numbers.iter_mut() {*n += 1;println!("{}", n);
    }
    let sum: i32 = numbers.iter().copied().sum();
    println!("sum={}", sum)
}
