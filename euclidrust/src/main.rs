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

fn main() {
    println!("GCD Calculator");
    println!("GCD of 14 and 15: {}", gcd(14, 15));
    println!("GCD of 48 and 18: {}", gcd(48, 18));
    println!("GCD of 56 and 42: {}", gcd(56, 42));
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 13 * 19), 3);
}
