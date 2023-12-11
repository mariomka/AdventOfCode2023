use helpers::{input_grid, run, Grid};

fn main() {
    let input: Grid<char> = input_grid(include_str!("../input.txt"));

    run("part1", || day10::part1(&input));
    run("part2", || day10::part2(&input));
}
