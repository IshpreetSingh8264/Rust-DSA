fn main() {
    print_name_recursive(5);
}

fn print_name_recursive(count:i32){
    if count <=0{
        return;
    }

    println!("Ishpreet Singh");
    print_name_recursive(count-1);
}