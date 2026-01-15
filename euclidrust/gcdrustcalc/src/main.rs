fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n !=0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

use std::io;

fn main() {
    println!("GCD Calculator");
    println!("Enter two positive integers to find their GCD:");

    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
        .expect("Failed to read line");
    
    let n: u64 = input1.trim().parse()
        .expect("Please enter a valid positive integer");

    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");
    
    let m: u64 = input2.trim().parse()
        .expect("Please enter a valid positive integer");

    if n == 0 || m == 0 {
        println!("Error: Both numbers must be greater than 0");
        return;
    }

    let result = gcd(n, m);
    println!("The GCD of {} and {} is: {}", n, m, result);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 13 * 19), 3);
}

