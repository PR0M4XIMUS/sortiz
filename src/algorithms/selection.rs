use super::SortStep;

pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    for i in 0..n {
        let mut min_idx = i;
        let sorted_so_far: Vec<usize> = (0..i).collect();

        for j in i + 1..n {
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j, min_idx];
            step.sorted = sorted_so_far.clone();
            steps.push(step);

            if data[j] < data[min_idx] {
                min_idx = j;
            }
        }

        if min_idx != i {
            data.swap(i, min_idx);
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![i, min_idx];
            step.sorted = sorted_so_far;
            steps.push(step);
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}
