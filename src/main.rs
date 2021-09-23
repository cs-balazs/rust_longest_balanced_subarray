fn identify_subvector(input: &str) -> Option<&str> {
    match input {
        "" => None,
        input => {
            match input.chars().fold(
                0_isize,
                |acc, ch| if ch.is_alphabetic() { acc + 1 } else { acc - 1 },
            ) {
                0 => Some(input),
                _ => {
                    let left_subvector = identify_subvector(&input[..input.len() - 1]);
                    let right_subvector = identify_subvector(&input[1..]);
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

fn main() {
    assert_eq!(None, identify_subvector(""));
    assert_eq!(None, identify_subvector("a"));
    assert_eq!(None, identify_subvector("1"));
    assert_eq!(Some("1a"), identify_subvector("1a"));
    assert_eq!(Some("1a"), identify_subvector("1ab"));
    assert_eq!(Some("ac412b"), identify_subvector("0ac412b023"));
    assert_eq!(Some("dac412b0"), identify_subvector("dac412b02"));
    assert_eq!(Some("b02c"), identify_subvector("12c412b02c"));
    assert_eq!(Some("z12c412bkc"), identify_subvector("z12c412bkc"));
    assert_eq!(Some("g2"), identify_subvector("abfdg2c"));
    assert_eq!(Some("1a"), identify_subvector("1a345c0023e"));
}
