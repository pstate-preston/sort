use sort::merge_sort::merge_sort;
use sort::quick_sort::quick_sort;
use sort::heap_sort::heap_sort;
use sort::selection_sort::selection_sort;
use sort::bubble_sort::bubble_sort;

fn main() {
    let test = [17,15,26,13,9,6,5,5,5,5,10,4,8,3,1];
    let mut heap_inplace = test.to_vec();
    let mut quick_inplace = heap_inplace.clone();

    let merge = merge_sort(&test);
    heap_sort(&mut heap_inplace);
    quick_sort(&mut quick_inplace);
    let selection = selection_sort(&test);
    let bubble = bubble_sort(&test);
    
    println!("{:?} merge sort", merge);
    println!("{:?} quick sort", quick_inplace);
    println!("{:?} heap sort", heap_inplace);
    println!("{:?} selection sort", selection);
    println!("{:?} bubble sort", bubble);
}