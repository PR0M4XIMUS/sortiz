pub mod bitonic;
pub mod bogo;
pub mod bubble;
pub mod cocktail;
pub mod comb;
pub mod cycle;
pub mod gnome;
pub mod heap;
pub mod insertion;
pub mod intro;
pub mod merge;
pub mod pancake;
pub mod quick;
pub mod radix;
pub mod roulette;
pub mod selection;
pub mod shell;
pub mod sleep;
pub mod stalin;
pub mod tim;

pub const MAX_STEPS: usize = 100_000;

/// One animation frame: full array state + highlighted index sets + cumulative stats.
#[derive(Clone, Debug)]
pub struct SortStep {
    pub data: Vec<usize>,
    pub comparing: Vec<usize>,
    pub swapping: Vec<usize>,
    pub sorted: Vec<usize>,
    /// Cumulative comparisons up to this step.
    pub comparisons: u32,
    /// Cumulative swaps/writes up to this step.
    pub swaps: u32,
}

impl SortStep {
    pub fn new(data: Vec<usize>) -> Self {
        Self {
            data,
            comparing: vec![],
            swapping: vec![],
            sorted: vec![],
            comparisons: 0,
            swaps: 0,
        }
    }

    pub fn with_stats(mut self, cmp: u32, swp: u32) -> Self {
        self.comparisons = cmp;
        self.swaps = swp;
        self
    }
}

pub struct Algorithm {
    pub name: &'static str,
    pub key: &'static str,
    pub complexity: &'static str,
    pub description: &'static str,
    pub generate_steps: fn(&[usize]) -> Vec<SortStep>,
}

