use std::collections::HashMap;

fn main() {
    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    count_frequency(&arr);

}

fn count_frequency(arr: &[i32]) {
    let mut freq_map = HashMap::new();

    for &num in arr {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    for (key, value) in &freq_map {
        println!("{}: {}", key, value);
    }
}