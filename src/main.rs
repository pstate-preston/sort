use Sort::merge_sort::merge_sort;
use Sort::heap_sort::heap_sort;
use Sort::selection_sort::selection_sort;
use Sort::bubble_sort::bubble_sort;

fn main() {
    let test = [17,15,26,13,9,6,5,5,5,5,10,4,8,3,1];
    let mut test_in_place = test.to_vec();
    
    let merge = merge_sort(&test);
    heap_sort(&mut test_in_place);
    let selection = selection_sort(&test);
    let bubble = bubble_sort(&test);
    
    println!("{:?} merge", merge);
    println!("{:?} heap", test_in_place);
    println!("{:?} selection", selection);
    println!("{:?} bubble", bubble);
}