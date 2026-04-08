use crate::algorithms::{all_algorithms, SortStep};
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct App {
    pub algorithm_name: String,
    pub steps: Vec<SortStep>,
    pub step_idx: usize,
    pub paused: bool,
    pub speed_ms: u64,
    pub array_size: usize,
    /// Set when the last step is reached; used to auto-advance in loop mode.
    pub done_at: Option<Instant>,
    pub loop_mode: bool,
    /// Index into all_algorithms(). Kept so next_algorithm() can avoid repeating.
    algo_idx: usize,
    /// When true, loop_mode restarts the same algorithm with a new array
    /// (used when --algorithm is combined with --loop).
    fixed_algo: bool,
}

impl App {
    pub fn new(
        array_size: usize,
        speed_ms: u64,
        algorithm: Option<&str>,
        loop_mode: bool,
    ) -> Self {
        let algos = all_algorithms();
        let mut rng = rand::thread_rng();

        let algo_idx = if let Some(key) = algorithm {
            algos.iter().position(|a| a.key == key).unwrap_or_else(|| {
                eprintln!("Unknown algorithm '{}', defaulting to bubble sort.", key);
                0
            })
        } else {
            rng.gen_range(0..algos.len())
        };

        const MAX_STEPS: usize = 100_000;
        let data = make_array(array_size);
        let mut steps = (algos[algo_idx].generate_steps)(&data);
        if steps.len() > MAX_STEPS {
            eprintln!(
                "Warning: {} steps truncated to {} to avoid excessive memory use. \
                 Try a smaller array size or a faster algorithm.",
                steps.len(),
                MAX_STEPS
            );
            steps.truncate(MAX_STEPS);
        }

        App {
            algorithm_name: algos[algo_idx].name.to_string(),
            steps,
            step_idx: 0,
            paused: false,
            speed_ms,
            array_size,
            done_at: None,
            loop_mode: loop_mode || algorithm.is_none(),
            algo_idx,
            fixed_algo: algorithm.is_some(),
        }
    }

    pub fn current_step(&self) -> &SortStep {
        debug_assert!(!self.steps.is_empty(), "steps must never be empty");
        let idx = self.step_idx.min(self.steps.len().saturating_sub(1));
        &self.steps[idx]
    }

    /// Advance one animation frame. In loop mode, automatically transitions
    /// to the next algorithm after a short pause when sorting completes.
    pub fn advance(&mut self) {
        if self.step_idx + 1 < self.steps.len() {
            self.step_idx += 1;
            if self.step_idx + 1 >= self.steps.len() {
                self.done_at = Some(Instant::now());
            }
        } else if let Some(done_at) = self.done_at {
            if self.loop_mode && done_at.elapsed() >= Duration::from_millis(1500) {
                self.next_algorithm();
            }
        }
    }

    pub fn next_algorithm(&mut self) {
        let algos = all_algorithms();
        let mut rng = rand::thread_rng();

        let new_idx = if self.fixed_algo || algos.len() == 1 {
            self.algo_idx
        } else {
            let mut idx = rng.gen_range(0..algos.len());
            while idx == self.algo_idx {
                idx = rng.gen_range(0..algos.len());
            }
            idx
        };

        self.load_algorithm(new_idx);
    }

    /// Restart the current algorithm with a freshly shuffled array.
    pub fn restart(&mut self) {
        let idx = self.algo_idx;
        self.load_algorithm(idx);
    }

    pub fn speed_up(&mut self) {
        self.speed_ms = (self.speed_ms / 2).max(5);
    }

    pub fn speed_down(&mut self) {
        self.speed_ms = (self.speed_ms * 2).min(5000);
    }

    pub fn is_done(&self) -> bool {
        self.step_idx + 1 >= self.steps.len()
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.step_idx + 1, self.steps.len())
    }

    fn load_algorithm(&mut self, idx: usize) {
        const MAX_STEPS: usize = 100_000;
        let algos = all_algorithms();
        let data = make_array(self.array_size);
        let mut steps = (algos[idx].generate_steps)(&data);
        if steps.len() > MAX_STEPS {
            eprintln!(
                "Warning: {} steps truncated to {} to avoid excessive memory use. \
                 Try a smaller array size or a faster algorithm.",
                steps.len(),
                MAX_STEPS
            );
            steps.truncate(MAX_STEPS);
        }
        self.algo_idx = idx;
        self.algorithm_name = algos[idx].name.to_string();
        self.steps = steps;
        self.step_idx = 0;
        self.done_at = None;
    }
}

fn make_array(size: usize) -> Vec<usize> {
    let mut arr: Vec<usize> = (1..=size).collect();
    let mut rng = rand::thread_rng();
    arr.shuffle(&mut rng);
    arr
}
