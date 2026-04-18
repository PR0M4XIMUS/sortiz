use super::SortStep;

/// Pancake Sort: sorts by flipping prefixes.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut sorted: Vec<usize> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    for size in (2..=n).rev() {
        let max_idx = data[..size]
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);

        if max_idx == size - 1 {
            sorted.push(size - 1);
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![size - 1];
            step.sorted = sorted.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);
            continue;
        }

        if max_idx > 0 {
            flip(&mut data, max_idx, &mut steps, &sorted, &mut cmp, &mut swp);
        }
        flip(&mut data, size - 1, &mut steps, &sorted, &mut cmp, &mut swp);

        sorted.push(size - 1);
        let mut step = SortStep::new(data.clone());
        step.sorted = sorted.clone();
        step.comparisons = cmp;
        step.swaps = swp;
        steps.push(step);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn flip(
    data: &mut [usize],
    end: usize,
    steps: &mut Vec<SortStep>,
    sorted: &[usize],
    cmp: &mut u32,
    swp: &mut u32,
) {
    let mut lo = 0;
    let mut hi = end;
    while lo < hi {
        data.swap(lo, hi);
        *swp += 1;
        let mut step = SortStep::new(data.to_owned());
        step.swapping = vec![lo, hi];
        step.sorted = sorted.to_vec();
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        lo += 1;
        hi -= 1;
    }
}
