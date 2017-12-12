pub fn solve_first(text: &str) -> i32 {
    let coordinates = get_coordinate(text);
    get_steps(coordinates)
}

pub fn solve_second(text: &str) -> i32 {
    get_furthest(text)
}

fn get_steps(coordinates: (f32, f32)) -> i32 {
    let (x_final, y_final) = coordinates;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut steps: i32 = 0;
    loop {
        if x == x_final {
            if y == y_final {
                break;
            }
        }

        if x > x_final {
            x -= 0.5;
            if y >= y_final {
                y -= 0.5;
            } else {
                y += 0.5;
            }
        } else if x < x_final {
            x += 0.5;
            if y >= y_final {
                y -= 0.5;
            } else {
                y += 0.5;
            }
        } else if x == x_final {
            if y >= y_final {
                y -= 1.0;
            } else {
                y += 1.0;
            }
        }

        steps += 1;
    }

    return steps;
}

fn get_furthest(text: &str) -> i32 {
    let split = text.split(',');
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    let mut steps;
    let mut max_steps: i32 = 0;
    for dir in split {

        match &dir as &str {
            "n" => y += 1.0,
            "nw" => { y += 0.5; x -= 0.5 },
            "ne" => { y += 0.5; x += 0.5 },
            "s" => y -= 1.0,
            "sw" => { y -= 0.5; x -= 0.5 },
            "se" => { y -= 0.5; x += 0.5 }
            _ => println!("no match")
        }

        steps = get_steps((x, y));
        if steps > max_steps {
            max_steps = steps;
        }
    }

    max_steps
}

fn get_coordinate(text: &str) -> (f32, f32) {
    let split = text.split(',');
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    for dir in split {

        match &dir as &str {
            "n" => y += 1.0,
            "nw" => { y += 0.5; x -= 0.5 },
            "ne" => { y += 0.5; x += 0.5 },
            "s" => y -= 1.0,
            "sw" => { y -= 0.5; x -= 0.5 },
            "se" => { y -= 0.5; x += 0.5 }
            _ => println!("no match")
        }
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_first() {
        let steps = solve_first("ne,ne,ne");
        assert_eq!(steps, 3);
    }

    #[test]
    fn first_second() {
        let steps = solve_first("ne,ne,sw,sw");
        assert_eq!(steps, 0);
    }

    #[test]
    fn first_third() {
        let steps = solve_first("ne,ne,s,s");
        assert_eq!(steps, 2);
    }

    #[test]
    fn first_fourth() {
        let steps = solve_first("se,sw,se,sw,sw");
        assert_eq!(steps, 3);
    }

    #[test]
    fn second_first() {
        let steps = solve_second("ne,ne,ne");
        assert_eq!(steps, 3);
    }

    #[test]
    fn second_second() {
        let steps = solve_second("ne,ne,sw,sw");
        assert_eq!(steps, 2);
    }

    #[test]
    fn second_third() {
        let steps = solve_second("ne,ne,s,s");
        assert_eq!(steps, 2);
    }

    #[test]
    fn second_fourth() {
        let steps = solve_second("se,sw,se,sw,sw");
        assert_eq!(steps, 3);
    }
}