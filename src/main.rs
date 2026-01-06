fn main() {
    let x = 2;
    print_fib_upto(x);

}

fn recursive_fib(n:i32) ->i32{
    if n <=1{
        return n;
    }
    recursive_fib(n-1) + recursive_fib(n-2)
}

fn print_fib_upto(n: i32) {
    if n < 0 {
        return;
    }
    print_fib_upto(n - 1);
    println!("{}", recursive_fib(n));
}
