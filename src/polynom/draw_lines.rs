extern crate piston_window;
use piston_window::*;
static LINE_COLOR:[f32; 4] = [0.0, 1., 0.0, 1.0];
static LINE_RADIUS:f64 = 1.;

pub fn draw_straight<G: Graphics>(points: &Vec<[f64; 2]>, transform: [[f64; 3]; 2], g: &mut G) {
    let l = points.len();
    if l > 1 {
        for _i in 1..=l-1 {
            line(LINE_COLOR,
                LINE_RADIUS,
                [points[_i-1][0], points[_i-1][1], points[_i][0], points[_i][1]],
                transform, g);
        }
    }
}

pub fn draw_lagrange<G: Graphics>(points: &Vec<[f64; 2]>, transform: [[f64; 3]; 2], g: &mut G) {
    let dx = 2.0;
    let l = points.len();
    if l > 1 {
        let right = points[l-1][0];
        let mut x = points[0][0];
        let mut cur_line = [x, points[0][1], x, points[0][1]];
        while x < right {
            cur_line[0] = cur_line[2];
            cur_line[1] = cur_line[3];
            let y = lagrange_func(points, x);
            cur_line[2] = x;
            cur_line[3] = y;
            line(LINE_COLOR,LINE_RADIUS, cur_line,transform, g);
            x += dx;
        }
    }
}

fn lagrange_func(points: &Vec<[f64; 2]>, x:f64) -> f64 {
    let mut result = 0.;
    for _i in 1..=points.len() {
        let i = _i - 1;
        result += points[i][1] * lagrange_basis_func(points, x, i);
    }
    return result;
}

fn lagrange_basis_func(points: &Vec<[f64; 2]>, x:f64, i:usize) -> f64 {
    let mut result = 1.;
    for _i in 1..=points.len() {
        let j = _i - 1;
        if j != i {
            result *= (x - points[j][0]) / (points[i][0] - points[j][0]);
        }
    }
    return result;
}
