fn main() {
    let vector = vec![1,1,2,1,3,4,5,6,7,8,10,10];
    let result = check_array_sorted(&vector);
    println!("Is the array sorted? {}", result);
}
fn check_array_sorted(arr:&Vec<i32>)->bool{

    for i in 0..arr.len()-1{
        if arr[i] > arr[i+1]{
            return false;
        }
    }
    return true;
        
}