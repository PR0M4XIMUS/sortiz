use crate::algorithms::{all_algorithms, SortStep, MAX_STEPS};
use crate::audio::Player;
use crate::config::ParsedAudio;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Distribution {
    Uniform,
    Reversed,
    NearlySorted,
    FewUnique,
    Sawtooth,
    Sorted,
    WorstCase,
}

impl Distribution {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "uniform"       => Some(Self::Uniform),
            "reversed"      => Some(Self::Reversed),
            "nearly-sorted" => Some(Self::NearlySorted),
            "few-unique"    => Some(Self::FewUnique),
            "sawtooth"      => Some(Self::Sawtooth),
            "sorted"        => Some(Self::Sorted),
            "worst-case"    => Some(Self::WorstCase),
            _ => None,
        }
    }
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Uniform      => "uniform",
            Self::Reversed     => "reversed",
            Self::NearlySorted => "nearly-sorted",
            Self::FewUnique    => "few-unique",
            Self::Sawtooth     => "sawtooth",
            Self::Sorted       => "sorted",
            Self::WorstCase    => "worst-case",
        }
    }
}

pub struct App {
    pub algorithm_name:  String,
    pub algorithm_key:   String,
    pub complexity:      &'static str,
    pub steps:           Vec<SortStep>,
    pub step_idx:        usize,
    pub paused:          bool,
    pub speed_ms:        u64,
    pub array_size:      usize,
    pub done_at:         Option<Instant>,
    pub loop_mode:       bool,
    pub show_help:       bool,
    pub show_summary:    bool,
    pub summary_shown:   bool,
    pub seed:            u64,
    pub distribution:    Distribution,
    pub sort_start_time: Option<Instant>,
    pub sort_elapsed_ms: Option<u64>,
    algo_idx:    usize,
    fixed_algo:  bool,
    player:      Player,
}

impl App {
    pub fn new(
        array_size: usize,
        speed_ms: u64,
        algorithm: Option<&str>,
        loop_mode: bool,
        seed: Option<u64>,
        distribution: Distribution,
        audio_cfg: &ParsedAudio,
    ) -> Self {
        let algos = all_algorithms();
        let mut rng_thread = rand::thread_rng();

        let seed = seed.unwrap_or_else(|| rng_thread.gen());

        let algo_idx = if let Some(key) = algorithm {
            algos.iter().position(|a| a.key == key).unwrap_or_else(|| {
                eprintln!("Unknown algorithm '{}', defaulting to bubble sort.", key);
                0
            })
        } else {
            rng_thread.gen_range(0..algos.len())
        };

        let data = build_array(array_size, seed, distribution, algos[algo_idx].key);
        let mut steps = (algos[algo_idx].generate_steps)(&data);
        if steps.len() > MAX_STEPS {
            eprintln!(
                "Warning: {} steps truncated to {}. Try a smaller array size.",
                steps.len(), MAX_STEPS
            );
            steps.truncate(MAX_STEPS);
        }

        let player = Player::new(audio_cfg);

        App {
            algorithm_name:  algos[algo_idx].name.to_string(),
            algorithm_key:   algos[algo_idx].key.to_string(),
            complexity:      algos[algo_idx].complexity,
            steps,
            step_idx:        0,
            paused:          false,
            speed_ms,
            array_size,
            done_at:         None,
            loop_mode:       loop_mode || algorithm.is_none(),
            show_help:       false,
            show_summary:    false,
            summary_shown:   false,
            seed,
            distribution,
            sort_start_time: Some(Instant::now()),
            sort_elapsed_ms: None,
            algo_idx,
            fixed_algo:      algorithm.is_some(),
            player,
        }
    }

    pub fn current_step(&self) -> &SortStep {
        debug_assert!(!self.steps.is_empty(), "steps must never be empty");
        let idx = self.step_idx.min(self.steps.len().saturating_sub(1));
        &self.steps[idx]
    }

