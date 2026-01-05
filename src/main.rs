fn main() {
    print_numbers_recursive_reverse(10);
}

fn print_numbers_recursive_reverse(n:i32){
    println!("{}",n);
    if n >1{
        print_numbers_recursive_reverse(n-1);
    }
}