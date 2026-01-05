fn main() {
    let num1 = 56;
    let num2 = 98;
    let result = gcd(num1, num2);
    println!("The GCD of {} and {} is {}", num1, num2, result);    
}
fn gcd(n1: i32, n2: i32) -> i32 {
    if n2 == 0 {
        n1.abs()
    } else {
        gcd(n2, n1 % n2)
    }
}