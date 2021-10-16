extern crate rand;

use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum Protein {
    A = 65, // ASCII 'A'
    C = 67, // ASCII 'C'
    G = 71, // ASCII 'G'
    T = 84, // ASCII 'T'
}

impl Into<u8> for Protein {
    fn into(self) -> u8 {
        self as u8
    }
}

use Protein::{A, C, G, T};

#[derive(Debug)]
pub struct MarkovModel {
    pub init: [(Protein, f32); 4],
    pub at_a: [(Protein, f32); 4],
    pub at_c: [(Protein, f32); 4],
    pub at_g: [(Protein, f32); 4],
    pub at_t: [(Protein, f32); 4],
}

#[derive(Debug)]
pub struct DnaGenerator {
    model: MarkovModel,
}

impl DnaGenerator {
    pub fn new(model: MarkovModel) -> Self {
        DnaGenerator { model: model }
    }

    pub fn generate(&self, len: usize) -> Vec<Protein> {
        fn as_weighted(proteins: &[(Protein, f32)]) -> Vec<Weighted<Protein>> {
            proteins.iter()
                .cloned()
                .map(|(item, weight)| {
                    Weighted {
                        weight: (((u32::max_value() - 1000) as f32) * weight) as u32,
                        item: item,
                    }
                })
                .collect()
        }

        let mut init = as_weighted(&self.model.init);
        let mut at_a = as_weighted(&self.model.at_a);
        let mut at_c = as_weighted(&self.model.at_c);
        let mut at_g = as_weighted(&self.model.at_g);
        let mut at_t = as_weighted(&self.model.at_t);

        let choices_init = WeightedChoice::new(&mut init);
        let choices_at_a = WeightedChoice::new(&mut at_a);
        let choices_at_c = WeightedChoice::new(&mut at_c);
        let choices_at_g = WeightedChoice::new(&mut at_g);
        let choices_at_t = WeightedChoice::new(&mut at_t);

        let mut rng = rand::thread_rng();
        let mut dna: Vec<Protein> = Vec::with_capacity(len);
        let first = choices_init.ind_sample(&mut rng);
        dna.push(first.clone());

        (0..len).fold(first, |previous, _| {
            let current = match &previous {
                &A => choices_at_a.ind_sample(&mut rng),
                &C => choices_at_c.ind_sample(&mut rng),
                &G => choices_at_g.ind_sample(&mut rng),
                &T => choices_at_t.ind_sample(&mut rng),
            };
            dna.push(current.clone());
            current
        });

        dna
    }
}

impl Default for DnaGenerator {
    fn default() -> Self {
        DnaGenerator {
            model: MarkovModel {
                init: [(A, 0.328), (C, 0.167), (G, 0.144), (T, 0.360)],
                at_a: [(A, 0.359), (C, 0.143), (G, 0.167), (T, 0.331)],
                at_c: [(A, 0.384), (C, 0.156), (G, 0.023), (T, 0.437)],
                at_g: [(A, 0.305), (C, 0.199), (G, 0.150), (T, 0.345)],
                at_t: [(A, 0.284), (C, 0.182), (G, 0.177), (T, 0.357)],
            },
        }
    }
}

#[derive(PartialEq)]
enum DnaMutation {
    Delete,
    Insert,
    Replace,
}

pub struct DnaMutator {
    mutation_rate: f32,
}

impl DnaMutator {
    fn new(mutation_rate: f32) -> Self {
        DnaMutator { mutation_rate: mutation_rate }
    }

    fn mutate(&self, source: &[Protein]) -> Vec<Protein> {
        let mut rng = rand::thread_rng();
        let mut destination: Vec<Protein> = Vec::with_capacity(source.len());
        for protein in source {
            if rng.next_f32() < self.mutation_rate {
                let mutations = [DnaMutation::Delete, DnaMutation::Insert, DnaMutation::Replace];
                let proteins = [Protein::A, Protein::C, Protein::G, Protein::T];
                let mutation = rng.choose(&mutations).unwrap();
                if (mutation == &DnaMutation::Insert) || (mutation == &DnaMutation::Replace) {
                    let protein = rng.choose(&proteins).unwrap();
                    destination.push(protein.clone());
                }
                if mutation == &DnaMutation::Insert {
                    destination.push(protein.clone());
                }
            } else {
                destination.push(protein.clone());
            }
        }
        destination
    }
}

#[cfg(test)]
mod tests {
    use super::{Protein, DnaGenerator, DnaMutator};

    fn as_string(sequence: &[Protein]) -> String {
        let bytes: Vec<u8> = sequence.iter().cloned().map(|p| p.into()).collect();
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn generate() {
        let generator = DnaGenerator::default();
        let dna = generator.generate(100);
        println!("{:?}", as_string(&dna));
        // panic!();
    }

    #[test]
    fn mutate() {
        let generator = DnaGenerator::default();
        let mutator = DnaMutator::new(0.1);
        let original = generator.generate(100);
        let mutation = mutator.mutate(&original);

        println!("{:?}", as_string(&original));
        println!("{:?}", as_string(&mutation));
        // panic!();
    }
}
