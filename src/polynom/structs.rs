pub struct MyPoint {
   pub x: f32,
   pub y: f32,
}

pub struct MyLine {
    pub points: Vec<MyPoint>
}

pub enum LineViewType {
    Straight,
    Lagrange
}