fn main() {
    print_numbers_recursive(10);
}

fn print_numbers_recursive(n:i32){
    if n >1{
        print_numbers_recursive(n-1);
    }
    println!("{}",n);
}