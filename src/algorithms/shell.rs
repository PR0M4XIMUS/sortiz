use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let temp = data[i];
            let mut j = i;

            while j >= gap {
                let mut step = SortStep::new(data.clone());
                step.comparing = vec![j - gap, j];
                steps.push(step);

                if data[j - gap] > temp {
                    data[j] = data[j - gap];
                    let mut step = SortStep::new(data.clone());
                    step.swapping = vec![j - gap, j];
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
    steps.push(final_step);
    steps
}
