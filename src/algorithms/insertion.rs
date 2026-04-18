use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    for i in 1..n {
        let key = data[i];
        let mut j = i;

        while j > 0 {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j - 1, j];
            step.sorted = (0..i).collect();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[j - 1] > key {
                data[j] = data[j - 1];
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![j - 1, j];
                step.sorted = (0..i).collect();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
                j -= 1;
            } else {
                break;
            }
        }
        data[j] = key;
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
