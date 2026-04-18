use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    if n > 1 {
        for i in (0..n / 2).rev() {
            heapify(&mut data, n, i, &[], &mut steps, &mut cmp, &mut swp);
        }

        for end in (1..n).rev() {
            data.swap(0, end);
            swp += 1;
            let sorted: Vec<usize> = (end..n).collect();
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![0, end];
            step.sorted = sorted.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            heapify(&mut data, end, 0, &sorted, &mut steps, &mut cmp, &mut swp);
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn heapify(
    data: &mut Vec<usize>,
    heap_size: usize,
    i: usize,
    already_sorted: &[usize],
    steps: &mut Vec<SortStep>,
    cmp: &mut u32,
    swp: &mut u32,
) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < heap_size {
        *cmp += 1;
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![left, largest];
        step.sorted = already_sorted.to_vec();
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        if data[left] > data[largest] {
            largest = left;
        }
    }

    if right < heap_size {
        *cmp += 1;
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![right, largest];
        step.sorted = already_sorted.to_vec();
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        if data[right] > data[largest] {
            largest = right;
        }
    }

    if largest != i {
        data.swap(i, largest);
        *swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![i, largest];
        step.sorted = already_sorted.to_vec();
        step.comparisons = *cmp;
        step.swaps = *swp;
        steps.push(step);
        heapify(data, heap_size, largest, already_sorted, steps, cmp, swp);
    }
}
