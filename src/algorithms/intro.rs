use super::SortStep;

/// Intro Sort: quicksort that switches to heapsort when recursion depth exceeds 2*log2(n).
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    if n > 1 {
        let depth_limit = 2 * (n as f64).log2() as usize;
        introsort(&mut data, 0, n, depth_limit, &mut steps, &mut cmp, &mut swp);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn introsort(
    data: &mut Vec<usize>,
    lo: usize,
    hi: usize,
    depth: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    let len = hi - lo;
    if len <= 1 {
        return;
    }
    if len <= 16 {
        insertion_sort(data, lo, hi, steps, cmp, swp);
        return;
    }
    if depth == 0 {
        heapsort(data, lo, hi, steps, cmp, swp);
        return;
    }
    let pivot = partition(data, lo, hi, steps, cmp, swp);
    introsort(data, lo, pivot, depth - 1, steps, cmp, swp);
    introsort(data, pivot + 1, hi, depth - 1, steps, cmp, swp);
}

fn insertion_sort(
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

fn partition(
    data: &mut Vec<usize>,
    lo: usize,
    hi: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) -> usize {
    let pivot_val = data[hi - 1];
    let mut i = lo;
    for j in lo..hi - 1 {
        *cmp += 1;
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![j, hi - 1];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        if data[j] <= pivot_val {
            if i != j {
                data.swap(i, j);
                *swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, j];
                step.comparisons = *cmp;
                step.swaps = *swp;
                steps.push(step);
            }
            i += 1;
        }
    }
    data.swap(i, hi - 1);
    *swp += 1;
    let mut step = SortStep::new(data.clone());
    step.swapping = vec![i, hi - 1];
    step.comparisons = *cmp;
    step.swaps = *swp;
    steps.push(step);
    i
}

fn heapsort(
    data: &mut Vec<usize>,
    lo: usize,
    hi: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    let len = hi - lo;
    for i in (0..len / 2).rev() {
        sift_down(data, lo, i, len, steps, cmp, swp);
    }
    for end in (1..len).rev() {
        data.swap(lo, lo + end);
        *swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![lo, lo + end];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        sift_down(data, lo, 0, end, steps, cmp, swp);
    }
}

fn sift_down(
    data: &mut Vec<usize>,
    base: usize,
    root: usize,
    end: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    let mut root = root;
    loop {
        let left = 2 * root + 1;
        if left >= end { break; }
        let right = left + 1;
        let mut largest = root;
        *cmp += 1;
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![base + left, base + largest];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        if data[base + left] > data[base + largest] { largest = left; }
        if right < end {
            *cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![base + right, base + largest];
            step.comparisons = *cmp;
            step.swaps = *swp;
            steps.push(step);
            if data[base + right] > data[base + largest] { largest = right; }
        }
        if largest == root { break; }
        data.swap(base + root, base + largest);
        *swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![base + root, base + largest];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        root = largest;
    }
}
