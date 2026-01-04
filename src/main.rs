fn main() {
    let number = 153;
    if amrstrong_number(number){
        println!("{} is an Armstrong number", number);
    }else{
        println!("{} is not an Armstrong number", number);
    }
}

fn amrstrong_number(num: i32)-> bool{
    let mut sum = 0;
    let mut temp = num;
    let digits = num.to_string().len() as u32;
    while temp != 0{
        let rem = temp %10;
        sum += rem.pow(digits);
        temp /=10;
    }
    return sum == num;
}
