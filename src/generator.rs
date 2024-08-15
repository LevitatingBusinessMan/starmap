use std::sync::LazyLock;
use rand::prelude::*;
use rand_distr;

pub mod names;

/// <https://en.wikipedia.org/wiki/Stellar_classification#Harvard_spectral_classification>
pub static STARCLASSES: &'static [(char, f64)] = &[('O', 0.00003), ('B', 0.12), ('A', 0.61), ('F', 3.0), ('G', 7.6), ('K', 12.0), ('M', 76.0)];

#[derive(Debug)]
pub struct Star {
    pub name: &'static str,
    pub class: char,
    pub planets: u8,
    pub cords: (f64, f64),
}

pub static AMOUNT: usize = 512;
static PLANET_DISTRIBUTION: LazyLock<rand_distr::Normal<f32>> = LazyLock::new(|| rand_distr::Normal::new(7.0, 4.0).unwrap());

// <SmallRng as SeedableRng>::Seed could be used here to seed from strings
pub fn generate_stars_with_seed(seed: u64) -> Vec<Star> {
    let mut rng = <SmallRng as SeedableRng>::seed_from_u64(seed);
    let mut stars = Vec::with_capacity(AMOUNT);

    for _ in 0..AMOUNT {
        stars.push(generate_star(&mut rng));
    }
    stars
}

pub fn generate_stars() -> (Vec<Star>, u64) {
    let seed = random();
    let stars = generate_stars_with_seed(seed);

    (stars, seed)
}

fn generate_star(rng: &mut impl Rng) -> Star {
    let name = *names::NAMES.choose(rng).unwrap();
    let class = STARCLASSES.choose_weighted(rng, |c| c.1).unwrap().0;
    let planets = PLANET_DISTRIBUTION.sample(rng).round() as u8;
    let cords = rng.gen();
    Star {name, class, planets, cords}
}
