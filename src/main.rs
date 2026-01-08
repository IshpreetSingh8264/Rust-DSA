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

    let mut highest_freq = (0, 0); 
    let mut lowest_freq = (0, i32::MAX);

    for (&key, &value) in &freq_map {
        if value > highest_freq.1 {
            highest_freq = (key, value);
        }
        if value < lowest_freq.1 {
            lowest_freq = (key, value);
        }
    }

    println!("Highest frequency element: {} with frequency {}", highest_freq.0, highest_freq.1);
    println!("Lowest frequency element: {} with frequency {}", lowest_freq.0, lowest_freq.1);
}