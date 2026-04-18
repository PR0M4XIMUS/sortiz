use super::SortStep;

/// Cocktail Shaker Sort: bidirectional bubble sort.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    let mut lo = 0usize;
    let mut hi = n.saturating_sub(1);
    let mut sorted_indices: Vec<usize> = Vec::new();

    while lo < hi {
        for i in lo..hi {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, i + 1];
            step.sorted = sorted_indices.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, i + 1];
                step.sorted = sorted_indices.clone();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }
        }
        sorted_indices.push(hi);
        hi = hi.saturating_sub(1);

        if lo >= hi { break; }

        for i in (lo..hi).rev() {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, i + 1];
            step.sorted = sorted_indices.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, i + 1];
                step.sorted = sorted_indices.clone();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }
        }
        sorted_indices.push(lo);
        lo += 1;
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
