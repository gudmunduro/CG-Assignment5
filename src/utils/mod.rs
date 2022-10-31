use nalgebra::Vector2;

#[derive(Clone)]
pub enum FacingDirection {
    North,
    West
}

pub fn limit(value: f32, from: f32, to: f32) -> f32 {
    value.min(to).max(from)
}

pub fn line_contains_point(start: &Vector2<f32>, end: &Vector2<f32>, point: &Vector2<f32>) -> bool {
    // We need to handle the cases when the line goes in both directions
    if start.x <= end.x {
        if !(point.x >= start.x && point.x <= end.x) {
            return false;
        }
    } else if start.x > end.x {
        if !(point.x >= end.x && point.x <= start.x) {
            return false;
        }
    }

    if start.y <= end.y {
        if !(point.y >= start.y && point.y <= end.y) {
            return false;
        }
    } else if start.y > end.y {
        if !(point.y >= end.y && point.y <= start.y) {
            return false;
        }
    }

    true
}