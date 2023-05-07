use std::f64::EPSILON;

pub fn unit_vector(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;
    let magnitude = (dx * dx + dy * dy).sqrt();

    if magnitude < EPSILON {
        return (0.0, 0.0);
    }

    (dx / magnitude, dy / magnitude)
}

pub fn unit_vector_u32(p1: &(u32, u32), p2: &(u32, u32)) -> (f64, f64) {
    let dx: i64 = p2.0 as i64 - p1.0 as i64;
    let dy: i64 = p2.1 as i64 - p1.1 as i64;
    let magnitude = ((dx * dx + dy * dy) as f64).sqrt();

    if magnitude < EPSILON {
        return (0.0, 0.0);
    }

    (dx as f64 / magnitude, dy as f64 / magnitude)
}

/// Finds every point from p1 to p2 in a direct line
/// adapted from https://lodev.org/cgtutor/raycasting.html
pub fn points_between(p1: &(u32, u32), p2: &(u32, u32)) -> Vec<(u32, u32)> {
    let unit = unit_vector_u32(p1, p2);

    let delta_dist_x = if unit.0 == 0_f64 {
        std::f64::MAX
    } else {
        (1_f64 / unit.0).abs()
    };

    let delta_dist_y = if unit.1 == 0_f64 {
        std::f64::MAX
    } else {
        (1_f64 / unit.1).abs()
    };

    let step_x: i32;
    let step_y: i32;
    let mut side_dist_x;
    let mut side_dist_y;

    if unit.0 < 0_f64 {
        step_x = -1;
        side_dist_x = 0_f64;
    } else {
        step_x = 1;
        side_dist_x = delta_dist_x;
    }

    if unit.1 < 0_f64 {
        step_y = -1;
        side_dist_y = 0_f64;
    } else {
        step_y = 1;
        side_dist_y = delta_dist_y;
    }

    let mut next_point = p1.clone();
    let mut points = Vec::new();
    while next_point != *p2 {
        points.push(next_point);

        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            if (next_point.0 as i32 + step_x) < 0_i32 {
                next_point.0 = 0_u32;
            } else {
                next_point.0 = (next_point.0 as i32 + step_x) as u32;
            }

            if (step_x < 0 && next_point.0 < p2.0) || (step_x > 0 && next_point.0 > p2.0) {
                break;
            }
        } else {
            side_dist_y += delta_dist_y;
            if (next_point.1 as i32 + step_y) < 0_i32 {
                next_point.1 = 0_u32;
            } else {
                next_point.1 = (next_point.1 as i32 + step_y) as u32;
            }

            if (step_y < 0 && next_point.1 < p2.1) || (step_y > 0 && next_point.1 > p2.1) {
                break;
            }
        }
    }

    points
}
