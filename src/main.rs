fn count(mut num:i32)->i32{
    if num < 0 {
        // return count(-num);
        num *= -1;

    }
        let mut n = num;
        let mut count =0;
        while n>0 {
            n /=10;
            count +=1;
        } 
        return count;
    
}


fn main() {
    println!("Hello, world!");
    // Count Digits in a number
    let number = 555;
    let count = count(number);
    println!("Count: {}", count);
}
