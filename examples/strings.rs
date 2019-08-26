extern crate seal;

use seal::pair::{
    strategy::discrete::{global::Strategy as GlobalStrategy, local::Strategy as LocalStrategy},
    Alignment, Alignments, Step, Strategy as StrategyTrait,
};

fn trace(x_seq: &Vec<char>, y_seq: &Vec<char>, alignment: &Alignment<isize>) {
    let mut x_vec: Vec<char> = vec![];
    let mut y_vec: Vec<char> = vec![];
    for step in alignment.steps() {
        match step {
            Step::Align { x, y } => {
                print!("=");
                x_vec.push(x_seq[x]);
                y_vec.push(y_seq[y]);
            }
            Step::Delete { x } => {
                print!(">");
                x_vec.push(x_seq[x]);
                y_vec.push('-');
            }
            Step::Insert { y } => {
                print!("<");
                x_vec.push('-');
                y_vec.push(y_seq[y]);
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

fn align<T>(label: &str, str_x: &str, str_y: &str, strategy: T)
where
    T: StrategyTrait<char, Score = isize>,
{
    let sequence_x: Vec<char> = str_x.chars().collect();
    let sequence_y: Vec<char> = str_y.chars().collect();
    let alignment_set: Alignments<isize> = strategy
        .alignments(&sequence_x[..], &sequence_y[..], |x, y| {
            if x == y {
                -1
            } else {
                1
            }
        });

    println!("{:?}", alignment_set.matrix());
    if let Some(alignment) = alignment_set.alignment() {
        println!("{}:", label);
        println!("{:#?}", alignment);
        trace(&sequence_x, &sequence_y, &alignment);
    } else {
        println!("No alignment found.");
    }
}

fn main() {
    let seq_a = "ABCDEFG";
    let seq_b = "CD";

    println!("");

    let global = GlobalStrategy::default();

    println!("{}", seq_a);
    println!("{}", seq_b);

    align("Global Alignment", seq_a, seq_b, global);

    let local = LocalStrategy::default();

    align("Local Alignment", seq_a, seq_b, local);

    println!("");
}
