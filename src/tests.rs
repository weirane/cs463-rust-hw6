use super::*;

#[test]
fn fib_00() {
    assert_eq!(fib(0), 0, "fib #0: 0");
}

#[test]
fn fib_01() {
    assert_eq!(fib(1), 1, "fib #1: 1");
}

#[test]
fn fib_02() {
    assert_eq!(fib(2), 1, "fib #2: 2");
}

#[test]
fn fib_03() {
    assert_eq!(fib(3), 2, "fib #3: 3");
}

#[test]
fn fib_04() {
    assert_eq!(fib(4), 3, "fib #4: 4");
}

#[test]
fn fib_05() {
    assert_eq!(fib(5), 5, "fib #5: 5");
}

#[test]
fn fib_06() {
    assert_eq!(fib(6), 8, "fib #6: 6");
}

#[test]
fn fib_07() {
    assert_eq!(fib(7), 13, "fib #7: 7");
}

#[test]
fn fib_08() {
    assert_eq!(fib(20), 6765, "fib #8: 20");
}

#[test]
fn fib_09() {
    assert_eq!(fib(29), 514229, "fib #9: 29");
}

#[test]
fn fib_10() {
    assert_eq!(fib(43), 433494437, "fib #10: 43");
}

#[test]
fn fib_11() {
    assert_eq!(fib(55), 139583862445, "fib #11: 55");
}

#[test]
fn reversed_00() {
    assert_eq!(reversed::<i32>(&[]), Vec::<i32>::new(), "reversed #0: []");
}

#[test]
fn reversed_01() {
    assert_eq!(reversed(&[5]), vec![5], "reversed #1: [5]");
}

#[test]
fn reversed_02() {
    assert_eq!(reversed(&[10, 5]), vec![5, 10], "reversed #2: [10,5]");
}

#[test]
fn reversed_03() {
    assert_eq!(
        reversed(&[2, 4, 6, 8]),
        vec![8, 6, 4, 2],
        "reversed #3: [2,4,6,8]"
    );
}

#[test]
fn reversed_04() {
    assert_eq!(
        reversed(&[1, 1, 1, 2, 1, 1]),
        vec![1, 1, 2, 1, 1, 1],
        "reversed #4: [1,1,1,2,1,1]"
    );
}

#[test]
fn reversed_05() {
    assert_eq!(
        reversed(&[1, 3, 5, 2, 4, 6]),
        vec![6, 4, 2, 5, 3, 1],
        "reversed #5: [1,3,5,2,4,6]"
    );
}

#[test]
fn reversed_06() {
    assert_eq!(
        reversed(&[5, 5, 5, 5, 5]),
        vec![5, 5, 5, 5, 5],
        "reversed #6: [5,5,5,5,5]"
    );
}

#[test]
fn reversed_07() {
    assert_eq!(
        reversed(&[true, false, false]),
        vec![false, false, true],
        "reversed #7: [True, False, False]"
    );
}

#[test]
fn reversed_08() {
    assert_eq!(
        reversed(&['a', 'b', 'c', 'd']),
        vec!['d', 'c', 'b', 'a'],
        "reversed #8: \"abcd\""
    );
}

#[test]
fn reversed_09() {
    assert_eq!(
        reversed(&(1..=1000).collect::<Vec<i32>>()),
        (1..=1000).rev().collect::<Vec<i32>>(),
        "reversed #9: [1..1000]"
    );
}

#[test]
fn reversed_10() {
    assert_eq!(
        reversed(&[vec![1, 2, 3], vec![4, 5], vec![6, 7, 8]]),
        vec![vec![6, 7, 8], vec![4, 5], vec![1, 2, 3]],
        "reversed #10: [[1,2,3],[4,5],[6,7,8]]"
    );
}

#[test]
fn reversed_11() {
    assert_eq!(
        reversed(&['l', 'o', 'l', 'o', 'l']),
        vec!['l', 'o', 'l', 'o', 'l'],
        "reversed #11: \"lolol\""
    );
}

#[test]
fn reversed_12() {
    assert_eq!(
        reversed(&[(1, 2), (3, 4), (5, 6)]),
        vec![(5, 6), (3, 4), (1, 2)],
        "reversed #12: [(1,2),(3,4),(5,6)]"
    );
}

