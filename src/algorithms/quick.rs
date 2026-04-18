use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    if n > 1 {
        quick_sort(&mut data, 0, n - 1, &mut steps, &mut cmp, &mut swp);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn quick_sort(
    data: &mut [usize],
    low: usize,
    high: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    if low < high {
        let pivot = partition(data, low, high, steps, cmp, swp);
        if pivot > 0 {
            quick_sort(data, low, pivot - 1, steps, cmp, swp);
        }
        quick_sort(data, pivot + 1, high, steps, cmp, swp);
    }
}

fn partition(
    data: &mut [usize],
    low: usize,
    high: usize,
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) -> usize {
    let pivot_val = data[high];
    let mut i = low;

    for j in low..high {
        *cmp += 1;
        let mut step = SortStep::new(data.to_owned());
        step.comparing = vec![j, high];
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);

        if data[j] <= pivot_val {
            if i != j {
                data.swap(i, j);
                *swp += 1;
                let mut step = SortStep::new(data.to_owned());
                step.swapping = vec![i, j];
                step.comparisons = *cmp;
                step.swaps = *swp;
                steps.push(step);
            }
            i += 1;
        }
    }

    data.swap(i, high);
    *swp += 1;
    let mut step = SortStep::new(data.to_owned());
    step.swapping = vec![i, high];
    step.sorted = vec![i];
    step.comparisons = *cmp;
    step.swaps = *swp;
    steps.push(step);

    i
}
