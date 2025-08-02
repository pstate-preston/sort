pub fn partition<T: Copy + Ord>(data: &mut Vec<T>, start: usize, end: usize) {
    let (mut i, mut j) = (start - 1, end + 1);
    let pivot = data[end];
    while i < j {

    }
}

/// Time complexity is O(n log(n)) best case, O(n^2) worst case.
/// # Parameters
/// * data: &mut Vec<[T]> - a reference to vector containing numerical data.
/// # Returns
/// * Vec<[T]> - the original parameter array, sorted in place.
pub fn quick_sort<T: Copy + Ord>(data: &mut Vec<T>) {
    let n = data.len();
    let pivot = n - 1;
    let (mut i, mut j) = (0, pivot-1);

    while i < j {
        if data[i] > data[pivot] && data[j] < data[pivot] {
            data.swap(i, j);
            i += 1;
            j -= 1;
        } else if data[i] > data[pivot] {
            j -= 1;
        } else {
            i += 1;
        }
    }
    // now, i and j have crossed over partition...
    data.swap(i, pivot);
    let mut left: Vec<T> = data[..pivot].to_vec();
    let mut right: Vec<T> = data[pivot+1..].to_vec();

    quick_sort(&mut left);
    quick_sort(&mut right);
}