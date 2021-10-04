extern crate piston_window;
use piston_window::*;
static LINE_COLOR:[f32; 4] = [0.0, 1., 0.0, 1.0];
static LINE_RADIUS:f64 = 1.;
static LINE_DX:f64 = 2.;
static LINE_DT:f64 = 0.01;

pub fn draw_hermite<G: Graphics>(points: &Vec<[f64; 2]>, transform: [[f64; 3]; 2], g: &mut G) {
    // (2*t*t*t - 3*t*t + 1) f0
    // (t*t*t - 2*t*t + t) df0
    // (-2*t*t*t + 3*t*t) f1
    // (t*t*t - t*t) df1
    let l = points.len();
    if l > 1 {
        let mut dist = 0.;
        let mut t = 0.;
        let mut df0 = 0.;
        let mut df1 = 0.;
        let mut x = 0.;
        let mut y = 0.;
        let mut cur_line = [points[0][0], points[0][1], points[0][0], points[0][1]];
        for _i in 1..=l-1 {
            t = 0.;
            dist = points[_i][0] - points[_i-1][0];
            while t < 1. {
                cur_line[0] = cur_line[2];
                cur_line[1] = cur_line[3];

                df0 = (points[_i][1] - points[_i-1][1]) / dist;
                df1 = df0;
                if _i == 1 {
                    df0 = 0.;
                } else if _i == l-1 {
                    df1 = 0.;
                }
                let t2 = t*t;
                let t3 = t2 * t;
                
                x = t * dist + points[_i-1][0];
                y = (2.*t3 - 3.*t2 + 1.) * points[_i-1][1] +
                (t3 - 2.*t2 + t) * df0 +
                (-2.*t3 + 3.*t2) * points[_i][1] +
                (t3 - t2) * df1;

                cur_line[2] = x;
                cur_line[3] = y;
                line(LINE_COLOR,LINE_RADIUS, cur_line,transform, g);

                t += LINE_DT;
            }
        }
    }
}

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
            x += LINE_DX;
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