    pub fn advance(&mut self) {
        if self.step_idx + 1 < self.steps.len() {
            self.step_idx += 1;
            let done = self.step_idx + 1 >= self.steps.len();
            if done && self.done_at.is_none() {
                self.done_at = Some(Instant::now());
                if let Some(start) = self.sort_start_time {
                    self.sort_elapsed_ms = Some(start.elapsed().as_millis() as u64);
                }
            }
            let step = &self.steps[self.step_idx];
            let n = self.array_size;
            let sp = self.speed_ms;
            self.player.play_step(step, n, sp, done);
        } else if let Some(done_at) = self.done_at {
            if self.loop_mode && done_at.elapsed() >= Duration::from_millis(1500) && !self.show_summary {
                self.next_algorithm();
            }
        }
    }

    pub fn step_back(&mut self) {
        if self.step_idx > 0 {
            self.step_idx -= 1;
            if self.done_at.is_some() {
                // Stepped back out of done state
                self.done_at = None;
            }
        }
    }

    pub fn step_forward(&mut self) {
        if self.step_idx + 1 < self.steps.len() {
            self.step_idx += 1;
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
        let new_seed: u64 = rng.gen();
        self.load_algorithm(new_idx, new_seed);
    }

    pub fn next_algorithm_sequential(&mut self) {
        let algos = all_algorithms();
        let idx = (self.algo_idx + 1) % algos.len();
        let seed = rand::thread_rng().gen();
        self.load_algorithm(idx, seed);
    }

    pub fn prev_algorithm_sequential(&mut self) {
        let algos = all_algorithms();
        let idx = if self.algo_idx == 0 { algos.len() - 1 } else { self.algo_idx - 1 };
        let seed = rand::thread_rng().gen();
        self.load_algorithm(idx, seed);
    }

    pub fn restart(&mut self) {
        let idx = self.algo_idx;
        self.load_algorithm(idx, self.seed);
    }

    pub fn restart_new_seed(&mut self) {
        let idx = self.algo_idx;
        let seed: u64 = rand::thread_rng().gen();
        self.load_algorithm(idx, seed);
    }

    pub fn speed_up(&mut self) {
        self.speed_ms = (self.speed_ms / 2).max(5);
    }

    pub fn speed_down(&mut self) {
        self.speed_ms = (self.speed_ms * 2).min(5000);
    }

    pub fn speed_inc(&mut self) {
        self.speed_ms = self.speed_ms.saturating_sub(10).max(5);
    }

    pub fn speed_dec(&mut self) {
        self.speed_ms = (self.speed_ms + 10).min(5000);
    }

    pub fn toggle_mute(&mut self) {
        self.player.toggle_mute();
    }

    pub fn is_muted(&self) -> bool {
        self.player.is_muted()
    }

    pub fn is_done(&self) -> bool {
        self.step_idx + 1 >= self.steps.len()
    }

    pub fn progress(&self) -> (usize, usize) {
        (self.step_idx + 1, self.steps.len())
    }

    fn load_algorithm(&mut self, idx: usize, seed: u64) {
        let algos = all_algorithms();
        let data = build_array(self.array_size, seed, self.distribution, algos[idx].key);
        let mut steps = (algos[idx].generate_steps)(&data);
        if steps.len() > MAX_STEPS {
            eprintln!(
                "Warning: {} steps truncated to {}.",
                steps.len(), MAX_STEPS
            );
            steps.truncate(MAX_STEPS);
        }
        self.algo_idx       = idx;
        self.algorithm_name = algos[idx].name.to_string();
        self.algorithm_key  = algos[idx].key.to_string();
        self.complexity     = algos[idx].complexity;
        self.steps          = steps;
        self.step_idx       = 0;
        self.done_at        = None;
        self.show_summary   = false;
        self.summary_shown  = false;
        self.seed           = seed;
        self.sort_start_time = Some(Instant::now());
        self.sort_elapsed_ms = None;
    }
}

// ── Array generation ──────────────────────────────────────────────────────────

pub fn build_array(size: usize, seed: u64, dist: Distribution, algo_key: &str) -> Vec<usize> {
    if size == 0 { return vec![]; }
    let mut rng = StdRng::seed_from_u64(seed);

    match dist {
        Distribution::Uniform => {
            let mut arr: Vec<usize> = (1..=size).collect();
            arr.shuffle(&mut rng);
            arr
        }
        Distribution::Reversed => (1..=size).rev().collect(),
        Distribution::Sorted   => (1..=size).collect(),
        Distribution::NearlySorted => {
            let mut arr: Vec<usize> = (1..=size).collect();
            let swaps = (size / 10).max(1);
            for _ in 0..swaps {
                let a = rng.gen_range(0..size);
                let b = rng.gen_range(0..size);
                arr.swap(a, b);
            }
            arr
        }
        Distribution::FewUnique => {
            let unique = (size / 5).max(2).min(size);
            (0..size).map(|i| (i % unique) + 1).collect()
        }
        Distribution::Sawtooth => {
            let period = (size / 4).max(2);
            (1..=size).map(|i| ((i - 1) % period) + 1).collect()
        }
        Distribution::WorstCase => worst_case(size, algo_key, &mut rng),
    }
}

fn worst_case(size: usize, algo_key: &str, rng: &mut StdRng) -> Vec<usize> {
    match algo_key {
        // Quicksort worst case: already sorted (last-element pivot)
        "quick" => (1..=size).collect(),
        // Bubble / insertion / gnome worst case: reversed
        "bubble" | "insertion" | "gnome" | "cocktail" => (1..=size).rev().collect(),
        // Bogo worst case: reversed (maximises shuffle attempts relative to n)
        "bogo" => (1..=size).rev().collect(),
        // Default: random shuffle
        _ => {
            let mut arr: Vec<usize> = (1..=size).collect();
            arr.shuffle(rng);
            arr
        }
    }
}

// ── Race-mode snapshot ────────────────────────────────────────────────────────

pub struct RaceAlgo {
    pub name:      &'static str,
    pub key:       &'static str,
    pub complexity: &'static str,
    pub steps:     Vec<SortStep>,
    pub step_idx:  usize,
}

impl RaceAlgo {
    pub fn current_step(&self) -> &SortStep {
        let idx = self.step_idx.min(self.steps.len().saturating_sub(1));
        &self.steps[idx]
    }

