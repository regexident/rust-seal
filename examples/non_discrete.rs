extern crate seal;

use seal::pair::strategy::non_discrete::Strategy;
use seal::pair::InMemoryAlignmentMatrix;
use seal::pair::Step;
use seal::pair::Strategy as StrategyTrait;
use seal::pair::{Alignment, AlignmentScope, AlignmentSet};

fn trace(_x_seq: &Vec<f64>, _y_seq: &Vec<f64>, alignment: &Alignment<f64>) {
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

    print!("\n");

    let x_str: String = x_vec.into_iter().collect();
    let y_str: String = y_vec.into_iter().collect();

    for (x, y) in x_str.chars().zip(y_str.chars()) {
        if x == y {
            print!("=");
        } else {
            print!("|");
        }
    }
    print!("\n");

    println!("{}", x_str);
    println!("{}", y_str);
}

fn align(label: &str, seq_x: &[f64], seq_y: &[f64], strategy: Strategy) {
    let sequence_x: Vec<_> = seq_x.to_owned();
    let sequence_y: Vec<_> = seq_y.to_owned();
    let alignment_set: Result<AlignmentSet<f64, InMemoryAlignmentMatrix<f64>>, _> =
        strategy.alignment_set(&sequence_x[..], &sequence_y[..]);

    match alignment_set {
        Ok(alignment_set) => {
            println!("{:?}", alignment_set.matrix());
            let alignment = alignment_set.alignment(AlignmentScope::Global);
            println!("Alignment: {:#?}", alignment);
            println!(
                "Alignment: {:?} (score: {}, origin: {:?})",
                label,
                alignment.score(),
                alignment.origin()
            );
            trace(&sequence_x, &sequence_y, &alignment);
            println!("\n--------------------------\n");
        }
        Err(error) => {
            println!("Failed to generate alignment set due to error:");
            println!("{:?}", error);
        }
    }
}

fn main() {
    let seq_a = vec![0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 5.0];
    let seq_b = vec![0.0, 1.0, 2.0, 3.0, 5.0, 5.0, 5.0, 6.0];

    let dtw = Strategy::dynamic_time_warping();

    align("Dynamic Time Warping", &seq_a[..], &seq_b[..], dtw.clone());

    println!("");
}
