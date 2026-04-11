pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod roulette;
pub mod selection;
pub mod shell;

/// Represents one frame of animation: the current array state plus
/// which indices are being compared, swapped, or confirmed sorted.
#[derive(Clone, Debug)]
pub struct SortStep {
    pub data: Vec<usize>,
    pub comparing: Vec<usize>,
    pub swapping: Vec<usize>,
    pub sorted: Vec<usize>,
}

impl SortStep {
    pub fn new(data: Vec<usize>) -> Self {
        Self {
            data,
            comparing: vec![],
            swapping: vec![],
            sorted: vec![],
        }
    }
}

/// Descriptor for a sorting algorithm. Add new algorithms here to register them.
pub struct Algorithm {
    pub name: &'static str,
    /// Short lowercase key used for CLI --algorithm flag
    pub key: &'static str,
    pub generate_steps: fn(&[usize]) -> Vec<SortStep>,
}

/// Central registry of all available algorithms.
/// To add a new algorithm: create a module in this directory, implement a
/// `steps(initial: &[usize]) -> Vec<SortStep>` function, then add an entry here.
pub fn all_algorithms() -> Vec<Algorithm> {
    vec![
        Algorithm {
            name: "Bubble Sort",
            key: "bubble",
            generate_steps: bubble::steps,
        },
        Algorithm {
            name: "Insertion Sort",
            key: "insertion",
            generate_steps: insertion::steps,
        },
        Algorithm {
            name: "Selection Sort",
            key: "selection",
            generate_steps: selection::steps,
        },
        Algorithm {
            name: "Merge Sort",
            key: "merge",
            generate_steps: merge::steps,
        },
        Algorithm {
            name: "Quick Sort",
            key: "quick",
            generate_steps: quick::steps,
        },
        Algorithm {
            name: "Heap Sort",
            key: "heap",
            generate_steps: heap::steps,
        },
        Algorithm {
            name: "Shell Sort",
            key: "shell",
            generate_steps: shell::steps,
        },
        Algorithm {
            name: "Roulette Sort",
            key: "roulette",
            generate_steps: roulette::steps,
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
    ];

    #[test]
    fn all_sort_correctly() {
        let cases: &[&[usize]] = &[
            &[],                               // empty
            &[1],                              // single element
            &[2, 1],                           // two elements reversed
            &[1, 2],                           // two elements already sorted
            &[5, 3, 1, 4, 2],                 // small shuffled
            &[9, 8, 7, 6, 5, 4, 3, 2, 1],    // fully reversed
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],    // already sorted
            &[3, 1, 4, 1, 5, 9, 2, 6, 5, 3], // duplicates
            &[1, 1, 1, 1, 1],                 // all equal
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
        let data: Vec<usize> = (1..=10).rev().collect();
        for &(name, algo) in ALGOS {
            let steps = algo(&data);
            assert!(!steps.is_empty(), "{name} returned empty steps");
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
