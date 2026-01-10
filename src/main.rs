fn main() {
    let vector = vec![1,1,2,3,4,5,6,7,8,10,10];
    let result = find_second_smallest_largest_using_sorting(&vector);
    println!("Second Smallest: {}, Second Largest: {}", result[0], result[1])

}

fn find_second_smallest_largest_using_sorting(arr: &Vec<i32>) -> [i32; 2]{
    let mut sorted_array = arr.clone();
    sorted_array.sort_unstable();
    sorted_array.dedup();
    let second_smallest = sorted_array[1];
    let second_largest = sorted_array[sorted_array.len() - 2];
    [second_smallest, second_largest]
}
