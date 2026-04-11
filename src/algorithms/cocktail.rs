use super::SortStep;

/// Cocktail Shaker Sort: a bidirectional bubble sort that sweeps left→right
/// then right→left each pass. The sorted region closes in from both ends
/// simultaneously, which looks very satisfying.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    let mut lo = 0usize;
    let mut hi = n.saturating_sub(1);
    let mut sorted_indices: Vec<usize> = Vec::new();

    while lo < hi {
        // ── Left → right pass ─────────────────────────────────────────────
        for i in lo..hi {
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, i + 1];
            step.sorted = sorted_indices.clone();
            steps.push(step);

            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, i + 1];
                step.sorted = sorted_indices.clone();
                steps.push(step);
            }
        }
        sorted_indices.push(hi);
        hi = hi.saturating_sub(1);

        if lo >= hi {
            break;
        }

        // ── Right → left pass ─────────────────────────────────────────────
        for i in (lo..hi).rev() {
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, i + 1];
            step.sorted = sorted_indices.clone();
            steps.push(step);

            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, i + 1];
                step.sorted = sorted_indices.clone();
                steps.push(step);
            }
        }
        sorted_indices.push(lo);
        lo += 1;
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}
