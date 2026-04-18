use super::SortStep;

/// Radix Sort (LSD, base-10): sorts digit by digit from least to most significant.
/// Each pass shows elements moving into their digit bucket.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut swp = 0u32;

    if n == 0 {
        steps.push(SortStep::new(data));
        return steps;
    }

    let max_val = *data.iter().max().unwrap_or(&1);
    let mut exp = 1usize;

    while max_val / exp > 0 {
        counting_pass(&mut data, exp, n, &mut steps, &mut swp);
        if exp > max_val { break; }
        exp *= 10;
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = 0;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}

fn counting_pass(
    data: &mut Vec<usize>,
    exp: usize,
    n: usize,
    steps: &mut Vec<SortStep>,
    swp: &mut u32,
) {
    let mut output = vec![0usize; n];
    let mut count = [0usize; 10];

    // Count occurrences of each digit
    for &val in data.iter() {
        let digit = (val / exp) % 10;
        count[digit] += 1;
    }

    // Cumulative count
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build output array (right to left for stability)
    for k in (0..n).rev() {
        let digit = (data[k] / exp) % 10;
        count[digit] -= 1;
        output[count[digit]] = data[k];
    }

    // Copy back, emitting a step each time an element moves
    for i in 0..n {
        if data[i] != output[i] {
            data[i] = output[i];
            *swp += 1;
        }
        let mut step = SortStep::new(data.clone());
        step.swapping = vec![i];
        step.comparisons = 0;
        step.swaps = *swp;
        steps.push(step);
    }
}
