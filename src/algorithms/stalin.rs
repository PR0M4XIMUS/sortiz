use super::SortStep;

/// Stalin Sort: exiles non-conforming elements to the tail, then re-integrates.
/// Array length never changes — exiles are swapped to a holding zone.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    let mut front = 0usize;
    let mut exile_start = n;
    let mut last_loyal: Option<usize> = None;
    let mut read = 0usize;

    while read < exile_start {
        cmp += 1;
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![read];
        step.comparisons = cmp;
        step.swaps = swp;
        steps.push(step);

        let loyal = match last_loyal {
            None => true,
            Some(prev) => data[read] >= prev,
        };

        if loyal {
            last_loyal = Some(data[read]);
            if read != front {
                data.swap(read, front);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![front, read];
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }
            front += 1;
            read += 1;
        } else {
            exile_start -= 1;
            data.swap(read, exile_start);
            swp += 1;
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![read, exile_start];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);
        }
    }

    for i in front..n {
        let key = data[i];
        let mut j = i;

        while j > 0 {
            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j - 1, j];
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step);

            if data[j - 1] > key {
                data[j] = data[j - 1];
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![j - 1, j];
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
