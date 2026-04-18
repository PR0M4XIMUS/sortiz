use super::SortStep;

/// Comb Sort: bubble sort with a shrinking gap.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    let shrink = 1.3f64;
    let mut gap = n;
    let mut sorted = false;

    while !sorted {
        gap = ((gap as f64) / shrink) as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..n.saturating_sub(gap) {
            let j = i + gap;
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, j];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[i] > data[j] {
                data.swap(i, j);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, j];
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
                sorted = false;
            }
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
