use cidre::cg;

pub fn min(a: f64, b: f64) -> f64 {
    if a > b { b } else { a }
}

pub fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

pub fn max_x(rect: &cg::Rect) -> cg::Float {
    rect.origin.x + rect.size.width
}

pub fn min_x(rect: &cg::Rect) -> cg::Float {
    rect.origin.x
}

pub fn max_y(rect: &cg::Rect) -> cg::Float {
    rect.origin.y + rect.size.height
}

pub fn min_y(rect: &cg::Rect) -> cg::Float {
    rect.origin.y
}