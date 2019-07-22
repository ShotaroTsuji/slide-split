use sliding_windows::*;

fn main() {
    let a = (0..30).collect::<Vec<usize>>();

    for w in sliding_windows_from_slice(&a, 3, 2) {
        println!("{:?}", w);
    }
    println!("");
    for w in sliding_windows_from_iter(a.iter(), 3, 2) {
        println!("{:?}", w);
    }
    println!("");
    println!("");
    for w in sliding_windows_from_slice(&a, 2, 4) {
        println!("{:?}", w);
    }
    println!("");
    for w in sliding_windows_from_iter(a.iter(), 2, 4) {
        println!("{:?}", w);
    }
    println!("");
    println!("");
    for w in sliding_windows_from_slice(&a, 5, 5) {
        println!("{:?}", w);
    }
    println!("");
    for w in sliding_windows_from_iter(a.iter(), 5, 5) {
        println!("{:?}", w);
    }
    println!("");
    println!("");
    for w in sliding_windows_from_slice(&a, 1, 10) {
        println!("{:?}", w);
    }
    println!("");
    for w in sliding_windows_from_iter(a.iter(), 1, 10) {
        println!("{:?}", w);
    }
    println!("");
    println!("");
    for w in sliding_windows_from_slice(&a, 10, 1) {
        println!("{:?}", w);
    }
    println!("");
    for w in sliding_windows_from_iter(a.iter(), 10, 1) {
        println!("{:?}", w);
    }
    println!("");
    for w in exact_sliding_windows_from_slice(&a, 10, 1) {
        println!("{:?}", w);
    }
    println!("");
    for w in exact_sliding_windows_from_iter(a.iter(), 10, 1) {
        println!("{:?}", w);
    }
}
