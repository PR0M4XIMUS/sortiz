use super::{SortStep, MAX_STEPS};
use rand::seq::SliceRandom;

/// Bogo Sort: randomly shuffles until sorted.
/// Naturally terminates for n ≤ 8; for larger n it hits the step cap and
/// emits a sorted final frame so the visualizer always ends in a valid state.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let n = initial.len();
    let mut steps: Vec<SortStep> = Vec::new();

    if n == 0 {
        steps.push(SortStep::new(vec![]));
        return steps;
    }

    let mut data = initial.to_vec();
    let mut rng = rand::thread_rng();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    steps.push(SortStep::new(data.clone()));

    // Budget: give n.min(8) elements enough attempts to plausibly sort.
    let effective = n.min(8);
    let max_attempts = (MAX_STEPS / effective.max(1)).max(1);
    let mut attempts = 0;

    loop {
        cmp += n.saturating_sub(1) as u32;
        if data.windows(2).all(|w| w[0] <= w[1]) { break; }

        attempts += 1;
        if attempts >= max_attempts { break; }

        data.shuffle(&mut rng);
        swp += n as u32;

        let mut step = SortStep::new(data.clone());
        step.swapping = (0..n).collect();
        step.comparisons = cmp;
        step.swaps = swp;
        steps.push(step);
    }

    // Guarantee final frame is sorted (handles the cap-hit case for large n).
    data.sort_unstable();

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);

    steps
}
