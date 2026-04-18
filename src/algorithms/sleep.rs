use super::SortStep;

/// Sleep Sort: elements "wake up" in value order and swap into a growing sorted prefix.
/// Visualised by tracking each original element's position and moving it leftward.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let n = initial.len();
    let mut steps: Vec<SortStep> = Vec::new();

    if n == 0 {
        steps.push(SortStep::new(vec![]));
        return steps;
    }

    // Wake-up order: sort by value, break ties by original index.
    let mut wake_order: Vec<(usize, usize)> = initial
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();
    wake_order.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    let mut data = initial.to_vec();

    // current_pos[orig_idx] = where original element orig_idx currently sits
    // at_pos[pos]           = which orig_idx is currently at position pos
    let mut current_pos: Vec<usize> = (0..n).collect();
    let mut at_pos: Vec<usize> = (0..n).collect();

    let mut cmp = 0u32;
    let mut swp = 0u32;

    steps.push(SortStep::new(data.clone()));

    let mut placed_count = 0usize;

    for (out_idx, &(_val, orig_idx)) in wake_order.iter().enumerate() {
        let target = out_idx;
        let cur = current_pos[orig_idx];

        // Highlight the waking element
        cmp += 1;
        let mut wake_step = SortStep::new(data.clone());
        wake_step.comparing = vec![cur];
        wake_step.comparisons = cmp;
        wake_step.swaps = swp;
        wake_step.sorted = (0..placed_count).collect();
        steps.push(wake_step);

        if cur != target {
            // Swap waking element to its output position
            let displaced = at_pos[target];

            data.swap(cur, target);
            swp += 1;

            // Update tracking arrays
            current_pos[orig_idx] = target;
            current_pos[displaced] = cur;
            at_pos[target] = orig_idx;
            at_pos[cur] = displaced;

            let mut swap_step = SortStep::new(data.clone());
            swap_step.swapping = vec![cur, target];
            swap_step.comparisons = cmp;
            swap_step.swaps = swp;
            swap_step.sorted = (0..placed_count).collect();
            steps.push(swap_step);
        }

        placed_count += 1;
        let mut placed_step = SortStep::new(data.clone());
        placed_step.sorted = (0..placed_count).collect();
        placed_step.comparisons = cmp;
        placed_step.swaps = swp;
        steps.push(placed_step);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);

    steps
}
