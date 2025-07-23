fn merge<T: Ord + Copy>(left: &[T],
                        right: &[T] ) -> Vec<T> {

    let mut merged: Vec<T> = vec![];
    let (mut i, mut j) = (0, 0);

    // can convert to a for loop with several if else's.
    while i < left.len() && j < right.len(){
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        merged.extend_from_slice(&left[i..]);
    }

    if j < right.len() {
        merged.extend_from_slice(&right[j..]);
    }
    merged
}

/// Time complexity is O(n log(n)) in all cases. Sorts an array of numbers.
/// # Parameters
/// * data: &[[T]] - a reference to an array containing numerical data.
/// # Returns
/// * Vec<[T]> - a vector containing the sorted contents of the parameter array.
pub fn merge_sort<T: Ord + Copy>(data: &[T]) -> Vec<T> {
    if data.len() < 2 {
        return data.to_vec();
    }

    let mid = data.len() / 2;
    let left_array: &[T] = &data[..mid];
    let right_array: &[T] = &data[mid..];

    if data.len() == 2 {
        return merge(left_array, right_array);
    }
    merge(&merge_sort(left_array), &merge_sort(right_array))
}