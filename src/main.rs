fn main() {
    let res = find_factorial(5);
    println!("{}",res);
}

fn find_factorial(num:i32) -> i32{
    if num == 0 {
        return 1;
    }
    num * find_factorial(num - 1)
}