#[test]
fn is_prime_00() {
    assert_eq!(is_prime(-5), false, "isPrime #0:  -5");
}

#[test]
fn is_prime_01() {
    assert_eq!(is_prime(1), false, "isPrime #1:  1");
}

#[test]
fn is_prime_02() {
    assert_eq!(is_prime(2), true, "isPrime #2:  2");
}

#[test]
fn is_prime_03() {
    assert_eq!(is_prime(3), true, "isPrime #3:  3");
}

#[test]
fn is_prime_04() {
    assert_eq!(is_prime(4), false, "isPrime #4:  4");
}

#[test]
fn is_prime_05() {
    assert_eq!(is_prime(5), true, "isPrime #5:  5");
}

#[test]
fn is_prime_06() {
    assert_eq!(is_prime(39), false, "isPrime #6:  39");
}

#[test]
fn is_prime_07() {
    assert_eq!(is_prime(41), true, "isPrime #7:  41");
}

#[test]
fn is_prime_08() {
    assert_eq!(is_prime(117), false, "isPrime #8:  117");
}

#[test]
fn is_prime_09() {
    assert_eq!(is_prime(1117), true, "isPrime #9: 1117");
}

#[test]
fn is_prime_10() {
    assert_eq!(is_prime(111117), false, "isPrime #10: 111117");
}

#[test]
fn is_prime_11() {
    assert_eq!(is_prime(104729), true, "isPrime #11: large prime");
}

#[test]
fn is_prime_12() {
    assert_eq!(is_prime(1488391), false, "isPrime #12: large non-prime");
}

#[test]
fn nub_00() {
    assert_eq!(nub::<i32>(&[]), Vec::<i32>::new(), "nub #0: []");
}

#[test]
fn nub_01() {
    assert_eq!(nub(&[5]), vec![5], "nub #1: [5]");
}

#[test]
fn nub_02() {
    assert_eq!(nub(&[13, 13, 13]), vec![13], "nub #2: [13,13,13]");
}

#[test]
fn nub_03() {
    assert_eq!(
        nub(&[1, 2, 3, 1, 2, 3, 1, 2, 3]),
        vec![1, 2, 3],
        "nub #3: [1,2,3,1,2,3,1,2,3]"
    );
}

#[test]
fn nub_04() {
    assert_eq!(
        nub(&[1, 1, 3, 2, 2, 5, 5, 5, 5, 4]),
        vec![1, 3, 2, 5, 4],
        "nub #4: [1,1,3,2,2,5,5,5,5,4]"
    );
}

#[test]
fn nub_05() {
    assert_eq!(
        nub(&[1, 4, 2, 5, 3, 1, 2, 3, 4, 5, 6, 1, 3, 2]),
        vec![1, 4, 2, 5, 3, 6],
        "nub #5: [1,4,2,5,3,1,2,3,4,5,6,1,3,2]"
    );
}

#[test]
fn nub_06() {
    assert_eq!(
        nub(&[1, 2, 3, 4, 5]),
        vec![1, 2, 3, 4, 5],
        "nub #6: [1,2,3,4,5]"
    );
}

#[test]
fn nub_07() {
    assert_eq!(
        nub(&[-3, -1, 1, 3, 2, 4, -2, -4]),
        vec![-3, -1, 1, 3, 2, 4, -2, -4],
        "nub #7: [-3,-1,1,3,2,4,-2,-4]"
    );
}

#[test]
fn nub_08() {
    assert_eq!(
        nub(&[1, 1, 5, 1, 10, 1, 1, 15, 1, 1]),
        vec![1, 5, 10, 15],
        "nub #8: [1,1,5,1,10,1,1,15,1,1]"
    );
}

#[test]
fn nub_09() {
    assert_eq!(nub(&[1, 2, 3, 2]), vec![1, 2, 3], "nub #9: [1,2,3,2]");
}

#[test]
fn nub_10() {
    assert_eq!(nub(&[1, 2, 3, 3]), vec![1, 2, 3], "nub #10: [1,2,3,3]");
}

