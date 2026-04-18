use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    for i in 0..n {
        let already_sorted: Vec<usize> = (n - i..n).collect();
        for j in 0..n - i - 1 {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j, j + 1];
            step.sorted = already_sorted.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![j, j + 1];
                step.sorted = already_sorted.clone();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
