use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp = data[i];
            let mut j = i;

            while j >= gap {
                cmp += 1;
                let mut step = SortStep::new(data.clone());
                step.comparing = vec![j - gap, j];
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);

                if data[j - gap] > temp {
                    data[j] = data[j - gap];
                    swp += 1;
                    let mut step = SortStep::new(data.clone());
                    step.swapping = vec![j - gap, j];
                    step.comparisons = cmp;
                    step.swaps = swp;
                    steps.push(step);
                    j -= gap;
                } else {
                    break;
                }
            }
            data[j] = temp;
        }
        gap /= 2;
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
