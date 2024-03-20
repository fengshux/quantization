
use ndarray::prelude::*;
use polars::prelude::*;

fn main() {
    let arr1 = array![1., 2., 3., 4., 5., 6.];
    let arr2 = array![1., 2.2, 3.3, 4., 5., 6.];
    let arr3 = arr1 + arr2;
    println!("1D array: \t{}", arr3);

    let arr4 = array![[1., 2., 3.], [ 4., 5., 6.]];
    let arr5 = array![[1.], [2.]];//Array::from_elem((2, 1), 1.);
    let arr6 = arr4 + arr5;
    println!("2D array:\n{}", arr6);
    

    let df = df![
        "Model" => ["iPhone XS", "iPhone 12", "iPhone 13", "iPhone 14", "Samsung S11", "Samsung S12", "Mi A1", "Mi A2"],
        "Company" => ["Apple", "Apple", "Apple", "Apple", "Samsung", "Samsung", "Xiao Mi", "Xiao Mi"],
        "Sales" => [80, 170, 130, 205, 400, 30, 14, 8],
        "Comment" => [None, None, Some("Sold Out"), Some("New Arrival"), None, Some("Sold Out"), None, None],
    ];

    println!("{}", &df.unwrap()); 
}