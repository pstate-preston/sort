use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

use Sort::merge_sort::merge_sort;
// use Sort::heap_sort::heap_sort;
use Sort::selection_sort::selection_sort;
use Sort::bubble_sort::bubble_sort;

// generates random vector of chosen size(range 0-1000) to pass as test.
fn random_vector(length: i32) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut elements: Vec<i32> = vec![];
    for _ in 0..length {
        elements.push(rng.random_range(0..=1000));
    }
    elements
}

// Problem: Heap sort is an in-place algorithm. Maybe need to adjust it to also take an array,
// then return a vector like the others? This may not be the best way to implement, but to
// compare the raw efficiency of heap and merge it may be useful.
fn sort_benchmark(c: &mut Criterion){
    
    let vector = random_vector(10000);
    let test: &[i32] = &vector;
    //println!("{:?}", test);
    println!("\n\n\n\n");

    //let test = [9, 5, 3, 9, 9, 8, 2, 4, 4, 4, 26, 29, 30, 43, 91, 10025];
    c.bench_function(
        "Merge Sort",
        |b|
            b.iter(|| merge_sort(black_box(test)))
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