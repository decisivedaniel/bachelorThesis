use std::{time::{Duration, Instant}};

use crate::bin_tree::BinTree;
//use crate::full_bin_tree::FullBinTree;

mod bin_tree;
//mod bin_tree_clone;
//mod full_bin_tree;

fn main() {
    let number_of_trials = 10;
    let counts = vec![1024,2048,4096,8192,16384];
    for upper in counts {
        print!("{} - ", upper);
        let optimal: (Vec<f64>, Vec<f64>) = test_loop(make_optimized_list, optimal_test_create_tree, optimal_test_read_tree, number_of_trials, upper);
        for value in optimal.0 {
            print!("{}\\\\", value);
        }
        println!("");
        for value in optimal.1 {
            print!("{}\\\\", value);
        }
        println!("");
        let worst = test_loop(make_worst_list, worst_case_create_tree, worst_case_read_tree, number_of_trials, upper);
        for value in worst.0 {
            print!("{}\\\\", value);
        }
        println!("");
        for value in worst.1 {
            print!("{}\\\\", value);
        }
        println!("");
    }
}

fn test_loop(make_list : fn(u64) -> Vec<u64>, create_test: fn(Vec<u64>) -> BinTree<u64>, read_test: fn(BinTree<u64>, u64), loops : u64, cap : u64) -> (Vec<f64>, Vec<f64>) {
    let mut create_total= Vec::<f64>::new();
    let mut read_total = Vec::<f64>::new();
    for _ in 0..loops {
        let round = test_timer(make_list, create_test, read_test, cap);
        create_total.push(round.0.as_secs_f64());
        read_total.push(round.1.as_secs_f64());
    }
    return (create_total, read_total);
}

fn test_timer(make_list : fn(u64) -> Vec<u64>, create_test: fn(Vec<u64>) -> BinTree<u64>, read_test: fn(BinTree<u64>, u64), cap : u64) -> (Duration, Duration) {
    let optimized_vec = make_list(cap);
    let first = Instant::now();
    let tree = create_test(optimized_vec);
    let create_result = first.elapsed();
    let second = Instant::now();
    read_test(tree, cap);
    let read_result = second.elapsed();
    return (create_result, read_result);
}

fn optimal_test_create_tree(values : Vec<u64>) -> BinTree<u64> {
    let mut tree = BinTree::<u64>::new();
    for i in values {
        tree.insert(i);
    }
    return tree;
}

fn optimal_test_read_tree(tree : BinTree<u64>, upper: u64) {
    for i in 0..upper {
        tree.contains(i);
    }
}

fn worst_case_create_tree(values : Vec<u64>) -> BinTree<u64> {
    let mut tree = BinTree::<u64>::new();
    for i in values {
        tree.insert(i);
    }
    return tree;
}

fn worst_case_read_tree(tree : BinTree<u64>, upper : u64) {
    for i in 0..upper {
        tree.contains(i);
    }
}

fn split_list(list : &mut Vec<u64>, lower : u64, upper : u64) {
    if upper == lower {
        return
    }
    let mid = ((upper - lower) / 2) + lower;
    list.push(mid);
    split_list(list, lower, mid);
    split_list(list, mid+1, upper);
}

fn make_optimized_list(max : u64) -> Vec<u64> {
    let mut optimized = Vec::<u64>::new();
    split_list(&mut optimized, 0, max);
    return optimized;
}

fn make_worst_list(max : u64) -> Vec<u64> {
    let mut worst = Vec::<u64>::new();
    for i in 0..max {
        worst.push(i);
    }
    return worst;
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn optimized () {
        let list = make_optimized_list(7);
        println!("{:?}",list);
    }
}
