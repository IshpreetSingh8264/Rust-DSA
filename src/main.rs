fn main() {
    let string = String::from("mom");
    let result = recursice_palindrome(&string);
    println!("Is '{}' a palindrome? {}", string, result);
}

fn recursice_palindrome(s:&str) -> bool{
    if s.len() <=1{
        return true;
    }

    let bytes = s.as_bytes();

    if bytes[0] != bytes[s.len()-1]{
        return false;
    }

    recursice_palindrome(&s[1..s.len()-1])
}