use super::SortStep;

/// Pancake Sort: sorts by flipping prefixes. Finds the largest unsorted
/// element, flips (reverses) the prefix up to it to bring it to the front,
/// then flips the whole unsorted prefix to drop it at the end. Each flip
/// is animated frame-by-frame so you see every bar reverse in a sweep.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut sorted: Vec<usize> = Vec::new();

    for size in (2..=n).rev() {
        // Find the index of the maximum element in data[0..size]
        let max_idx = data[..size]
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);

        if max_idx == size - 1 {
            // Already in place — just mark it sorted
            sorted.push(size - 1);
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![size - 1];
            step.sorted = sorted.clone();
            steps.push(step);
            continue;
        }

        // ── Flip 1: bring max to front (if not already there) ─────────────
        if max_idx > 0 {
            flip(&mut data, max_idx, &mut steps, &sorted);
        }

        // ── Flip 2: flip the entire unsorted prefix to drop max at end ────
        flip(&mut data, size - 1, &mut steps, &sorted);

        sorted.push(size - 1);

        // Show the newly placed element
        let mut step = SortStep::new(data.clone());
        step.sorted = sorted.clone();
        steps.push(step);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}

/// Reverse data[0..=end], emitting one swapping frame per swap.
fn flip(data: &mut [usize], end: usize, steps: &mut Vec<SortStep>, sorted: &[usize]) {
    let mut lo = 0;
    let mut hi = end;
    while lo < hi {
        data.swap(lo, hi);
        let mut step = SortStep::new(data.to_owned());
        step.swapping = vec![lo, hi];
        step.sorted = sorted.to_vec();
        steps.push(step);
        lo += 1;
        hi -= 1;
    }
}
