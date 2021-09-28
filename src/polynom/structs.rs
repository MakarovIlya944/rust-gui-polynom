pub enum LineViewType {
    Straight,
    Lagrange,
    Hermite
}

pub fn cast_enum(view_type: &LineViewType) -> &'static str {
    match view_type {
        LineViewType::Straight => return "Straight",
        LineViewType::Lagrange => return "Lagrange",
        LineViewType::Hermite => return "Hermite",
    }
}