use aoc22::read_test_data;
use itertools::Itertools;

fn main() {
    env_logger::init();

    let input = read_test_data(1);
    let subtotals = input
        .lines()
        .map(|line| line.parse::<i64>().ok())
        .batching(|it| it.while_some().sum1());

    let part1: i64 = subtotals
        .clone()
        .max()
        .expect("Error getting max for part 1!");
    println!("[p1] Most calories: {part1:?}");

    let part2 = -subtotals.map(|n| -n).k_smallest(3).sum::<i64>();
    println!("[p2] Sum of top 3 calories: {part2:?}");
}
