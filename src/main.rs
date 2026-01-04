fn main() {
    let number = 121;
    if is_palindrome(number) {
        println!("{} is a palindrome.", number);
    } else {
        println!("{} is not a palindrome.", number);
    }
}
fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}