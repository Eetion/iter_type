use itertools::Itertools;
use itertools::kmerge_by;

fn main() { 
    println!("Hello, world!");
    
    let a = vec![-1f64, 2., 3., -5., 6., -7.];
    let b = vec![0., 2., -4.];
    //let mut it = Itertools::kmerge_by( vec![a, b].into_iter(), |a: f64, b: f64| a.abs() < b.abs() );
    let mut it = vec![a, b].into_iter().kmerge_by(|a, b| a.abs() < b.abs());
    let mut it = kmerge_by(vec![a, b].into_iter(), |a, b| a.abs() < b.abs());
}
