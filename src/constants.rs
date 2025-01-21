#![allow(dead_code)]

pub const FIRST_POINTS: f64 = 11.0;
pub const SECOND_POINTS: f64 = 9.5;
pub const THIRD_POINTS: f64 = 8.0;
pub const FOURTH_POINTS: f64 = 7.0;
pub const FIFTH_POINTS: f64 = 5.0;
pub const SEVENTH_POINTS: f64 = 3.0;
pub const NINTH_POINTS: f64 = 1.0;
pub const ELSE_POINTS: f64 = 0.0;

pub const MINIMUM_ENTRANT_COUNT: u32 = 10;

pub enum CalculationMethods {
    AveragePlacement,
    WeightedPoints,
    MedianPoints,
    MeanPoints,
    UnweightedPoints,
    OverallPRPlacement
}

pub const fn point_values(placement: u32) -> f64 {
    match placement {
        1 => FIRST_POINTS,
        2 => SECOND_POINTS,
        3 => THIRD_POINTS,
        4 => FOURTH_POINTS,
        5 => FIFTH_POINTS,
        7 => SEVENTH_POINTS,
        9 => NINTH_POINTS,
        _ => ELSE_POINTS
    }
}
