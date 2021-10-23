# seal

[![Downloads](https://img.shields.io/crates/d/seal.svg?style=flat-square)](https://crates.io/crates/seal/)
[![Version](https://img.shields.io/crates/v/seal.svg?style=flat-square)](https://crates.io/crates/seal/)
[![License](https://img.shields.io/crates/l/seal.svg?style=flat-square)](https://crates.io/crates/seal/)

## Synopsis

A Rust implementation of Needleman-Wunsch & Smith-Waterman sequence alignment.

## Motivation

The aim of this crate is to provide a memory- and time-efficient implementation of Needleman-Wunsch as well as Smith-Waterman sequence alignment using a unified API.

## Getting Started

Add the most recent [version](https://crates.io/crates/seal) of `seal`
to your dependencies in your project's `Cargo.toml`.

Then add …

```rust
extern crate seal;
```

… to your crate's root file (e.g. `lib.rs`, `main.rs`).

Once that's done you're ready to play!

## Example

```rust
extern crate seal;

use seal::pair::{
    Alignment, AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman, Step,
};

fn main() {
    let str_x = "The quick brown fox jumps over the lazy dog.";
    let str_y = "The brown dog jumps over the very lazy snail.";

    let strategy = NeedlemanWunsch::new(1, -1, -1, -1);
    // Alternatively:
    // let strategy = SmithWaterman::new(2, -1, -1, -1);

    let sequence_x: Vec<char> = str_x.chars().collect();
    let sequence_y: Vec<char> = str_y.chars().collect();
    let set: AlignmentSet<InMemoryAlignmentMatrix> =
        AlignmentSet::new(sequence_x.len(), sequence_y.len(), strategy, |x, y| {
            sequence_x[x] == sequence_y[y]
        })
        .unwrap();

    let print_alignment = |alignment: Alignment| {
        for step in alignment.steps() {
            match step {
                Step::Align { x, y } => {
                    if sequence_x[x] == sequence_y[y] {
                        print!("=")
                    } else {
                        print!("!")
                    }
                }
                Step::Delete { .. } => print!("-"),
                Step::Insert { .. } => print!("+"),
            }
        }
        println!("\n");
    };

    println!("Local alignment:");
    let local_alignment = set.local_alignment();
    print_alignment(local_alignment);

    println!("Global alignment:");
    let global_alignment = set.global_alignment();
    print_alignment(global_alignment);

    // Local alignment:
    // ====------======!=!================+++++=====
    //
    // Global alignment:
    // ====------======!=!================+++++=====!!!++=
}
```

See the [examples directory](examples) for more in-depth examples.

## API Reference

An `AlignmentSet` contains all optimal alignments for a given pair of sequences.

### Retrieving a single locally/globally optimal alignment

```rust
let alignment in alignment_set.local_alignment();
let alignment in alignment_set.global_alignment();
```

### Enumerate all locally/globally optimal alignments

```rust
for alignment in alignment_set.local_alignments() {
    // …
}
for alignment in alignment_set.global_alignments() {
    // …
}
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our [code of conduct](https://www.rust-lang.org/conduct.html),
and the process for submitting pull requests to us.

## License

This project is licensed under the [**MPL-2.0**](https://www.tldrlegal.com/l/mpl-2.0) – see the [LICENSE.md](LICENSE.md) file for details.
