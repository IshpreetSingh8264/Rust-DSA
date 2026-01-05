fn main() {
    let res = find_sum_of_first_n_numbers_recursive(4);
    println!("{}",res);
    let res2:i32 = find_sum_of_first_n_numbers_recursive_formula(4);
    println!("{}",res2);
}

fn find_sum_of_first_n_numbers_recursive(n:i32)->i32{
    if n == 1{
        return 1;
    }
    return n + find_sum_of_first_n_numbers_recursive(n-1);
}
fn find_sum_of_first_n_numbers_recursive_formula(n:i32)->i32{
    return n*(n+1)/2;
}