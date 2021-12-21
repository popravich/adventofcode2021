use std::collections::HashSet;


pub fn main(input: &str) -> Result<(isize, isize), String> {
    let mut it = input
        .trim_end()
        .split(|c| c == '=' || c == ',' || c == '.')
        .filter(|s| !s.is_empty())
        .filter(|s| s.chars().all(|c| c == '-' || c.is_digit(10)));

    let x1 = it
        .next()
        .ok_or("invalid data".to_string())?
        .parse::<isize>()
        .map_err(|e| format!("{}", e))?;
    let x2 = it
        .next()
        .ok_or("invalid data".to_string())?
        .parse::<isize>()
        .map_err(|e| format!("{}", e))?;
    let y1 = it
        .next()
        .ok_or("invalid data".to_string())?
        .parse::<isize>()
        .map_err(|e| format!("{}", e))?;
    let y2 = it
        .next()
        .ok_or("invalid data".to_string())?
        .parse::<isize>()
        .map_err(|e| format!("{}", e))?;

    let top_vertical_speed = 0 - y1 - 1;
    // TODO: find out number of steps;
    let part1 = arithm_series_sum(
        top_vertical_speed as f32,
        top_vertical_speed as f32,
    ) as isize;

    let mut initial_y = HashSet::new();
    for target_y in y1..=y2 {
        initial_y.insert((target_y, 1));
        let lo = target_y;
        let hi = 0 - target_y - 1;
        for test_y in lo..=hi {
            if let Some(s) = quadrtic_equation_for_arithm_series(test_y, target_y) {
                initial_y.insert((test_y, s));
            }
        }
    }
    // next, calculate all x speed for each steps for each X target;

    let mut initial_x = HashSet::new();
    for target_x in x1..=x2 {
        for test_x in 1..=target_x {
            if let Some(s) = quadrtic_equation_for_arithm_series(test_x, target_x) {
                initial_x.insert((test_x, s));
            }
        }
    }
    let mut result = HashSet::new();
    for (y, y_steps) in initial_y.iter() {
        for (x, x_steps) in initial_x.iter() {
            if y_steps == x_steps {
                result.insert((x, y));
            } else if y_steps > x_steps && x == x_steps {
                result.insert((x, y));
            }
        }
    }

    let part2 = result.len() as isize;

    Ok((part1, part2))
}

fn arithm_series_sum(speed: f32, time: f32) -> f32 {
    (time / 2.0) * ((2.0 * speed) - (time - 1.0))
}

/**

Resulting coordinate (S) at certain moment/step (t) with speed (s)
is a sum of arithmetic progression:

    S(v,t) = t / 2 * (2 * v - (t - 1))

    S(v,t) = t / 2 * (2 * v - t + 1)
    S(v,t) = t *v - t**2 + 1/2*t
    S(v,t) = -t**2 + (v+1/2)*t

    -1 * t**2 + t * (v + 1/2) - S = 0

Solutions:
    t1 = (-(v+1/2) + sqrt( (v+1/2)**2 - 4 * (1/2 * -1 * (-S)))) / 2 * (1/2 * -1)
    t2 = (-(v+1/2) - sqrt( (v+1/2)**2 - 4 * (1/2 * -1 * (-S)))) / 2 * (1/2 * -1)

    t1 = (-(v+1/2) + sqrt( (s+1/2)**2 - 2 * S)) * -1
    t2 = (-(v+1/2) - sqrt( (s+1/2)**2 - 2 * S)) * -1

    t1 = (v+1/2) - sqrt( (s+1/2)**2 - 2 * S))
    t2 = (v+1/2) + sqrt( (s+1/2)**2 - 2 * S))

**/
fn quadrtic_equation_for_arithm_series(initial_speed: isize, target: isize) -> Option<isize> {
    let s = initial_speed as f32 + 0.5;
    let ss = s * s;
    // we are not interested in fraction or negative time, so:
    let t1 = s - (ss - (2.0 * target as f32)).sqrt();
    let t2 = s + (ss - (2.0 * target as f32)).sqrt();
    // take result which is positive but min
    let is_ok = |t: f32| {
        t.is_finite() && t.is_sign_positive() && (t - t.floor()).abs() <= 1e-10
    };
    if is_ok(t1) {
        Some(t1 as isize)
    } else if is_ok(t2) {
        Some(t2 as isize)
    } else {
        None
    }
}


#[cfg(test)]
mod test {
    use crate::day17::main;

    #[test]
    fn solution() {
        let (p1, p2) = main("target area: x=20..30, y=-10..-5")
            .expect("invalid input");
        assert_eq!(p1, 45);
        assert_eq!(p2, 112);
    }
}
