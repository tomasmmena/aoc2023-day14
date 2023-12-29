use std::env;
use std::fs;
use std::io::{self, BufRead};


#[derive(Debug, PartialEq, Clone, Copy)]
enum TPSpace {
    Empty,
    RoundStone,
    SquareStone
}

enum Direction {
    North,
    West,
    East,
    South
}


#[derive(Debug, PartialEq, Clone)]
struct TiltingPlatform {
    matrix: Vec<Vec<TPSpace>>
}

impl TiltingPlatform {

    fn parse(lines: Vec<String>) -> Self {
        TiltingPlatform { 
            matrix: lines
                .into_iter()
                .map(|l| l
                    .chars()
                    .map(|c| match c {
                        '.' => TPSpace::Empty,
                        '#' => TPSpace::SquareStone,
                        'O' => TPSpace::RoundStone,
                        _ => panic!("Invalid char!")
                    })
                    .collect()
                )
                .collect() 
        }
    }

    fn load(path: &str) -> Self {
        TiltingPlatform::parse(
            io::BufReader::new(
                fs::File::open(path).expect("Could not open tilting platform file!")
            )
            .lines()
            .map(|line| line.expect("Could not read line!"))
            .collect()
        )
    }

    fn get_load(&self) -> usize {
        self.matrix.iter().rev().enumerate().map(|(factor, row)| {
            row.iter().filter(|s| **s == TPSpace::RoundStone).count() * (factor + 1)
        }).sum()
    }

    fn rotate_matrix(matrix: &Vec<Vec<TPSpace>>, times: usize) -> Vec<Vec<TPSpace>> {
        match times % 4 {
            0 => matrix.clone(),
            1 => (0..matrix[0].len()).into_iter().map(| i |
                matrix.iter().map(|row| row[i]).rev().collect()
            ).collect(),
            2 => matrix.iter().map(|row| row.iter().rev().map(|s| *s).collect()).rev().collect(),
            3 => TiltingPlatform::rotate_matrix(&TiltingPlatform::rotate_matrix(matrix, 1), 2),
            _ => panic!("This is impossible!")
        }
        
    }

    fn tilt_row(row: &Vec<TPSpace>) -> Vec<TPSpace> {
        let mut rounds: usize = 0;
        let mut empties: usize = 0;
        let mut new_row: Vec<TPSpace> = vec![];
        for space in row.iter() {
            match space {
                TPSpace::Empty => empties += 1,
                TPSpace::RoundStone => rounds += 1,
                TPSpace::SquareStone => {
                    new_row.extend(vec![TPSpace::RoundStone; rounds]);
                    new_row.extend(vec![TPSpace::Empty; empties]);
                    new_row.push(TPSpace::SquareStone);
                    rounds = 0;
                    empties = 0;
                }
            }
        }
        new_row.extend(vec![TPSpace::RoundStone; rounds]);
        new_row.extend(vec![TPSpace::Empty; empties]);

        new_row
    }

    fn tilt(&self, direction: &Direction) -> Self {
        let rotate = match direction {
            Direction::West => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::North => 3
        };
        let mut matrix = TiltingPlatform::rotate_matrix(&self.matrix, rotate);
        matrix = matrix
            .iter()
            .map(TiltingPlatform::tilt_row)
            .collect();
        matrix = TiltingPlatform::rotate_matrix(&matrix, 4 - rotate);
        TiltingPlatform { matrix }
    }

    fn cycle_brute_force(&self, times: usize) -> Self {
        let mut out: TiltingPlatform = self.clone();
        for iteration in 0..times {
            for direction in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East
            ] {
                out = out.tilt(&direction);
            }
        }
        out
    }

    /// This method produces the state after `times` cycles, but it keeps a list of states encountered to abort 
    /// when a loop is detected and all possible configurations have been encountered. When that condition is met 
    /// the previously calculated state that lines up with the remaining number of iterations is returned, if no 
    /// loop is found by the time the iterations are exhausted, this function operates essentialy like the brute 
    /// force version.
    /// 
    /// # Arguments
    /// 
    /// - `times`: number of cycles as a usize.
    fn cycle(&self, times: usize) -> Self {
        let mut out: TiltingPlatform = self.clone();
        let mut states: Vec<TiltingPlatform> = vec![];
        for iteration in 0..times {
            for direction in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East
            ] {
                out = out.tilt(&direction);
            }
            match states.iter().position(|s| *s == out) {
                Some(i) => return states[i + (times - iteration) % (states.len() - i) - 1].clone(),
                None => states.push(out.clone())
            }
        }
        out
    }

    fn to_str(&self) -> String {
        self.matrix
            .iter()
            .map(|row| row
                    .iter()
                    .map(|s| match s {
                        TPSpace::Empty => '.',
                        TPSpace::RoundStone => 'O',
                        TPSpace::SquareStone => '#',
                    })
                    .collect::<String>()
            )
            .collect::<Vec<String>>()
            .join("\n")
            
    }

}


