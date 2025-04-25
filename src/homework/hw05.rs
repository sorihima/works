fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let x = 48;
    let y = 18;

    println!("GCD of {} and {} is {}", x, y, gcd(x, y));
}