#[test]
fn nub_11() {
    assert_eq!(
        nub(&['l', 'o', 'l', 'o', 'l']),
        vec!['l', 'o'],
        "nub #11: \"lolol\""
    );
}

#[test]
fn nub_12() {
    assert_eq!(
        nub(&[(1, 1), (2, 3), (1, 1), (2, 4), (2, 1), (1, 1), (5, 4)]),
        vec![(1, 1), (2, 3), (2, 4), (2, 1), (5, 4)],
        "nub #12: [(1,1),(2,3),(1,1),(2,4),(2,1),(1,1),(5,4)]"
    );
}

#[test]
fn zip_with_00() {
    assert_eq!(
        zip_with(|x, y| x + y, &[1, 2, 3, 4], &[10, 10, 10, 10]),
        vec![11, 12, 13, 14],
        "zipWith #0:   (+) [1,2,3,4] [10,10,10,10]"
    );
}

#[test]
fn zip_with_01() {
    assert_eq!(
        zip_with(|x, y| x + y, &[1, 2, 3, 4], &[5, 6, 7, 8]),
        vec![6, 8, 10, 12],
        "zipWith #1:   (+) [1,2,3,4] [5,6,7,8]"
    );
}

#[test]
fn zip_with_02() {
    assert_eq!(
        zip_with(|x, y| x * y, &[2, 3, 4], &[5, 5, 5, 5]),
        vec![10, 15, 20],
        "zipWith #2:   (*) [2,3,4] [5,5,5,5,5]"
    );
}

#[test]
fn zip_with_03() {
    assert_eq!(
        zip_with(|x, y| x * y, &[2, 3, 4, 5, 6, 7, 8], &[5, 5, 5]),
        vec![10, 15, 20],
        "zipWith #3:   (*) [2,3,4,5,6,7,8] [5,5,5]"
    );
}

#[test]
fn zip_with_04() {
    assert_eq!(
        zip_with(|x: &i32, y: &i32| x * y, &[1, 2, 3, 4, 5], &[]),
        Vec::<i32>::new(),
        "zipWith #4:   (*) [1,2,3,4,5] []"
    );
}

#[test]
fn zip_with_05() {
    assert_eq!(
        zip_with(|x: &i32, y: &i32| x * y, &[], &[1, 2, 3, 4, 5]),
        Vec::<i32>::new(),
        "zipWith #5:   (*) [] [1,2,3,4,5]"
    );
}

#[test]
fn zip_with_06() {
    assert_eq!(
        zip_with(|x: &i32, y: &i32| x * y, &[], &[]),
        Vec::<i32>::new(),
        "zipWith #6:   (*) [] []"
    );
}

#[test]
fn zip_with_07() {
    assert_eq!(
        zip_with(|x, y| *x.max(y), &[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]),
        vec![5, 4, 3, 4, 5],
        "zipWith #7:   max [1,2,3,4,5] [5,4,3,2,1]"
    );
}

#[test]
fn zip_with_08() {
    assert_eq!(
        zip_with(|x: &i32, y: &u32| x.pow(*y), &[2, 2, 2], &[4, 5, 6]),
        vec![16, 32, 64],
        "zipWith #8:   (^) [2,2,2] [4,5,6]"
    );
}

#[test]
fn zip_with_09() {
    assert_eq!(
        zip_with(
            |x, y| if x % 2 == 0 { *x } else { *y },
            &[1, 2, 3, 4, 5],
            &[6, 7, 8, 9, 10]
        ),
        vec![6, 2, 8, 4, 10],
        "zipWith #9:   (\\x y-> if even x then x else y) [1,2,3,4,5] [6,7,8,9,10]"
    );
}

#[test]
fn zip_with_10() {
    assert_eq!(
        zip_with(|x, y| format!("{x}{y}"), &["a", "b", "c"], &["x", "y", "z"]),
        vec!["ax", "by", "cz"],
        "zipWith #10:  (++) [\"a\",\"b\",\"c\"] [\"x\",\"y\",\"z\"]"
    );
}

