use bevy::math::Vec2;
use num_complex::Complex;

use crate::shader::Root;

pub fn expand_polynomial(roots: &Vec<Root>) -> Vec<Vec2> {
    let mut coefficients = vec![Complex::new(1.0, 0.0)];

    for root in roots.iter().map(|r| Complex::new(r.pos.x, r.pos.y)) {
        let mut new_coefficients = vec![Complex::new(0.0, 0.0); coefficients.len() + 1];

        for (i, &coef) in coefficients.iter().enumerate() {
            new_coefficients[i] += coef;
            new_coefficients[i + 1] -= coef * root;
        }

        coefficients = new_coefficients;
    }

    coefficients.reverse();
    coefficients.iter().map(|c| Vec2::new(c.re, c.im)).collect()
}

pub fn derivative(coefficients: &Vec<Vec2>) -> Vec<Vec2> {
    let mut new_coefficients = vec![];

    for (i, coef) in coefficients
        .iter()
        .map(|c| Complex::new(c.x, c.y))
        .enumerate()
        .skip(1)
    {
        new_coefficients.push(coef * i as f32);
    }

    new_coefficients
        .iter()
        .map(|c| Vec2::new(c.re, c.im))
        .collect()
}
