use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    merge_sort(&mut data, 0, n, &mut steps);

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}

fn merge_sort(data: &mut Vec<usize>, left: usize, right: usize, steps: &mut Vec<SortStep>) {
    if right - left <= 1 {
        return;
    }
    let mid = left + (right - left) / 2;
    merge_sort(data, left, mid, steps);
    merge_sort(data, mid, right, steps);
    merge(data, left, mid, right, steps);
}

fn merge(
    data: &mut Vec<usize>,
    left: usize,
    mid: usize,
    right: usize,
    steps: &mut Vec<SortStep>,
) {
    let left_part = data[left..mid].to_vec();
    let right_part = data[mid..right].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < left_part.len() && j < right_part.len() {
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![left + i, mid + j];
        steps.push(step);

        if left_part[i] <= right_part[j] {
            data[k] = left_part[i];
            i += 1;
        } else {
            data[k] = right_part[j];
            j += 1;
        }

        let mut step = SortStep::new(data.clone());
        step.swapping = vec![k];
        steps.push(step);
        k += 1;
    }

    while i < left_part.len() {
        data[k] = left_part[i];
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![k];
        steps.push(step);
        i += 1;
        k += 1;
    }

    while j < right_part.len() {
        data[k] = right_part[j];
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![k];
        steps.push(step);
        j += 1;
        k += 1;
    }
}