#[test]
fn zip_with_11() {
    assert_eq!(
        zip_with(
            |x, ys: &Vec<i32>| {
                let mut result = vec![*x];
                result.extend_from_slice(ys);
                result
            },
            &[1, 2, 3],
            &[vec![10], vec![20, 20], vec![30, 30, 30]]
        ),
        vec![vec![1, 10], vec![2, 20, 20], vec![3, 30, 30, 30]],
        "zipWith #11:  (:) [1,2,3] [[10],[20,20],[30,30,30]]"
    );
}

#[test]
fn zip_with_12() {
    assert_eq!(
        zip_with(
            |x, y| *x && *y,
            &[false, false, true, true],
            &[false, true, false, true]
        ),
        vec![false, false, false, true],
        "zipWith #12:  (&&) [False, False, True, True] [False, True, False, True]"
    );
}

#[test]
fn zip_with_13() {
    assert_eq!(
        zip_with(
            |x, y| *x || *y,
            &[false, false, true, true, false],
            &[false, true, false, true]
        ),
        vec![false, true, true, true],
        "zipWith #13:  (||) [False, False, True, True, False] [False, True, False, True]"
    );
}

#[test]
fn collatz_00() {
    assert_eq!(collatz(1), vec![1], "collatz #0: 1");
}

#[test]
fn collatz_01() {
    assert_eq!(collatz(2), vec![2, 1], "collatz #1: 2");
}

#[test]
fn collatz_02() {
    assert_eq!(collatz(3), vec![3, 10, 5, 16, 8, 4, 2, 1], "collatz #2: 3");
}

#[test]
fn collatz_03() {
    assert_eq!(collatz(4), vec![4, 2, 1], "collatz #3: 4");
}

#[test]
fn collatz_04() {
    assert_eq!(collatz(5), vec![5, 16, 8, 4, 2, 1], "collatz #4: 5");
}

#[test]
fn collatz_05() {
    assert_eq!(
        collatz(6),
        vec![6, 3, 10, 5, 16, 8, 4, 2, 1],
        "collatz #5: 6"
    );
}

#[test]
fn collatz_06() {
    assert_eq!(collatz(10), vec![10, 5, 16, 8, 4, 2, 1], "collatz #6: 10");
}

#[test]
fn collatz_07() {
    assert_eq!(
        collatz(11),
        vec![11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1],
        "collatz #7: 11"
    );
}

#[test]
fn collatz_08() {
    assert_eq!(
        collatz(17),
        vec![17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1],
        "collatz #8: 17"
    );
}

#[test]
fn collatz_09() {
    assert_eq!(
        collatz(42),
        vec![42, 21, 64, 32, 16, 8, 4, 2, 1],
        "collatz #9: 42"
    );
}

#[test]
fn collatz_10() {
    assert_eq!(
        collatz(99),
        vec![
            99, 298, 149, 448, 224, 112, 56, 28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5,
            16, 8, 4, 2, 1
        ],
        "collatz #10: 99"
    );
}

#[test]
fn collatz_11() {
    assert_eq!(
        collatz(100),
        vec![
            100, 50, 25, 76, 38, 19, 58, 29, 88, 44, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16,
            8, 4, 2, 1
        ],
        "collatz #11: 100"
    );
}

#[test]
fn collatz_12() {
    assert_eq!(
        collatz(8192),
        vec![
            8192, 4096, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1
        ],
        "collatz #12: 8192"
    );
}

#[test]
fn median_00() {
    assert_eq!(median(&[1, 1, 1]), 1.0, "median #0: [1,1,1]");
}

#[test]
fn median_01() {
    assert_eq!(median(&[1, 2, 3]), 2.0, "median #1: [1,2,3]");
}

#[test]
fn median_02() {
    assert_eq!(median(&[1, 2, 2, 3]), 2.0, "median #2: [1,2,2,3]");
}

#[test]
fn median_03() {
    assert_eq!(median(&[1, 2, 3, 4]), 2.5, "median #3: [1,2,3,4]");
}

#[test]
fn median_04() {
    assert_eq!(median(&[1, 3, 2, 3, 1]), 2.0, "median #4: [1,3,2,3,1]");
}

#[test]
fn median_05() {
    assert_eq!(median(&[5, 5, 5, 25]), 5.0, "median #5: [5,5,5,25]");
}

#[test]
fn median_06() {
    assert_eq!(
        median(&[13, 6, 13, 3, 7, 29, 12, 1, 2, 14]),
        9.5,
        "median #6: [13,6,13,3,7,29,12,1,2,14]"
    );
}

