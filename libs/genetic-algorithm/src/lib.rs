use std::collections::BTreeMap;
use rand::SeedableRng;

use rand::RngCore;
use rand::Rng;
use rand::seq::SliceRandom;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
}
pub struct RouletteWheelSelection;

pub trait Individual {
    fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I where I: Individual;
}

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(
       &self,
       rng: &mut dyn RngCore,
       population: &'a [I],
    ) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

impl<S> GeneticAlgorithm<S> where S: SelectionMethod,
{
    pub fn new(selection_method: S) -> Self {
        Self { selection_method }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I> where I: Individual, {
        assert!(!population.is_empty());

        (0..population.len())
        .map(|_| {
            let parent_a = self
                .selection_method
                .select(rng, population);

            let parent_b = self
                .selection_method
                .select(rng, population);

            // TODO crossover
            // TODO mutation
            todo!()
        })
        .collect()
    }
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[test]
fn test() {
    let method = RouletteWheelSelection::new();
    let mut rng = rand_chacha::ChaCha8Rng::from_seed(Default::default());

    let population = vec![
        TestIndividual::new(2.0),
        TestIndividual::new(1.0),
        TestIndividual::new(4.0),
        TestIndividual::new(3.0),
    ];

    let actual_histogram: BTreeMap<i32, _> = (0..1000)
        .map(|_| method.select(&mut rng, &population))
        .fold(Default::default(), |mut histogram, individual| {
            *histogram
                .entry(individual.fitness() as _)
                .or_default() += 1;

            histogram
        });

    let expected_histogram = maplit::btreemap! {
        // fitness => how many times this fitness has been chosen
        1 => 98,
        2 => 202,
        3 => 278,
        4 => 422,
    };

    assert_eq!(actual_histogram, expected_histogram);
}