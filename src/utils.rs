use cidre::cg::{Point, Rect, Size, EventSrc, EventSrcStateId};

pub fn min(a: f64, b: f64) -> f64 {
    if a > b { b } else { a }
}

pub fn max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

pub fn union_rect(a: &Rect, b: &Rect) -> Rect {
    if *a == Rect::null() {
        return *b;
    }
    if *b == Rect::null() {
        return *a;
    }
    let min_x = min(a.origin.x, b.origin.x);
    let min_y = min(a.origin.y, b.origin.y);
    let max_x = max(a.origin.x + a.size.width, b.origin.x + b.size.width);
    let max_y = max(a.origin.y + a.size.height, b.origin.y + b.size.height);

    Rect {
        origin: Point { x: min_x, y: min_y },
        size: Size {
            width: max_x - min_x,
            height: max_y - min_y,
        },
    }
}

pub fn disable_local_event_suppression() {
    let state_id = EventSrcStateId::CombinedSession;
    let mut event_source_ref = EventSrc::with_state(state_id);
    if let Some(ref mut retained) = event_source_ref {
        EventSrc::set_local_events_suppression_interval(retained, 0.0);
    }
}
