fn main() {
    let arr = [12, 35, 1, 10, 34, 1];
    let (second_smallest, second_largest) = find_second_smallest_largest(&arr);
    println!("Second Smallest: {}", second_smallest);
    println!("Second Largest: {}", second_largest);
}

fn find_second_smallest_largest(arr: &[i32]) -> (i32, i32) {
    let mut first_smallest = i32::MAX;
    let mut second_smallest = i32::MAX;
    let mut first_largest = i32::MIN;
    let mut second_largest = i32::MIN;

    for &num in arr {
        if num < first_smallest {
            second_smallest = first_smallest;
            first_smallest = num;
        } else if num > first_smallest && num < second_smallest {
            second_smallest = num;
        }

        if num > first_largest {
            second_largest = first_largest;
            first_largest = num;
        } else if num < first_largest && num > second_largest {
            second_largest = num;
        }
    }

    let second_smallest_result = if second_smallest == i32::MAX { -1 } else { second_smallest };
    let second_largest_result = if second_largest == i32::MIN { -1 } else { second_largest };

    (second_smallest_result, second_largest_result)
}