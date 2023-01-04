use aoc22::read_data;
use itertools::Itertools;

fn parse_calories<'a>(input: &'a str) -> impl Iterator<Item = Option<i64>> + 'a {
    input.lines().map(|line| line.parse::<i64>().ok())
}

fn calorie_subtotals(calories: impl Iterator<Item = Option<i64>>) -> impl Iterator<Item = i64> {
    calories.batching(|it| it.while_some().sum1())
}

fn sum_top_n(xs: &[i64], n: usize) -> i64 {
    if n == 1 {
        *xs.iter().max().expect("Error getting max element!")
    } else {
        -xs.iter().map(|x| -x).k_smallest(n).sum::<i64>()
    }
}

fn main() {
    env_logger::init();

    let input = read_data(1);
    let subtotals: Vec<i64> = calorie_subtotals(parse_calories(&input)).collect();

    let part1: i64 = sum_top_n(&subtotals, 1);
    println!("[p1] Most calories: {part1:?}");

    let part2: i64 = sum_top_n(&subtotals, 3);
    println!("[p2] Sum of top 3 calories: {part2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc22::read_test_data;

    #[test]
    fn test_day1_parses() {
        let input = "1000\n2000\n\n3000\n\n";
        itertools::assert_equal(
            parse_calories(&input),
            [Some(1000), Some(2000), None, Some(3000), None],
        );
    }

    #[test]
    fn test_day1_subtotals() {
        let input = [Some(1000i64), Some(2000), None, Some(3000), None];
        itertools::assert_equal(
            calorie_subtotals(input.into_iter()),
            [3000i64, 3000].into_iter(),
        );
    }

    #[test]
    fn test_day1_top_n() {
        let input = [3000, 1000, -6000, 2000];
        assert_eq!(sum_top_n(&input, 1), 3000);
        assert_eq!(sum_top_n(&input, 2), 5000);
        assert_eq!(sum_top_n(&input, 3), 6000);
        assert_eq!(sum_top_n(&input, 4), 0);
    }

    #[test]
    fn test_day1_example() {
        let subtotals: Vec<i64> = calorie_subtotals(parse_calories(&read_test_data(1))).collect();

        assert_eq!(sum_top_n(&subtotals, 1), 24000);
        assert_eq!(sum_top_n(&subtotals, 3), 45000);
    }
}
