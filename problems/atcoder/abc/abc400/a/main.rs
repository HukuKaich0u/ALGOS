fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();
    
    if 400 % n == 0 {
        println!("{}", 400 / n);
    } else {
        println!("-1");
    }
}
