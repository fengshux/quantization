
use ndarray::prelude::*;

fn main() {
    let arr1 = array![1., 2., 3., 4., 5., 6.];
    let arr2 = array![1., 2.2, 3.3, 4., 5., 6.];
    let arr3 = arr1 + arr2;
    println!("1D array: \t{}", arr3);

    let arr4 = array![[1., 2., 3.], [ 4., 5., 6.]];
    let arr5 = array![[1.], [2.]];//Array::from_elem((2, 1), 1.);
    let arr6 = arr4 + arr5;
    println!("2D array:\n{}", arr6);
}
