use super::*;

#[test]
fn fib_cases() {
    let cases = [
        (0, 0),
        (1, 1),
        (2, 1),
        (10, 55),
        (45, 1_134_903_170),
        (100, 354_224_848_179_261_915_075),
        (186, 332_825_110_087_067_562_321_196_029_789_634_457_848),
    ];
    for (n, expected) in cases {
        assert_eq!(fib(n), expected, "fib({n})");
    }
}

#[test]
fn reversed_cases() {
    assert_eq!(reversed::<i32>(&[]), Vec::<i32>::new());
    let xs = [1, 2, 3, 2];
    assert_eq!(reversed(&xs), vec![2, 3, 2, 1]);
    assert_eq!(xs, [1, 2, 3, 2]);
    assert_eq!(reversed(&["red", "blue"]), vec!["blue", "red"]);
}

#[test]
fn is_prime_cases() {
    for n in [-5, 0, 1, 4, 39, 117, 1001] {
        assert!(!is_prime(n), "{n} is not prime");
    }
    for n in [2, 3, 5, 41, 1009, 1117] {
        assert!(is_prime(n), "{n} is prime");
    }
}

#[test]
fn nub_cases() {
    assert_eq!(nub::<i32>(&[]), Vec::<i32>::new());
    let xs = [3, 2, 1, 2, 3, 4, 1];
    assert_eq!(nub(&xs), vec![3, 2, 1, 4]);
    assert_eq!(xs, [3, 2, 1, 2, 3, 4, 1]);
    assert_eq!(nub(&["a", "b", "a"]), vec!["a", "b"]);
}

#[test]
fn zip_with_cases() {
    assert_eq!(
        zip_with(|x, y| x + y, &[1, 2, 3], &[10, 20, 30]),
        vec![11, 22, 33]
    );
    assert_eq!(zip_with(|x, y| x * y, &[2, 3, 4], &[5, 6]), vec![10, 18]);
    assert_eq!(
        zip_with(|x, y| format!("{x}{y}"), &["a", "b"], &[1, 2, 3]),
        vec!["a1", "b2"]
    );
    assert_eq!(
        zip_with(|x: &i32, y: &i32| x + y, &[], &[1]),
        Vec::<i32>::new()
    );
    let mut calls = 0;
    assert_eq!(
        zip_with(
            |x, y| {
                calls += 1;
                x + y
            },
            &[1, 2],
            &[3, 4]
        ),
        vec![4, 6]
    );
    assert_eq!(calls, 2);
}

#[test]
fn collatz_cases() {
    assert_eq!(collatz(1), vec![1]);
    assert_eq!(collatz(2), vec![2, 1]);
    assert_eq!(collatz(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
    assert_eq!(
        collatz(17),
        vec![17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
    );
}

#[test]
fn median_cases() {
    assert_eq!(median(&[3, 4, 5, 1, 2]), 3.0);
    assert_eq!(median(&[1, 3, 6, 8]), 4.5);
    assert_eq!(median(&[-5, -1]), -3.0);
    let xs = [9, 1, 5];
    assert_eq!(median(&xs), 5.0);
    assert_eq!(xs, [9, 1, 5]);
}

#[test]
fn mode_cases() {
    assert_eq!(mode(&[1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(mode(&[5, 5, 5, 25]), vec![5]);
    let xs = [3, 1, 2, 3, 1];
    assert_eq!(mode(&xs), vec![1, 3]);
    assert_eq!(xs, [3, 1, 2, 3, 1]);
}

#[test]
fn sudoku_cases() {
    let good = [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [4, 5, 6, 7, 8, 9, 1, 2, 3],
        [7, 8, 9, 1, 2, 3, 4, 5, 6],
        [2, 3, 4, 5, 6, 7, 8, 9, 1],
        [5, 6, 7, 8, 9, 1, 2, 3, 4],
        [8, 9, 1, 2, 3, 4, 5, 6, 7],
        [3, 4, 5, 6, 7, 8, 9, 1, 2],
        [6, 7, 8, 9, 1, 2, 3, 4, 5],
        [9, 1, 2, 3, 4, 5, 6, 7, 8],
    ];
    assert!(check_sudoku(&good));

    let mut bad_row = good;
    bad_row[0][0] = bad_row[0][1];
    assert!(!check_sudoku(&bad_row));

    let mut bad_column = good;
    bad_column[3] = good[0];
    assert!(!check_sudoku(&bad_column));

    let mut bad_box = good;
    for row in &mut bad_box {
        row.swap(0, 4);
    }
    assert!(!check_sudoku(&bad_box));

    let mut bad_digit = good;
    bad_digit[0][0] = 0;
    assert!(!check_sudoku(&bad_digit));
    assert_eq!(good[0][0], 1);
}
