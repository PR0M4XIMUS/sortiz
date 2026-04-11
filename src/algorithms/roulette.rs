use super::SortStep;
use rand::Rng;

/// Roulette Sort: goes index by index and spins the remaining unsorted
/// elements like a slot-machine reel — rotating them one position at a time
/// so you can see every bar scroll past the spotlight. After the reel slows
/// down it checks the value at the current position. A few spins are rigged
/// to land on the wrong value for dramatic effect; the final spin always
/// wins, so the algorithm always terminates.
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

    // Rotate data[i..] right by one position (last element wraps to front).
    // This is what creates the slot-machine scrolling effect.
    let rotate_once = |data: &mut Vec<usize>, i: usize| {
        data[i..].rotate_right(1);
    };

    for i in 0..n {
        let remaining = n - i;

        // ── Fake failing spins ────────────────────────────────────────────────
        let fake_spins: usize = if remaining > 1 { rng.gen_range(1..=4) } else { 0 };

        for _ in 0..fake_spins {
            // Spin the reel: rotate by a random number of steps (at least a
            // full cycle so every bar gets a turn in the spotlight).
            let rotations = rng.gen_range(remaining..=remaining * 2 + 4);

            for _ in 0..rotations {
                if remaining > 1 {
                    rotate_once(&mut data, i);
                }
                let mut step = SortStep::new(data.clone());
                step.swapping = (i..n).collect();   // whole reel spinning = swapping color
                step.sorted = confirmed.clone();
                steps.push(step);
            }

            // Rig the result: if the correct value accidentally landed at i,
            // rotate one more time to move it away.
            if data[i] == sorted_target[i] && remaining > 1 {
                rotate_once(&mut data, i);
                let mut step = SortStep::new(data.clone());
                step.swapping = (i..n).collect();
                step.sorted = confirmed.clone();
                steps.push(step);
            }

            // Brief pause on the (wrong) result — highlight the checked slot.
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i];
            step.sorted = confirmed.clone();
            steps.push(step.clone());
            // Emit the same frame twice so the pause reads as a "no" moment.
            steps.push(step);
        }

        // ── Winning spin ──────────────────────────────────────────────────────
        // Rotate until sorted_target[i] is at position i, then slow-spin a
        // short extra burst so the win feels earned, not instant.
        if remaining > 1 {
            debug_assert!(
                data[i..].contains(&sorted_target[i]),
                "roulette: sorted_target[{i}]={} not found in remaining slice — data and sorted_target have diverged",
                sorted_target[i]
            );
            // Spin until the correct value is one step away from the front.
            for _ in 0..remaining * 2 {
                // Check if rotating once more would land the winner.
                let next_front = data[n - 1];
                if next_front == sorted_target[i] {
                    break;
                }
                rotate_once(&mut data, i);
                let mut step = SortStep::new(data.clone());
                step.swapping = (i..n).collect();
                step.sorted = confirmed.clone();
                steps.push(step);
            }
            // Final rotation — the winner clicks into place.
            rotate_once(&mut data, i);
        }

        // Show the winning result spinning frame.
        let mut step = SortStep::new(data.clone());
        step.swapping = (i..n).collect();
        step.sorted = confirmed.clone();
        steps.push(step);

        // Lock in the winner — a few held frames so it feels satisfying.
        confirmed.push(i);
        let mut win_step = SortStep::new(data.clone());
        win_step.comparing = vec![i];
        win_step.sorted = confirmed.clone();
        steps.push(win_step.clone());
        steps.push(win_step.clone());
        steps.push(win_step);
    }

    // Final frame: everything sorted.
    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);

    steps
}
