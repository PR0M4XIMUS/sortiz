pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
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
    ]
}
