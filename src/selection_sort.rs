/// Time complexity is O(n^2) for all cases.
/// # Parameters
/// * data: &[[T]] - a reference to an array containing numerical data.
/// # Returns
/// * Vec<[T]> - a vector containing the sorted contents of the parameter array.
pub fn selection_sort<T: Ord + Copy>(data: &[T]) -> Vec<T> {
    
    let mut sorted: Vec<T> = data.to_vec();
    let n = sorted.len();
    
    for i in 0..n {
        let mut min = i;
        for j in (i + 1)..n {
            if sorted[j] < sorted[min] {
                min = j;
            }
        }
        sorted.swap(min,i);
    }
    sorted
}