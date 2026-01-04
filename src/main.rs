fn main() {
    println!("Hello World");
    let n = 16;
    let result: i32= divisor_count(n);
    println!("Result is: {}  :)))))",result)

}

fn divisor_count(n: i32) -> i32 {
    
    let mut count: i32 =0;
    let half:i32 = n /2;
    
    for i in 1..=half{
        if n % i == 0{
            count +=1;
        }
    }
    return count
}