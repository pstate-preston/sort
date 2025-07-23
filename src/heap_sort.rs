// auxiliary function to max heapify a node
fn heapify<T: Ord + Copy>(data: &mut Vec<T>, n: usize, index: usize){
    let (left_child, right_child)  = (2*index + 1, 2*index + 2);
    let mut max = index;

    if left_child < n && data[left_child] > data[max] {
        max = left_child;
    }

    if right_child < n && data[right_child] > data[max] {
        max = right_child;
    }

    if index != max {
        data.swap(index, max);
        heapify(data, n, max);
    }
}

/// Time complexity is O(n log(n)) in all cases. Sorts an array of numbers.
/// # Parameters
/// * data: &mut Vec<[T]> - a reference to vector containing numerical data.
/// # Returns
/// * Vec<[T]> - the original parameter array, sorted in place.
pub fn heap_sort<T: Ord + Copy>(data: &mut Vec<T>) {
    let n = data.len();
    
    // convert array to max heap
    for index in (0..n/2).rev() {
        heapify(data, n, index);
    }
    
    // For each n, swap the greatest element with the last at index, then heapify node zero.
    // This reimplements max heap and places the next element. 
    for i in (0..n).rev() {
        data.swap(0, i);
        heapify(data, i, 0);
    }
}