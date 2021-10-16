extern crate seal;

use seal::pair::{
    strategy::discrete::{global::Strategy as GlobalStrategy, local::Strategy as LocalStrategy},
    Alignment, Alignments, Step, Strategy as StrategyTrait,
};

fn trace(_x: &[isize], _y: &[isize], alignment: &Alignment<isize>) {
    let mut x_vec: Vec<char> = vec![];
    let mut y_vec: Vec<char> = vec![];
    for step in alignment.steps() {
        match step {
            Step::Align { x: _x, y: _y } => {
                print!("=");
                x_vec.push('=');
                y_vec.push('=');
            }
            Step::Delete { x: _x } => {
                print!(">");
                x_vec.push('=');
                y_vec.push('-');
            }
            Step::Insert { y: _y } => {
                print!("<");
                x_vec.push('-');
                y_vec.push('=');
            }
        }
    }

    println!();

    let x_str: String = x_vec.into_iter().collect();
    let y_str: String = y_vec.into_iter().collect();

    for (x, y) in x_str.chars().zip(y_str.chars()) {
        if x == y {
            print!("=");
        } else {
            print!("|");
        }
    }
    println!();

    println!("{}", x_str);
    println!("{}", y_str);
}

fn align<T>(label: &str, x: &[isize], y: &[isize], strategy: T)
where
    T: StrategyTrait<isize, Score = isize>,
{
    fn cost_fn(x: &isize, y: &isize) -> isize {
        (x - y).abs() - 1
    }

    let alignments: Alignments<isize> = strategy.alignments(x, y, cost_fn);
    let distance = strategy.distance(x, y, cost_fn);

    println!("{:?}", alignments.matrix());
    if let Some(alignment) = alignments.alignment() {
        println!("{}:", label);
        println!("{:#?}", alignment);
        println!("{:#?}", distance);
        trace(x, y, &alignment);
    } else {
        println!("No alignment found.");
    }
}

fn main() {
    let seq_a = vec![0, 1, 5, 10, 10, 10, 5, 1, 0];
    let seq_b = vec![0, 10, 10, 10, 0];

    let global = GlobalStrategy::default();

    align("Global Alignment", &seq_a[..], &seq_b[..], global);

    println!();

    let local = LocalStrategy::default();

    align("Local Alignment", &seq_a[..], &seq_b[..], local);

    println!();
}
