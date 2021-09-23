fn helper(balances: Vec<i32>, from: usize, current: (usize, usize)) -> Option<(usize, usize)> {
    let max_index = balances.len() - 1;
    let current_length = current.1 - current.0 + 1;

    if from == max_index || max_index < current_length + from {
        if current.0 == 0 && current.1 == 0 {
            return None;
        }
        return Some((current.0, current.1));
    }

    let rightmost_zero_position = balances[from..].iter().rposition(|n| *n == 0);

    match rightmost_zero_position {
        Some(index) => helper(
            balances.iter().map(|n| n - balances[from]).collect(),
            from + 1,
            if index + 1 > current_length {
                (from, from + index + 1)
            } else {
                current
            },
        ),
        None => helper(
            balances.iter().map(|n| n - balances[from]).collect(),
            from + 1,
            current,
        ),
    }
}

fn identify_subvector(input: &str) -> Option<&str> {
    if input.len() == 0 {
        return None;
    }

    let indexes = helper(
        input
            .chars()
            .scan(0, |state, char| {
                *state += if char.is_alphabetic() { -1 } else { 1 };
                return Some(*state);
            })
            .collect(),
        0,
        (0, 0),
    );
    match indexes {
        None => None,
        Some((from, to)) => Some(&input[from..to]),
    }
}

fn main() {
    assert_eq!(Some("b02c"), identify_subvector("12c412b02c"));
    assert_eq!(None, identify_subvector(""));
    assert_eq!(None, identify_subvector("a"));
    assert_eq!(None, identify_subvector("1"));
    assert_eq!(Some("1a"), identify_subvector("1a"));
    assert_eq!(Some("1a"), identify_subvector("1ab"));
    assert_eq!(Some("ac412b"), identify_subvector("0ac412b023"));
    assert_eq!(Some("dac412b0"), identify_subvector("dac412b02"));
    assert_eq!(Some("z12c412bkc"), identify_subvector("z12c412bkc"));
    assert_eq!(Some("g2"), identify_subvector("abfdg2c"));
    assert_eq!(Some("1a"), identify_subvector("1a345c0023e"));
    assert_eq!(
        Some("aaaaaaaaaaaaaaaaaaaaa111111111111111111111"),
        identify_subvector("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa111111111111111111111aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    );
}

fn slow_recursive_solution(input: &str) -> Option<&str> {
    match input {
        "" => None,
        input => {
            match input.chars().fold(
                0_isize,
                |acc, ch| if ch.is_alphabetic() { acc + 1 } else { acc - 1 },
            ) {
                0 => Some(input),
                _ => {
                    let left_subvector = slow_recursive_solution(&input[..input.len() - 1]);
                    let right_subvector = slow_recursive_solution(&input[1..]);
                    match [left_subvector, right_subvector] {
                        [Some(left), Some(right)] => Some(if right.len() > left.len() {
                            right
                        } else {
                            left
                        }),
                        [Some(left), None] => Some(left),
                        [None, Some(right)] => Some(right),
                        [None, None] => return None,
                    }
                }
            }
        }
    }
}
