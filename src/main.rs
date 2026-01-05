fn main() {
    let array: &mut [i32] = &mut [10, 20, 30, 40, 50];
    
    reverse_array(array, 0, array.len() - 1);
    println!("{:?}", array);
}


fn reverse_array(arr: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }
    // Swap elements at start and end
    let temp = arr[start];
    arr[start] = arr[end];
    arr[end] = temp;

    // Recursive call
    reverse_array(arr, start + 1, end - 1);
}