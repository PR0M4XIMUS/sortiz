use super::SortStep;

/// Comb Sort: like bubble sort but starts comparing elements far apart.
/// The gap starts at n/1.3 and shrinks by a factor of 1.3 each pass,
/// creating a visual rhythm of wide sweeps that narrow down to a final
/// bubble-sort polish.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    let shrink = 1.3f64;
    let mut gap = n;
    let mut sorted = false;

    while !sorted {
        gap = ((gap as f64) / shrink) as usize;
        if gap <= 1 {
            gap = 1;
            sorted = true; // will be set false again if a swap occurs
        }

        for i in 0..n.saturating_sub(gap) {
            let j = i + gap;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i, j];
            steps.push(step);

            if data[i] > data[j] {
                data.swap(i, j);
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![i, j];
                steps.push(step);
                sorted = false;
            }
        }
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    steps.push(final_step);
    steps
}
