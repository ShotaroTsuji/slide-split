use sliding_windows::*;

fn compare_slice_and_iter(width: usize, stride: usize) {
    let a: Vec<usize> = (0..1000).collect();

    let b = sliding_windows_from_slice(&a, width, stride)
        .map(|w| w.iter().cloned().collect())
        .collect::<Vec<Vec<usize>>>();
    let c = sliding_windows_from_iter(a.into_iter(), width, stride)
        .collect::<Vec<Vec<usize>>>();

    if b != c {
        eprintln!("# slice");
        for w in b.iter() {
            eprintln!("{:?}", w);
        }
        eprintln!("# iter");
        for w in c.iter() {
            eprintln!("{:?}", w);
        }
        panic!("The sliding windows from the slice and that from the iterator are different");
    }
}

fn compare_slice_and_iter_exact(width: usize, stride: usize) {
    let a: Vec<usize> = (0..1000).collect();

    let b = exact_sliding_windows_from_slice(&a, width, stride)
        .map(|w| w.iter().cloned().collect())
        .collect::<Vec<Vec<usize>>>();
    let c = exact_sliding_windows_from_iter(a.into_iter(), width, stride)
        .collect::<Vec<Vec<usize>>>();

    if b != c {
        eprintln!("# slice");
        for w in b.iter() {
            eprintln!("{:?}", w);
        }
        eprintln!("# iter");
        for w in c.iter() {
            eprintln!("{:?}", w);
        }
        panic!("The sliding windows from the slice and that from the iterator are different");
    }
}

#[test]
fn standard() {
    for width in 1..1001 {
        for stride in 1..1001 {
            compare_slice_and_iter(width, stride);
        }
    }
}

#[test]
fn exact() {
    for width in 1..1001 {
        for stride in 1..1001 {
            compare_slice_and_iter_exact(width, stride);
        }
    }
}
