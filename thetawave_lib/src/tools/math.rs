use amethyst::{
    core::math::Vector3
};

pub fn distance(point_a: Vector3<f32>, point_b: Vector3<f32>, is_2d: bool) -> f32 {
    let mut value = (point_a.x - point_b.x).powi(2) + (point_a.y - point_b.y).powi(2);

    if !is_2d {
        value += (point_a.z - point_b.z).powi(2);
    }

    value.sqrt()
}

pub fn signed_modulo(a: f32, n: f32) -> f32 {
    a - (a / n).floor() * n
}

pub fn rotate_x(x: f32, y: f32, angle: f32) -> f32 {
    (x * angle.cos()) + (y * angle.sin())
}

pub fn rotate_y(x: f32, y: f32, angle: f32) -> f32 {
    (-x * angle.sin()) + (y * angle.cos())
}

/*
The code below is from this repository: https://github.com/JoelEager/Rust-Collision-Detector

MIT License

Copyright (c) 2018 Joel Eager

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f32, pub f32);

pub fn sat_is_colliding(poly1: &[Vector], poly2: &[Vector], max_dist: &Option<f32>) -> bool {
    let estimated_dist = (poly1[1].0 - poly2[0].0).powi(2) + (poly1[1].1 - poly2[0].1).powi(2);
    match max_dist {
        &Some(max_dist) if estimated_dist > max_dist.powi(2) => false,
        &Some(_) | &None => run_sat(poly1, poly2),
    }
}

pub fn run_sat(poly1: &[Vector], poly2: &[Vector]) -> bool {
    let mut edges = Vec::new();
    edges.append(&mut poly_to_edges(poly1));
    edges.append(&mut poly_to_edges(poly2));

    let axes = edges.into_iter().map(orthogonal);

    for axis in axes {
        if !overlap(project(poly1, axis), project(poly2, axis)) {
            return false;
        }
    }

    true
}

pub fn edge_vector(point1: Vector, point2: Vector) -> Vector {
    Vector(point2.0 - point1.0, point2.1 - point1.1)
}

pub fn poly_to_edges(poly: &[Vector]) -> Vec<Vector> {
    // Returns a Vec of the edges of the poly as vectors
    let mut edges = Vec::with_capacity(poly.len());

    for index in 0..poly.len() {
        edges.push(edge_vector(poly[index], poly[(index + 1) % poly.len()]));
    }

    edges
}

pub fn orthogonal(vector: Vector) -> Vector {
    Vector(vector.1, -vector.0)
}

pub fn dot_product(vector1: Vector, vector2: Vector) -> f32 {
    vector1.0 * vector2.0 + vector1.1 * vector2.1
}

pub fn project(poly: &[Vector], axis: Vector) -> Vector {
    let mut min: Option<f32> = None;
    let mut max: Option<f32> = None;

    for point in poly.iter() {
        let dot = dot_product(*point, axis);

        match min {
            Some(val) if val < dot => (),
            _ => min = Some(dot),
        }
        match max {
            Some(val) if val > dot => (),
            _ => max = Some(dot),
        }
    }

    Vector(min.unwrap(), max.unwrap())
}

pub fn overlap(projection1: Vector, projection2: Vector) -> bool {
    projection1.0 <= projection2.1 && projection2.0 <= projection1.1
}
