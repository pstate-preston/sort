// returns the index at which the pivot is found after partitioning.
fn partition<T: Copy + Ord>(data: &mut Vec<T>,
                            left: isize,
                            right: isize) -> isize {
    let mut index = left;
    let pivot = data[right as usize];

    for i in left..right {
        if data[i as usize] < pivot {
            data.swap(i as usize, index as usize);
            index += 1;
        }
    }
    data.swap(index as usize, right as usize);
    index
}

fn qs_utility<T: Copy + Ord>(data: &mut Vec<T>,
                             left: isize,
                             right: isize) {

    if left >= right { return; }
    let p = partition(data, left, right);
    qs_utility(data, left, p-1);
    qs_utility(data, p+1, right);
}

/// Time complexity is O(n log(n)) best case, O(n^2) worst case.
/// # Parameters
/// * data: &mut Vec<[T]> - a reference to vector containing numerical data.
/// # Returns
/// * Vec<[T]> - the original parameter array, sorted in place.
pub fn quick_sort<T: Copy + Ord>(data: &mut Vec<T>) {
    if data.len() > 1 {
        let n = data.len() as isize;
        let (i, j) = (0, n-1);
        qs_utility(data, i, j);
    }
}