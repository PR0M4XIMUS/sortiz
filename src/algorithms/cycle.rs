use super::SortStep;

/// Cycle Sort: minimises writes by cycling each element directly to its final position.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    for cycle_start in 0..n.saturating_sub(1) {
        let mut item = data[cycle_start];

        // Find where this item belongs
        let mut pos = cycle_start;
        for i in cycle_start + 1..n {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, cycle_start];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);
            if data[i] < item {
                pos += 1;
            }
        }

        if pos == cycle_start {
            continue;
        }

        // Skip duplicates
        while item == data[pos] {
            pos += 1;
        }

        std::mem::swap(&mut data[pos], &mut item);
        swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![pos, cycle_start];
        step.comparisons = cmp;
        step.swaps = swp;
        steps.push(step);

        // Rotate the rest of the cycle
        while pos != cycle_start {
            pos = cycle_start;
            for i in cycle_start + 1..n {
                cmp += 1;
                let mut step = SortStep::new(data.clone());
                step.comparing = vec![i, cycle_start];
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
                if data[i] < item {
                    pos += 1;
                }
            }

            while item == data[pos] {
                pos += 1;
            }

            std::mem::swap(&mut data[pos], &mut item);
            swp += 1;
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![pos, cycle_start];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
