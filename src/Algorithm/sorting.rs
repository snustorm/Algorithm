

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