fn main() {
    let path = env::args().nth(1).expect("Missing required param path!");
    let platform = TiltingPlatform::load(path.as_str())
        .cycle(1_000_000_000);

    // println!("{}", platform.to_str());
    println!("Total load: {}", platform.get_load());

}


#[cfg(test)]
mod tests {
    use crate::{Direction, TiltingPlatform};

    fn get_tp1() -> TiltingPlatform {
        TiltingPlatform::parse(
            vec![
                String::from("..#..O.."),
                String::from("..#O...."),
                String::from("O.#....."),
                String::from("..#.O..O"),
                String::from("..#.O.OO"),
            ]
        )
    }

    fn get_tp2() -> TiltingPlatform {
        TiltingPlatform::parse(
            vec![
                String::from("..O..#.."),
                String::from("....O#.."),
                String::from(".....#.O"),
                String::from("O..O.#.."),
            ]
        )
    }

    fn get_tp3() -> TiltingPlatform {
        TiltingPlatform::parse(
            vec![
                String::from("..O...O"),
                String::from("....O.."),
                String::from("....O.."),
                String::from(".O....."),
                String::from("#######"),
                String::from("O..O..."),
            ]
        )
    }

    fn get_tp4() -> TiltingPlatform {
        TiltingPlatform::parse(
            vec![
                String::from("O..O..."),
                String::from("#######"),
                String::from(".O....."),
                String::from("....O.."),
                String::from("....O.."),
                String::from("..O...O"),
            ]
        )
    }

    fn get_tp5() -> TiltingPlatform {
        TiltingPlatform::parse(
            vec![
                String::from("O..O..."),
                String::from("#######"),
                String::from(".O....."),
                String::from("....O.."),
                String::from("....O#."),
                String::from("..O...O"),
            ]
        )
    }

    #[test]
    fn test_tilt_west() {
        let test_platform = get_tp1();
        let stepped_platform = test_platform.tilt(&Direction::West);
        println!("{}", test_platform.to_str());
        println!("{}", stepped_platform.to_str());
        assert_eq!(
            stepped_platform,
            TiltingPlatform::parse(
                vec![
                    String::from("..#O...."),
                    String::from("..#O...."),
                    String::from("O.#....."),
                    String::from("..#OO..."),
                    String::from("..#OOO.."),
                ]
            )
        )
    }

    #[test]
    fn test_tilt_east() {
        let test_platform = get_tp2();
        let stepped_platform = test_platform.tilt(&Direction::East);
        println!("{}", test_platform.to_str());
        println!("{}", stepped_platform.to_str());
        assert_eq!(
            stepped_platform,
            TiltingPlatform::parse(
                vec![
                    String::from("....O#.."),
                    String::from("....O#.."),
                    String::from(".....#.O"),
                    String::from("...OO#.."),
                ]
            )
        )
    }

    #[test]
    fn test_tilt_south() {
        let test_platform = get_tp3();
        let stepped_platform = test_platform.tilt(&Direction::South);
        println!("{}", test_platform.to_str());
        println!("{}", stepped_platform.to_str());
        assert_eq!(
            stepped_platform,
            TiltingPlatform::parse(
                vec![
                    String::from("......."),
                    String::from("......."),
                    String::from("....O.."),
                    String::from(".OO.O.O"),
                    String::from("#######"),
                    String::from("O..O..."),
                ]
            )
        )
    }

    #[test]
    fn test_tilt_north() {
        let test_platform = get_tp4();
        let stepped_platform = test_platform.tilt(&Direction::North);
        println!("{}", test_platform.to_str());
        println!("{}", stepped_platform.to_str());
        assert_eq!(
            stepped_platform,
            TiltingPlatform::parse(
                vec![
                    String::from("O..O..."),
                    String::from("#######"),
                    String::from(".OO.O.O"),
                    String::from("....O.."),
                    String::from("......."),
                    String::from("......."),
                ]
            )
        )
    }

    #[test]
    fn test_cycle() {
        let test_platform = get_tp5();
        let stepped_platform = test_platform.cycle_brute_force(100);
        println!("{}", test_platform.to_str());
        println!("{}", stepped_platform.to_str());
        assert_eq!(
            stepped_platform,
            TiltingPlatform::parse(
                vec![
                    String::from(".....OO"),
                    String::from("#######"),
                    String::from("......."),
                    String::from("......O"),
                    String::from("....O#."),
                    String::from("....OOO"),
                ]
            )
        )
    }

    #[test]
    fn test_cycle_with_loop_detection() {
        let test_platform = get_tp5();
        let stepped_platform = test_platform.cycle(100);
        println!("{}", test_platform.to_str());
        println!("{}", stepped_platform.to_str());
        assert_eq!(
            stepped_platform,
            TiltingPlatform::parse(
                vec![
                    String::from(".....OO"),
                    String::from("#######"),
                    String::from("......."),
                    String::from("......O"),
                    String::from("....O#."),
                    String::from("....OOO"),
                ]
            )
        )
    }

}