    pub fn is_done(&self) -> bool {
        self.step_idx + 1 >= self.steps.len()
    }
}

pub struct RaceApp {
    pub racers:   Vec<RaceAlgo>,
    pub array_size: usize,
    pub speed_ms: u64,
    pub paused:   bool,
}

impl RaceApp {
    pub fn new(array_size: usize, speed_ms: u64, seed: u64, distribution: Distribution) -> Self {
        let algos = all_algorithms();
        let data = build_array(array_size, seed, distribution, "uniform");
        let racers = algos.into_iter().map(|a| {
            let mut steps = (a.generate_steps)(&data);
            steps.truncate(MAX_STEPS);
            RaceAlgo {
                name: a.name,
                key: a.key,
                complexity: a.complexity,
                steps,
                step_idx: 0,
            }
        }).collect();

        RaceApp { racers, array_size, speed_ms, paused: false }
    }

    pub fn advance(&mut self) {
        for r in &mut self.racers {
            if r.step_idx + 1 < r.steps.len() {
                r.step_idx += 1;
            }
        }
    }

    pub fn all_done(&self) -> bool {
        self.racers.iter().all(|r| r.is_done())
    }

    pub fn speed_up(&mut self) { self.speed_ms = (self.speed_ms / 2).max(5); }
    pub fn speed_down(&mut self) { self.speed_ms = (self.speed_ms * 2).min(5000); }
}
