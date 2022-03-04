use crate::GRAMS_TO_KG;

pub const C6_THRUST: [[f32; 2]; 21] = [
    [0.014, 0.633],
    [0.026, 1.533],
    [0.067, 2.726],
    [0.099, 5.136],
    [0.150, 9.103],
    [0.183, 11.465],
    [0.207, 11.635],
    [0.219, 11.391],
    [0.262, 6.377],
    [0.333, 5.014],
    [0.349, 5.209],
    [0.392, 4.722],
    [0.475, 4.771],
    [0.653, 4.746],
    [0.913, 4.673],
    [1.366, 4.625],
    [1.607, 4.625],
    [1.745, 4.868],
    [1.978, 4.795],
    [2.023, 0.828],
    [2.024, 0.0],
];

//TODO check if this realy is correct (initial) - propelant weight
pub const DRY_WEIGHT: f64 = (22.7 * GRAMS_TO_KG) - WET_WEIGHT;
pub const WET_WEIGHT: f64 = 12.48 * GRAMS_TO_KG;
