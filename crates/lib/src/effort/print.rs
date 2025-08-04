use std::{collections::HashMap, sync::LazyLock};

struct PrintEffort {
    c: char,
    turns: u8,
    strokes: u8,
}

const PRINT_EFFORTS: [PrintEffort; 95] = [
    PrintEffort {
        c: ' ',
        turns: 0,
        strokes: 0,
    },
    PrintEffort {
        c: '!',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: '"',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: '#',
        turns: 0,
        strokes: 4,
    },
    PrintEffort {
        c: '$',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: '%',
        turns: 0,
        strokes: 3,
    },
    PrintEffort {
        c: '&',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: '(',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: ')',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '*',
        turns: 0,
        strokes: 3,
    },
    PrintEffort {
        c: '+',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: ',',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '-',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '.',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '/',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '0',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        // depends
        c: '1',
        turns: 0,
        strokes: 1, // maybe 2 or 3, depending how you print it
    },
    PrintEffort {
        c: '2',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '3',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '4',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: '5',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: '6',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '7',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '8',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '9',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: ':',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: ';',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: '<',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '=',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: '>',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '?',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: '@',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'A',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: 'B',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: 'C',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'D',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        // depends
        c: 'E',
        turns: 2,
        strokes: 2,
    },
    PrintEffort {
        c: 'F',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: 'G',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'H',
        turns: 0,
        strokes: 3,
    },
    PrintEffort {
        c: 'I',
        turns: 0,
        strokes: 3,
    },
    PrintEffort {
        c: 'J',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'K',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: 'L',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'M',
        turns: 3,
        strokes: 1,
    },
    PrintEffort {
        c: 'N',
        turns: 2,
        strokes: 1,
    },
    PrintEffort {
        c: 'O',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'P',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'Q',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'R',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: 'S',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'T',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'U',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'V',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'W',
        turns: 3,
        strokes: 1,
    },
    PrintEffort {
        c: 'X',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'Y',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'Z',
        turns: 2,
        strokes: 1,
    },
    PrintEffort {
        c: '[',
        turns: 2,
        strokes: 1,
    },
    PrintEffort {
        c: '\'',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '\\',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: ']',
        turns: 2,
        strokes: 1,
    },
    PrintEffort {
        c: '^',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '_',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '`',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'a',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'b',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'c',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'd',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'e',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'f',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'g',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'h',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'i',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'j',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'k',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: 'l',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'm',
        turns: 2,
        strokes: 1,
    },
    PrintEffort {
        c: 'n',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'o',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 'p',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'q',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'r',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 's',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: 't',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'u',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'v',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: 'w',
        turns: 3,
        strokes: 1,
    },
    PrintEffort {
        c: 'x',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'y',
        turns: 0,
        strokes: 2,
    },
    PrintEffort {
        c: 'z',
        turns: 2,
        strokes: 1,
    },
    PrintEffort {
        c: '{',
        turns: 1,
        strokes: 1,
    },
    PrintEffort {
        c: '|',
        turns: 0,
        strokes: 1,
    },
    PrintEffort {
        c: '}',
        turns: 1,
        strokes: 2,
    },
    PrintEffort {
        c: '~',
        turns: 0,
        strokes: 1,
    },
];

const fn one_char_effort(e: &PrintEffort) -> u64 {
    2 * e.strokes as u64 + e.turns as u64
}

const PRECOMPUTED_EFFORT_ARRAY: [(char, u64); 95] = {
    let mut result: [(char, u64); 95] = [(' ', 0); 95];
    let mut idx = 0;
    while idx < 95 {
        let k = &PRINT_EFFORTS[idx];
        result[idx] = (k.c, one_char_effort(k));
        idx += 1;
    }
    result
};

pub static PRECOMPUTED_EFFORTS: LazyLock<HashMap<char, u64>> =
    LazyLock::new(|| HashMap::from(PRECOMPUTED_EFFORT_ARRAY));

// ref: print-effort
/// Compute the print effort
///
/// # Panics
///
/// If passed an invalid character
#[must_use]
pub fn print_effort(s: &str) -> u64 {
    if s.chars().all(|c| c == ' ') {
        return 1;
    }
    s.chars()
        .map(|c| {
            if c.is_whitespace() {
                1
            } else {
                *PRECOMPUTED_EFFORTS
                    .get(&c)
                    .unwrap_or_else(|| panic!("Invalid character for `print_effort`: {c}"))
            }
        })
        .sum()
}

// ref: print-effort
#[must_use]
pub fn print_effort_fill(s: &str, fill: u64) -> u64 {
    if s.chars().all(|c| c == ' ') {
        return 1;
    }
    s.chars()
        .map(|c| {
            if c.is_whitespace() {
                1
            } else {
                *PRECOMPUTED_EFFORTS.get(&c).unwrap_or(&fill)
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::print_effort;

    fn e(s: &str) -> u64 {
        print_effort(s)
    }

    #[test]
    fn test_print_effort_single_char() {
        // ref: print-effort-examples
        assert_eq!(e(" "), 1);
        assert_eq!(e("f"), 4);
        assert_eq!(e("["), 4);
        assert_eq!(e("F"), 5);
        assert_eq!(e(":"), 4);
        assert_eq!(e("@"), 3);
        assert_eq!(e("~"), 2);
        assert_eq!(e("/"), 2);
        assert_eq!(e("<"), 3);
        assert_eq!(e("m"), 4);
    }
}
