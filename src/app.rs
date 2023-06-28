use bevy::prelude::*;

use rand::{thread_rng, Rng};

use num_bigint::BigInt;
use num_traits::Zero;

#[derive(Resource)]
pub struct Currency(pub BigInt);

impl Default for Currency {
    fn default() -> Self {
        Currency(BigInt::zero())
    }
}

#[derive(Resource)]
pub struct Stats {
    pub per_click: BigInt,
    pub crit_chance: f32,
    pub crit_mult: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            per_click: BigInt::from(1),
            crit_chance: 0.0,
            crit_mult: 1.0,
        }
    }
}

pub enum Upgrade {
    Chicken,
    HenHouse,
}

impl std::fmt::Display for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Chicken => "Chicken",
                Self::HenHouse => "Hen House",
            }
        )
    }
}

#[derive(Resource)]
pub struct Upgrades {
    pub owned: Vec<(Upgrade, u32)>,
}

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Currency::default())
            .insert_resource(Stats::default());
    }
}

pub fn does_crit(crit_chance: f32) -> bool {
    thread_rng().gen::<f32>() < crit_chance
}

pub fn apply_crit(click_value: &BigInt, crit_mult: &f32) -> BigInt {
    // Multiply big numbers by a fraction
    let denominator = 100000;
    let numerator = BigInt::from((denominator as f32 * crit_mult).round() as u32);
    let denominator = BigInt::from(denominator);
    click_value * numerator / denominator
}
