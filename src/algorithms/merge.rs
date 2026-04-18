use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    merge_sort(&mut data, 0, n, &mut steps, &mut cmp, &mut swp);

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn merge_sort(
    data: &mut [usize],
    left: usize,
    right: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    if right - left <= 1 {
        return;
    }
    let mid = left + (right - left) / 2;
    merge_sort(data, left, mid, steps, cmp, swp);
    merge_sort(data, mid, right, steps, cmp, swp);
    merge(data, left, mid, right, steps, cmp, swp);
}

fn merge(
    data: &mut [usize],
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
        let mut step = SortStep::new(data.to_owned());
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
        let mut step = SortStep::new(data.to_owned());
        step.swapping = vec![k];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        k += 1;
    }

    while i < left_part.len() {
        data[k] = left_part[i];
        *swp += 1;
        let mut step = SortStep::new(data.to_owned());
        step.swapping = vec![k];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        i += 1;
        k += 1;
    }

    while j < right_part.len() {
        data[k] = right_part[j];
        *swp += 1;
        let mut step = SortStep::new(data.to_owned());
        step.swapping = vec![k];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        j += 1;
        k += 1;
    }
}
