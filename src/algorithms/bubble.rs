use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    for i in 0..n {
        let already_sorted: Vec<usize> = (n - i..n).collect();
        for j in 0..n - i - 1 {
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j, j + 1];
            step.sorted = already_sorted.clone();
            steps.push(step);

            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![j, j + 1];
                step.sorted = already_sorted.clone();
                steps.push(step);
            }
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}
