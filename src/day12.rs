use crate::util::{Coord2D, Grid};

pub struct Plot {
    plant: char,
    visited: bool,
}
type Input = Grid<i32, Plot>;

pub fn input() -> Input {
    Grid::from_file("data/day12.txt", |p, _| {
        Some(Plot {
            plant: p,
            visited: false,
        })
    })
}

fn grow(grid: &mut Input, coord: &Coord2D<i32>, plant: char) -> (usize, usize) {
    let plot = grid.squares.get_mut(&coord).unwrap();
    if plot.plant != plant {
        return (0, 1);
    }
    if plot.visited {
        return (0, 0);
    }
    plot.visited = true;

    let mut area = 1;
    let mut perimeter = 0;
    for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbour = Coord2D::new(coord.x + dx, coord.y + dy);
        if grid.squares.contains_key(&neighbour) {
            let (new_area, new_perimeter) = grow(grid, &neighbour, plant);
            area += new_area;
            perimeter += new_perimeter;
        } else {
            perimeter += 1;
        }
    }
    (area, perimeter)
}

pub fn part1(mut grid: Input) -> usize {
    let mut cost = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let plant = grid.squares.get_mut(&Coord2D::new(x, y)).unwrap().plant;
            let (area, perimeter) = grow(&mut grid, &Coord2D::new(x, y), plant);
            cost += area * perimeter;
        }
    }
    cost
}

pub fn part2(grid: Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1446042);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 0);
    }
}