pub fn all_algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "Bubble Sort",
            key: "bubble",
            complexity: "O(n²)",
            description: "Repeatedly swaps adjacent out-of-order pairs until sorted.",
            generate_steps: bubble::steps,
        },
        Algorithm {
            name: "Insertion Sort",
            key: "insertion",
            complexity: "O(n²)",
            description: "Builds sorted list by inserting each element leftward into place.",
            generate_steps: insertion::steps,
        },
        Algorithm {
            name: "Selection Sort",
            key: "selection",
            complexity: "O(n²)",
            description: "Finds the minimum of the unsorted portion and swaps it into place.",
            generate_steps: selection::steps,
        },
        Algorithm {
            name: "Merge Sort",
            key: "merge",
            complexity: "O(n log n)",
            description: "Divides array in half, sorts each half, merges them in order.",
            generate_steps: merge::steps,
        },
        Algorithm {
            name: "Quick Sort",
            key: "quick",
            complexity: "O(n log n)",
            description: "Picks a pivot and partitions elements smaller/larger to each side.",
            generate_steps: quick::steps,
        },
        Algorithm {
            name: "Heap Sort",
            key: "heap",
            complexity: "O(n log n)",
            description: "Builds a max-heap, then extracts the maximum repeatedly to the end.",
            generate_steps: heap::steps,
        },
        Algorithm {
            name: "Shell Sort",
            key: "shell",
            complexity: "O(n log² n)",
            description: "Insertion sort with shrinking gap — sorts far-apart elements first.",
            generate_steps: shell::steps,
        },
        Algorithm {
            name: "Roulette Sort",
            key: "roulette",
            complexity: "O(n²)",
            description: "Spins the unsorted reel like a slot machine — rigged to always win.",
            generate_steps: roulette::steps,
        },
        Algorithm {
            name: "Cocktail Sort",
            key: "cocktail",
            complexity: "O(n²)",
            description: "Bidirectional bubble sort — sweeps left→right then right→left.",
            generate_steps: cocktail::steps,
        },
        Algorithm {
            name: "Comb Sort",
            key: "comb",
            complexity: "O(n log n)",
            description: "Bubble sort with a shrinking gap to eliminate turtles early.",
            generate_steps: comb::steps,
        },
        Algorithm {
            name: "Pancake Sort",
            key: "pancake",
            complexity: "O(n²)",
            description: "Sorts by flipping prefixes to bring the max to the front, then drop.",
            generate_steps: pancake::steps,
        },
        Algorithm {
            name: "Gnome Sort",
            key: "gnome",
            complexity: "O(n²)",
            description: "A gnome steps forward, swaps if out of order, then steps back.",
            generate_steps: gnome::steps,
        },
        Algorithm {
            name: "Stalin Sort",
            key: "stalin",
            complexity: "O(n)",
            description: "Exiles non-conforming elements to the tail, then re-integrates them.",
            generate_steps: stalin::steps,
        },
        Algorithm {
            name: "Radix Sort",
            key: "radix",
            complexity: "O(nk)",
            description: "Sorts digit by digit from least to most significant (LSD base-10).",
            generate_steps: radix::steps,
        },
        Algorithm {
            name: "Cycle Sort",
            key: "cycle",
            complexity: "O(n²)",
            description: "Minimizes writes by cycling each element to its final position.",
            generate_steps: cycle::steps,
        },
        Algorithm {
            name: "Bitonic Sort",
            key: "bitonic",
            complexity: "O(n log² n)",
            description: "Comparator network — builds a bitonic sequence then merges it.",
            generate_steps: bitonic::steps,
        },
        Algorithm {
            name: "Tim Sort",
            key: "tim",
            complexity: "O(n log n)",
            description: "Hybrid of insertion sort on small runs + merge. Used in Python/Java.",
            generate_steps: tim::steps,
        },
        Algorithm {
            name: "Intro Sort",
            key: "intro",
            complexity: "O(n log n)",
            description: "Starts quicksort, switches to heapsort when depth limit exceeded.",
            generate_steps: intro::steps,
        },
        Algorithm {
            name: "Sleep Sort",
            key: "sleep",
            complexity: "O(n + max)",
            description: "Each element 'sleeps' proportional to its value before joining output.",
            generate_steps: sleep::steps,
        },
        Algorithm {
            name: "Bogo Sort",
            key: "bogo",
            complexity: "O((n+1)!)",
            description: "Randomly shuffles until sorted. Pray for a short run.",
            generate_steps: bogo::steps,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_sorted(v: &[usize]) -> bool {
        v.windows(2).all(|w| w[0] <= w[1])
    }

    fn final_state(algo: fn(&[usize]) -> Vec<SortStep>, input: &[usize]) -> Vec<usize> {
        algo(input).last().expect("algorithm produced no steps").data.clone()
    }

    const ALGOS: &[(&str, fn(&[usize]) -> Vec<SortStep>)] = &[
        ("bubble",    bubble::steps),
        ("insertion", insertion::steps),
        ("selection", selection::steps),
        ("merge",     merge::steps),
        ("quick",     quick::steps),
        ("heap",      heap::steps),
        ("shell",     shell::steps),
        ("roulette",  roulette::steps),
        ("cocktail",  cocktail::steps),
        ("comb",      comb::steps),
        ("pancake",   pancake::steps),
        ("gnome",     gnome::steps),
        ("stalin",    stalin::steps),
        ("radix",     radix::steps),
        ("cycle",     cycle::steps),
        ("bitonic",   bitonic::steps),
        ("tim",       tim::steps),
        ("intro",     intro::steps),
        ("sleep",     sleep::steps),
        ("bogo",      bogo::steps),
    ];

    #[test]
    fn all_sort_correctly() {
        let cases: &[&[usize]] = &[
            &[],
            &[1],
            &[2, 1],
            &[1, 2],
            &[5, 3, 1, 4, 2],
            &[9, 8, 7, 6, 5, 4, 3, 2, 1],
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            &[3, 1, 4, 1, 5, 9, 2, 6, 5, 3],
            &[1, 1, 1, 1, 1],
        ];

        for &(name, algo) in ALGOS {
            for &case in cases {
                let result = final_state(algo, case);
                let mut expected = case.to_vec();
                expected.sort_unstable();
                assert!(
                    is_sorted(&result) && result == expected,
                    "{name} failed on input {case:?}\n  got:      {result:?}\n  expected: {expected:?}"
                );
            }
        }
    }

    #[test]
    fn steps_never_empty() {
        let cases: &[&[usize]] = &[
            &[],
            &[1],
            &[2, 1],
            &(1usize..=10).rev().collect::<Vec<_>>(),
        ];
        for &(name, algo) in ALGOS {
            for &case in cases {
                let steps = algo(case);
                assert!(!steps.is_empty(), "{name} returned empty steps for input {case:?}");
            }
        }
    }

    #[test]
    fn each_step_has_correct_length() {
        let data: Vec<usize> = vec![4, 2, 7, 1, 9, 3];
        let n = data.len();
        for &(name, algo) in ALGOS {
            for (i, step) in algo(&data).iter().enumerate() {
                assert_eq!(
                    step.data.len(), n,
                    "{name} step {i}: data length changed from {n} to {}",
                    step.data.len()
                );
            }
        }
    }
}