#[test]
fn median_07() {
    assert_eq!(median(&[2, 4, 7, 8]), 5.5, "median #7: [2,4,7,8]");
}

#[test]
fn mode_00() {
    assert_eq!(mode(&[1, 1, 1]), vec![1], "mode #0: [1,1,1]");
}

#[test]
fn mode_01() {
    assert_eq!(mode(&[1, 2, 3]), vec![1, 2, 3], "mode #1: [1,2,3]");
}

#[test]
fn mode_02() {
    assert_eq!(mode(&[1, 2, 2, 3]), vec![2], "mode #2: [1,2,2,3]");
}

#[test]
fn mode_03() {
    assert_eq!(mode(&[1, 2, 3, 4]), vec![1, 2, 3, 4], "mode #3: [1,2,3,4]");
}

#[test]
fn mode_04() {
    assert_eq!(mode(&[1, 3, 2, 3, 1]), vec![1, 3], "mode #4: [1,3,2,3,1]");
}

#[test]
fn mode_05() {
    assert_eq!(mode(&[5, 5, 5, 25]), vec![5], "mode #5: [5,5,5,25]");
}

#[test]
fn mode_06() {
    assert_eq!(
        mode(&[13, 6, 13, 3, 7, 29, 12, 1, 2, 14]),
        vec![13],
        "mode #6: [13,6,13,3,7,29,12,1,2,14]"
    );
}

#[test]
fn mode_07() {
    assert_eq!(
        mode(&[5, 5, 5, 1, 1, 5, 1, 1, 1, 5, 5]),
        vec![5],
        "mode #7: [5,5,5,1,1,5,1,1,1,5,5]"
    );
}

#[test]
fn sudoku_00() {
    assert_eq!(
        check_sudoku(&GRID_GOOD1),
        true,
        "checkSudoku #0: grid_good1"
    );
}

#[test]
fn sudoku_01() {
    assert_eq!(
        check_sudoku(&GRID_ROWS_BAD),
        false,
        "checkSudoku #1: grid_rows_bad"
    );
}

#[test]
fn sudoku_02() {
    assert_eq!(
        check_sudoku(&GRID_COLS_BAD),
        false,
        "checkSudoku #2: grid_cols_bad"
    );
}

#[test]
fn sudoku_03() {
    assert_eq!(
        check_sudoku(&GRID_GRPS_BAD),
        false,
        "checkSudoku #3: grid_grps_bad"
    );
}

#[test]
fn sudoku_04() {
    assert_eq!(
        check_sudoku(&GRID_BAD_5),
        false,
        "checkSudoku #4: grid_bad_5"
    );
}

#[test]
fn sudoku_05() {
    assert_eq!(
        check_sudoku(&GRID_BAD_ONES),
        false,
        "checkSudoku #5: grid_bad_ones"
    );
}

#[test]
fn sudoku_06() {
    assert_eq!(
        check_sudoku(&GRID_GOOD2),
        true,
        "checkSudoku #6: grid_good2"
    );
}

#[test]
fn sudoku_07() {
    assert_eq!(
        check_sudoku(&GRID_GOOD3),
        true,
        "checkSudoku #7: grid_good3"
    );
}

#[test]
fn sudoku_08() {
    assert_eq!(
        check_sudoku(&GRID_GOOD2),
        true,
        "checkSudoku #8: grid_good2"
    );
}

#[test]
fn sudoku_09() {
    assert_eq!(
        check_sudoku(&GRID_GOOD3),
        true,
        "checkSudoku #9: grid_good3"
    );
}

#[test]
fn sudoku_10() {
    assert_eq!(
        check_sudoku(&GRID_BAD_6),
        false,
        "checkSudoku #10: grid_bad_6"
    );
}

