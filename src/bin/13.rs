use fraction::GenericFraction;

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Button {
    _label: char,
    cost: usize,
    dx: usize,
    dy: usize,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let (button, deltas) = value.split_once(':').unwrap();
        let (_, label) = button.split_once(' ').unwrap();
        let (dx, dy) = deltas.split_once(", ").unwrap();
        let (_, dx) = dx.split_once('+').unwrap();
        let dx = dx.parse().unwrap();
        let (_, dy) = dy.split_once('+').unwrap();
        let dy = dy.parse().unwrap();
        let label = label.chars().next().unwrap();
        Self {
            _label: label,
            cost: match label {
                'A' => 3,
                'B' => 1,
                _ => panic!(),
            },
            dx,
            dy,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    x: usize,
    y: usize,
}

impl From<&str> for Prize {
    fn from(value: &str) -> Self {
        let (_prize, coords) = value.split_once(':').unwrap();
        let (x, y) = coords.split_once(", ").unwrap();
        let (_, x) = x.split_once('=').unwrap();
        let x = x.parse().unwrap();
        let (_, y) = y.split_once('=').unwrap();
        let y = y.parse().unwrap();
        Self { x, y }
    }
}

type F = GenericFraction<usize>;

impl Prize {
    /// ## Return
    /// - Some(usize) -> min token cost of button pushes
    /// - None -> no sol
    fn solve_for(&self, button1: Button, button2: Button) -> Option<usize> {
        // f1 * dx1 + f2 * dx2 = X
        // f1 * dy1 + f2 * dy2 = Y
        // <=>
        // f1 = (X - f2 * dx2) / dx1
        // (X - f2 * dx2) * dy1 / dx1 + f2 * dy2 = Y
        // f2 * dy2 - (f2 * dx2 * dy1) / dx1 = Y - X * dy1 / dx1
        // f2 (dy2 - dx2 * dy1 / dx1) = Y - X * dy1 / dx1
        // f2 = (Y - X * dy1 / dx1) / (dy2 - dx2 * dy1 / dx1)
        let dy1_dx1 = F::new(button1.dy, button1.dx);
        let f2 = (F::from(self.y) - F::from(self.x) * dy1_dx1)
            / (F::from(button2.dy) - F::from(button2.dx) * dy1_dx1);
        let f1 = (F::from(self.x) - f2 * button2.dx) / F::from(button1.dx);
        let f2: usize = f2.try_into().ok()?;
        let f1: usize = f1.try_into().ok()?;
        return Some(f1 * button1.cost + f2 * button2.cost);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let arcades = input.split("\n\n");
    Some(
        arcades
            .filter_map(|arcade| {
                let mut lines = arcade.split('\n');
                let button_a = Button::from(lines.next().unwrap());
                let button_b = Button::from(lines.next().unwrap());
                let prize = Prize::from(lines.next().unwrap());
                prize.solve_for(button_a, button_b)
            })
            .sum(),
    )
}

const PRIZE_DELTA: usize = 10_000_000_000_000;
pub fn part_two(input: &str) -> Option<usize> {
    let arcades = input.split("\n\n");
    // return None;
    Some(
        arcades
            .filter_map(|arcade| {
                let mut lines = arcade.split('\n');
                let button_a = Button::from(lines.next().unwrap());
                let button_b = Button::from(lines.next().unwrap());
                let mut prize = Prize::from(lines.next().unwrap());
                prize.x += PRIZE_DELTA;
                prize.y += PRIZE_DELTA;
                prize.solve_for(button_a, button_b)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
