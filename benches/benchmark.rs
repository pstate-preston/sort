use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

use Sort::merge_sort::merge_sort;
use Sort::heap_sort::heap_sort;
use Sort::selection_sort::selection_sort;
use Sort::bubble_sort::bubble_sort;

/// generates random vector of chosen size(range 0-1000) to pass as test.
/// # Parameter
/// * length - the desired length of the vector
/// # Returns
/// * returns a Vec<i32> of the specified length
fn random_vector(length: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut elements: Vec<i32> = vec![];
    for _ in 0..length {
        elements.push(rng.random_range(0..=1000));
    }
    elements
}

fn sort_benchmark(c: &mut Criterion){
    
    let mut vector = random_vector(10000);
    let mut test_inplace = vector.clone();
    let test: &[i32] = &mut vector;
    println!("\n\n\n\n");

    c.bench_function(
        "Merge Sort",
        |b|
            b.iter(|| merge_sort(black_box(test)))
    );

    c.bench_function(
        "Heap Sort",
        |b|
            b.iter(|| heap_sort(black_box(&mut test_inplace)))
    );

    c.bench_function(
        "Selection Sort",
        |b|
            b.iter(|| selection_sort(black_box(test)))
    );

    c.bench_function(
        "Bubble Sort",
        |b|
            b.iter(|| bubble_sort(black_box(test)))
    );
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);