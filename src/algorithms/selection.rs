use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    for i in 0..n {
        let mut min_idx = i;
        let sorted_so_far: Vec<usize> = (0..i).collect();

        for j in i + 1..n {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j, min_idx];
            step.sorted = sorted_so_far.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[j] < data[min_idx] {
                min_idx = j;
            }
        }

        if min_idx != i {
            data.swap(i, min_idx);
            swp += 1;
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![i, min_idx];
            step.sorted = sorted_so_far;
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
