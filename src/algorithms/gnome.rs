use super::SortStep;

/// Gnome Sort: a garden gnome moves forward through the array until it
/// finds an out-of-order pair, swaps them, then steps back one position
/// to recheck — wandering erratically forward and backward until the
/// whole array is in order. Simple, goofy, and oddly charming to watch.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    let mut pos = 0usize;

    while pos < n {
        if pos == 0 || data[pos] >= data[pos - 1] {
            // All good here — gnome steps forward
            if pos > 0 {
                let mut step = SortStep::new(data.clone());
                step.comparing = vec![pos - 1, pos];
                steps.push(step);
            }
            pos += 1;
        } else {
            // Out of order — gnome swaps and steps back
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![pos - 1, pos];
            steps.push(step);

            data.swap(pos - 1, pos);
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![pos - 1, pos];
            steps.push(step);

            pos -= 1;
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}
