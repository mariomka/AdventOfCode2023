use helpers::{input_grid, run, Grid};

fn main() {
    let input: Grid<char> = input_grid(include_str!("../input.txt"));

    run("part1", || day3::part1(&input));
    run("part2", || day3::part2(&input));
}
