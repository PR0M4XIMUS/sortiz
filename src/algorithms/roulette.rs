use super::SortStep;
use rand::seq::SliceRandom;
use rand::Rng;

/// Roulette Sort: goes index by index, spinning (shuffling) the remaining
/// unsorted elements. After each spin it checks whether the right value
/// landed at the current position. A few spins are deliberately rigged to
/// fail for dramatic effect; the final spin always succeeds, so the
/// algorithm always terminates.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    if n == 0 {
        steps.push(SortStep::new(data));
        return steps;
    }

    let mut sorted_target = data.clone();
    sorted_target.sort_unstable();

    let mut rng = rand::thread_rng();
    let mut confirmed: Vec<usize> = Vec::new();

    for i in 0..n {
        // How many fake failing spins before the guaranteed success
        let fake_spins: usize = if n - i > 1 { rng.gen_range(1..=4) } else { 0 };

        // --- Fake (failed) spins ---
        for _ in 0..fake_spins {
            // Shuffle data[i..] randomly, but make sure sorted_target[i]
            // does NOT end up at position i (rig it to fail).
            data[i..].shuffle(&mut rng);

            // If the correct value accidentally landed at i, swap it away
            if data[i] == sorted_target[i] && n - i > 1 {
                // Find another position to put it
                let swap_to = rng.gen_range(i + 1..n);
                data.swap(i, swap_to);
            }

            // Emit a "spin" frame: show all unsorted elements as swapping
            let mut step = SortStep::new(data.clone());
            step.swapping = (i..n).collect();
            step.sorted = confirmed.clone();
            steps.push(step);

            // Emit a "checking" frame: highlight the position being checked
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i];
            step.sorted = confirmed.clone();
            steps.push(step);
        }

        // --- Guaranteed successful spin ---
        // Find where sorted_target[i] currently sits in data[i..] and swap it to i
        if let Some(pos) = data[i..].iter().position(|&v| v == sorted_target[i]) {
            let actual_pos = i + pos;
            if actual_pos != i {
                data.swap(i, actual_pos);
            }
        }

        // Emit the winning spin frame
        let mut step = SortStep::new(data.clone());
        step.swapping = (i..n).collect();
        step.sorted = confirmed.clone();
        steps.push(step);

        // Lock in the winner
        confirmed.push(i);
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![i];
        step.sorted = confirmed.clone();
        steps.push(step);
    }

    // Final frame: everything sorted
    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);

    steps
}
