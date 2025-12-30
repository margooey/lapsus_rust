use cidre::cg;

pub fn min(a: f64, b: f64) -> f64 {
    if a > b { b } else { a }
}

pub fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

pub fn union_rect(a: &cg::Rect, b: &cg::Rect) -> cg::Rect {
    if *a == cg::Rect::null() {
        return *b;
    }
    if *b == cg::Rect::null() {
        return *a;
    }
    let min_x = min(a.origin.x, b.origin.x);
    let min_y = min(a.origin.y, b.origin.y);
    let max_x = max(a.origin.x + a.size.width, b.origin.x + b.size.width);
    let max_y = max(a.origin.y + a.size.height, b.origin.y + b.size.height);

    cg::Rect {
        origin: cg::Point { x: min_x, y: min_y },
        size: cg::Size {
            width: max_x - min_x,
            height: max_y - min_y,
        },
    }
}