const GRID_GOOD1: [[i32; 9]; 9] = [
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

const GRID_ROWS_BAD: [[i32; 9]; 9] = [
    [1, 2, 3, 1, 2, 3, 1, 2, 3],
    [4, 5, 6, 4, 5, 6, 4, 5, 6],
    [7, 8, 9, 7, 8, 9, 7, 8, 9],
    [2, 3, 4, 2, 3, 4, 2, 3, 4],
    [5, 6, 7, 5, 6, 7, 5, 6, 7],
    [8, 9, 1, 8, 9, 1, 8, 9, 1],
    [3, 4, 5, 3, 4, 5, 3, 4, 5],
    [6, 7, 8, 6, 7, 8, 6, 7, 8],
    [9, 1, 2, 9, 1, 2, 9, 1, 2],
];

const GRID_COLS_BAD: [[i32; 9]; 9] = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
];

const GRID_GRPS_BAD: [[i32; 9]; 9] = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [2, 3, 4, 5, 6, 7, 8, 9, 1],
    [3, 4, 5, 6, 7, 8, 9, 1, 2],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [5, 6, 7, 8, 9, 1, 2, 3, 4],
    [6, 7, 8, 9, 1, 2, 3, 4, 5],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
    [8, 9, 1, 2, 3, 4, 5, 6, 7],
    [9, 1, 2, 3, 4, 5, 6, 7, 8],
];

const GRID_BAD_5: [[i32; 9]; 9] = [
    [1, 2, 3, 4, 5, 6, 7, 8, 9],
    [4, 5, 6, 7, 8, 9, 1, 2, 3],
    [7, 8, 9, 1, 2, 3, 4, 5, 6],
    [2, 3, 4, 5, 6, 7, 8, 9, 1],
    [5, 6, 7, 8, 5, 1, 2, 3, 4],
    [8, 9, 1, 2, 3, 4, 5, 6, 7],
    [3, 4, 5, 6, 7, 8, 9, 1, 2],
    [6, 7, 8, 9, 1, 2, 3, 4, 5],
    [9, 1, 2, 3, 4, 5, 6, 7, 8],
];

const GRID_BAD_ONES: [[i32; 9]; 9] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 2, 3, 1, 1, 1],
    [1, 1, 1, 4, 5, 6, 1, 1, 1],
    [1, 1, 1, 7, 8, 9, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const GRID_GOOD2: [[i32; 9]; 9] = [
    [2, 7, 8, 4, 6, 9, 1, 5, 3],
    [6, 9, 3, 1, 2, 5, 4, 7, 8],
    [4, 5, 1, 7, 8, 3, 2, 6, 9],
    [7, 6, 5, 8, 9, 4, 3, 2, 1],
    [3, 4, 2, 6, 1, 7, 9, 8, 5],
    [8, 1, 9, 3, 5, 2, 6, 4, 7],
    [9, 8, 4, 5, 3, 6, 7, 1, 2],
    [5, 2, 6, 9, 7, 1, 8, 3, 4],
    [1, 3, 7, 2, 4, 8, 5, 9, 6],
];

const GRID_GOOD3: [[i32; 9]; 9] = [
    [7, 8, 6, 4, 9, 3, 2, 5, 1],
    [3, 2, 1, 8, 7, 5, 4, 9, 6],
    [9, 4, 5, 6, 2, 1, 8, 7, 3],
    [1, 5, 7, 9, 3, 8, 6, 2, 4],
    [2, 9, 8, 5, 4, 6, 3, 1, 7],
    [4, 6, 3, 7, 1, 2, 9, 8, 5],
    [8, 1, 2, 3, 6, 7, 5, 4, 9],
    [6, 7, 4, 2, 5, 9, 1, 3, 8],
    [5, 3, 9, 1, 8, 4, 7, 6, 2],
];

const GRID_BAD_6: [[i32; 9]; 9] = [
    [1, 1, 1, 2, 2, 2, 3, 3, 3],
    [1, 1, 1, 2, 2, 2, 3, 3, 3],
    [1, 1, 1, 2, 2, 2, 3, 3, 3],
    [4, 4, 4, 5, 5, 5, 6, 6, 6],
    [4, 4, 4, 5, 5, 5, 6, 6, 6],
    [4, 4, 4, 5, 5, 5, 6, 6, 6],
    [7, 7, 7, 8, 8, 8, 9, 9, 9],
    [7, 7, 7, 8, 8, 8, 9, 9, 9],
    [7, 7, 7, 8, 8, 8, 9, 9, 9],
];
