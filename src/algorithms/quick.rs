use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    if n > 1 {
        quick_sort(&mut data, 0, n - 1, &mut steps);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}

fn quick_sort(data: &mut [usize], low: usize, high: usize, steps: &mut Vec<SortStep>) {
    if low < high {
        let pivot = partition(data, low, high, steps);
        if pivot > 0 {
            quick_sort(data, low, pivot - 1, steps);
        }
        quick_sort(data, pivot + 1, high, steps);
    }
}

fn partition(
    data: &mut [usize],
    low: usize,
    high: usize,
    steps: &mut Vec<SortStep>,
) -> usize {
    let pivot_val = data[high];
    let mut i = low;

    for j in low..high {
        let mut step = SortStep::new(data.to_owned());
        step.comparing = vec![j, high];
        steps.push(step);

        if data[j] <= pivot_val {
            if i != j {
                data.swap(i, j);
                let mut step = SortStep::new(data.to_owned());
                step.swapping = vec![i, j];
                steps.push(step);
            }
            i += 1;
        }
    }

    data.swap(i, high);
    let mut step = SortStep::new(data.to_owned());
    step.swapping = vec![i, high];
    step.sorted = vec![i];
    steps.push(step);

    i
}
