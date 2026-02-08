use core::f32;

// -------------------------------------------------------------------------- //
// -------------------------------- ANGLES ---------------------------------- //
// -------------------------------------------------------------------------- //

// returns Some(angle) of the line connecting point 'a' to point 'b' in the xy plane.
// returns None if the points are the same.
// e.g.
//
//   * --> (+x)
//   |
//   v         (ax, ay)
//  (+y)     x
//
//                 x
//                  (bx, by)
//
pub fn get_angle(a: &(f32, f32), b: &(f32, f32)) -> Option<f32> {
    let (xa, ya) = a;
    let (xb, yb) = b;

    // ensure the deltas are only the absolute differences
    let dy = (ya.abs() - yb.abs()).abs();
    let dx = (xa.abs() - xb.abs()).abs();

    let result: f32;

    // point 'a' and 'b are actually the same point,
    // and therefore no angle exists between them.
    if dx == 0.0 && dy == 0.0 {
        return None;
    }

    // the two points make a vertical line
    if dx == 0.0 {
        if yb < ya {
            // yb is "above" ya
            result = 90.0;
        } else {
            // yb is "below" ya
            result = 270.0;
        }
        return Some(result);
    }

    // the two points make a horizontal line
    if dy == 0.0 {
        if xb > xa {
            // xb is "right" of xa
            result = 0.0;
        } else {
            // xb is "left" of xa
            result = 180.0;
        }
        return Some(result);
    }

    // only positive deltas means the atan only returns angles [0, 90[.
    // this initial angle can then be rotated by 90 degree increments as needed.
    result = (dy / dx).atan() * 180.0 / f32::consts::PI;

    // is the line oriented top-left? (Q2: 90->180)
    // reflect the result around the y-axis
    if xb < xa && yb < ya {
        Some(180.0 - result)
    // is the line oriented bottom-left? (Q3: 180->270)
    // reflect the result around both the x-axis and y-axis
    } else if xb < xa && (yb > ya || dy == 0.0) {
        Some(180.0 + result)
    // is the line oriented bottom-right? (Q4: 270->360)
    // reflect the result around both the x-axis
    } else if xb > xa && yb > ya {
        Some(360.0 - result)
    } else {
        // is the line oriented top-right? (Q1: 0->90)
        // no need to reflect into another Quadrant
        Some(result)
    }
}

/// divides the unit-circle into the specified number of segments
/// and maps the input angle to the closest segment angle.
/// the angles are assumed to be in degrees
/// (NOTE: degrees * 180 / pi = radians)
///
/// # Examples
///
/// ```rust
/// use ascii_arcade::entity::angles::map_angle;
/// assert_eq!(map_angle(350.0, 8), 0.0);
/// assert_eq!(map_angle(10.0, 8), 0.0);
/// assert_eq!(map_angle(40.0, 8), 45.0);
/// assert_eq!(map_angle(50.0, 8), 45.0);
/// assert_eq!(map_angle(80.0, 8), 90.0);
/// assert_eq!(map_angle(100.0, 8), 90.0);
/// assert_eq!(map_angle(120.0, 8), 135.0);
/// assert_eq!(map_angle(140.0, 8), 135.0);
/// assert_eq!(map_angle(170.0, 8), 180.0);
/// assert_eq!(map_angle(190.0, 8), 180.0);
/// assert_eq!(map_angle(220.0, 8), 225.0);
/// assert_eq!(map_angle(230.0, 8), 225.0);
/// assert_eq!(map_angle(260.0, 8), 270.0);
/// assert_eq!(map_angle(280.0, 8), 270.0);
/// assert_eq!(map_angle(310.0, 8), 315.0);
/// assert_eq!(map_angle(320.0, 8), 315.0);
/// assert_eq!(map_angle(46.0, 4), 90.0);
/// ```
///
pub fn map_angle(angle: f32, segment_count: usize) -> f32 {
    // normalize the angle to fit into one rotation
    let deg = angle % 360.0;

    // determine the size of each segment
    let step = 360.0 / (segment_count as f32 * 2.0);
    let mut current_step = 0.0;

    // map the angle to the nearest segment
    let mut counter: usize = 1;
    while current_step < 360.0 {
        if (current_step..(current_step + step)).contains(&deg) {
            if counter.is_multiple_of(2) {
                if current_step + step == 360.0 {
                    return 0.0;
                }
                return current_step + step;
            }
            return current_step;
        }
        current_step += step;
        counter += 1;
    }

    0.0
}
