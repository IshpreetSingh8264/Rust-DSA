fn main() {
    printNameRecursive(5);
}

fn printNameRecursive(count:i32){
    if count <=0{
        return;
    }

    println!("Ishpreet Singh");
    printNameRecursive(count-1);

}