use super::SortStep;

/// Gnome Sort: a garden gnome wanders forward and back, swapping when needed.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;
    let mut pos = 0usize;

    while pos < n {
        if pos == 0 || data[pos] >= data[pos - 1] {
            if pos > 0 {
                cmp += 1;
                let mut step = SortStep::new(data.clone());
                step.comparing = vec![pos - 1, pos];
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }
            pos += 1;
        } else {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![pos - 1, pos];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            data.swap(pos - 1, pos);
            swp += 1;
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![pos - 1, pos];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            pos -= 1;
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
