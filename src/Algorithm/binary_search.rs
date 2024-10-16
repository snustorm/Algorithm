


//Binary search 
pub fn binary_search(arr: [char; 6], start: usize, end: usize, target: char) -> Option<usize>{
    
    if start > end {
        return None;
    }

    let middle = (start + end) / 2;
    let middle_value = arr[middle];

    if middle_value == target {
        return Some(middle);
    } else if middle_value > target {
        return binary_search(arr, start, middle - 1, target);
    } else {
        return binary_search(arr, middle + 1, end, target);
    }

}