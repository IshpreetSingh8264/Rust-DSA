fn main() {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest_element(&numbers);
    println!("The largest element is {}", result);

}


fn largest_element(arr: &[i32]) -> i32 {
    let mut largest = arr[0];
    for &item in arr.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}