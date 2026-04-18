use super::SortStep;

const RUN: usize = 32;

/// Tim Sort: insertion sort on small runs, then merge them together.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    if n == 0 {
        steps.push(SortStep::new(data));
        return steps;
    }

    // Sort individual runs with insertion sort
    let mut start = 0;
    while start < n {
        let end = (start + RUN).min(n);
        insertion_run(&mut data, start, end, &mut steps, &mut cmp, &mut swp);
        start += RUN;
    }

    // Merge runs
    let mut size = RUN;
    while size < n {
        let mut left = 0;
        while left < n {
            let mid = (left + size).min(n);
            let right = (left + 2 * size).min(n);
            if mid < right {
                merge(&mut data, left, mid, right, &mut steps, &mut cmp, &mut swp);
            }
            left += 2 * size;
        }
        size *= 2;
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn insertion_run(
    data: &mut Vec<usize>,
    lo: usize,
    hi: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    for i in lo + 1..hi {
        let key = data[i];
        let mut j = i;
        while j > lo {
            *cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j - 1, j];
            step.comparisons = *cmp;
            step.swaps = *swp;
            steps.push(step);

            if data[j - 1] > key {
                data[j] = data[j - 1];
                *swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![j - 1, j];
                step.comparisons = *cmp;
                step.swaps = *swp;
                steps.push(step);
                j -= 1;
            } else {
                break;
            }
        }
        data[j] = key;
    }
}

fn merge(
    data: &mut Vec<usize>,
    left: usize,
    mid: usize,
    right: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    let left_part = data[left..mid].to_vec();
    let right_part = data[mid..right].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < left_part.len() && j < right_part.len() {
        *cmp += 1;
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![left + i, mid + j];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);

        if left_part[i] <= right_part[j] {
            data[k] = left_part[i];
            i += 1;
        } else {
            data[k] = right_part[j];
            j += 1;
        }
        *swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![k];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        k += 1;
    }

    while i < left_part.len() {
        data[k] = left_part[i];
        *swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![k];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        i += 1; k += 1;
    }

    while j < right_part.len() {
        data[k] = right_part[j];
        *swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![k];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        j += 1; k += 1;
    }
}
