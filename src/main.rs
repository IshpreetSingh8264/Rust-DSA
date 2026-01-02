fn reverse(mut num:i32)->i32{
    let mut rev = 0;
    while num!=0{
        let digit = num %10;
        num /=10;
         if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && digit > 7){
                return 0;
            }
            if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && digit < -8){
                return 0;
            }
        rev = rev * 10 + digit;
    }
    return rev;

}


fn main() {
    let number = -123;
    let reversed = reverse(number);
    println!("reversed: {}", reversed);
}
