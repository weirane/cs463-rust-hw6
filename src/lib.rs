// Name:
// NetID:
// Resources and collaborators:

pub fn fib(n: u128) -> u128 {
    todo!()
}

pub fn reversed<T: Clone>(xs: &[T]) -> Vec<T> {
    todo!()
}

pub fn is_prime(n: i32) -> bool {
    todo!()
}

pub fn nub<T: PartialEq + Clone>(xs: &[T]) -> Vec<T> {
    todo!()
}

pub fn zip_with<A, B, C, F>(f: F, xs: &[A], ys: &[B]) -> Vec<C>
where
    F: FnMut(&A, &B) -> C,
{
    todo!()
}

pub fn collatz(n: i32) -> Vec<i32> {
    todo!()
}

pub fn median(xs: &[i32]) -> f64 {
    todo!()
}

pub fn mode(xs: &[i32]) -> Vec<i32> {
    todo!()
}

pub fn check_sudoku(grid: &[[u8; 9]; 9]) -> bool {
    todo!()
}

#[cfg(test)]
mod tests;
