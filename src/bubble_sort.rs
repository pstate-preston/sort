/// Time complexity is O(n^2) for worst case, O(n) for best case.
/// # Parameters
/// * data: &[[T]] - a reference to an array containing numerical data.
/// # Returns
/// * Vec<[T]> - a vector containing the sorted contents of the parameter array.
pub fn bubble_sort<T: Ord + Copy>(data: &[T]) -> Vec<T> {
    
    let n = data.len();
    let mut sorted = data.to_vec();
    
    for _ in 0..(n-1) {
        let (mut left, mut right) = (0, 1);
        while right < n {
            if sorted[left] > sorted[right] {
                sorted.swap(left,right);
            }
            left += 1;
            right += 1;
        }
    }
    sorted
}