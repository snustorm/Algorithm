

//Bubble sort
pub fn bubble_sort<T: PartialOrd + Copy>(arr: &[T]) -> Vec<T> {
    let mut sorted_arr = arr.to_vec();

    let len = sorted_arr.len();

    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if sorted_arr[j] > sorted_arr[j + 1] {
                sorted_arr.swap(j, j + 1);
            }
        }
    }

    sorted_arr
}

pub fn merge_sort<T: PartialOrd + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let middle = arr.len() / 2;
    let (left, right) = arr.split_at(middle);

    let sorted_left = merge_sort(left);
    let sorted_right = merge_sort(right);

    merge(&sorted_left, &sorted_right)
}

//Mergo Sort
fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut merged = Vec::with_capacity(left.len() + right.len());
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            merged.push(left[left_index].clone());
            left_index += 1;
        } else {
            merged.push(right[right_index].clone());
            right_index += 1;
        }
    }

    // Append remaining elements from the left half (if any)
    while left_index < left.len() {
        merged.push(left[left_index].clone());
        left_index += 1;
    }

    // Append remaining elements from the right half (if any)
    while right_index < right.len() {
        merged.push(right[right_index].clone());
        right_index += 1;
    }

    merged
}




//Quick Sort
pub fn quick_sort(arr: &[u32]) -> Vec<u32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let mut sorted_arr = arr.to_vec();
    let pivot_index = sorted_arr.len() / 2 - 1;
    let last_index = sorted_arr.len() - 1;

    // Swap the pivot and last element
    if pivot_index != last_index {
        sorted_arr.swap(pivot_index, last_index);
    }

    let pivot_value = sorted_arr[last_index];

    let mut item_from_left_index = 0;
    let mut item_from_right_index = last_index - 1;

    while item_from_left_index <= item_from_right_index {
        while sorted_arr[item_from_left_index] < pivot_value {
            item_from_left_index += 1;
        }

        while sorted_arr[item_from_right_index] > pivot_value {
            if item_from_right_index == 0 {
                break; // Prevent underflow when decrementing
            }
            item_from_right_index -= 1;
        }

        if item_from_left_index >= item_from_right_index {
            break;
        }

        sorted_arr.swap(item_from_left_index, item_from_right_index);
        item_from_left_index += 1;
        if item_from_right_index > 0 {
            item_from_right_index -= 1;
        }
    }

    // Swap pivot back into its correct position
    sorted_arr.swap(item_from_left_index, last_index);

    // Recursively sort the left and right partitions
    let left_sorted = quick_sort(&sorted_arr[0..item_from_left_index]);
    let right_sorted = quick_sort(&sorted_arr[item_from_left_index + 1..]);

    // Merge sorted subarrays and pivot, return the result
    [left_sorted, vec![pivot_value], right_sorted].concat()
}