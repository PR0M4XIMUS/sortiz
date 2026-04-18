use super::SortStep;

/// Bitonic Sort: Batcher comparator network.
/// Array is padded to next power-of-2 with usize::MAX sentinels (act as +∞).
/// Steps truncate the padded tail so all frames stay at n_orig length.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let n_orig = initial.len();
    let mut steps: Vec<SortStep> = Vec::new();

    if n_orig == 0 {
        steps.push(SortStep::new(vec![]));
        return steps;
    }
    if n_orig == 1 {
        let mut s = SortStep::new(initial.to_vec());
        s.sorted = vec![0];
        steps.push(s);
        return steps;
    }

    let n_pow2 = n_orig.next_power_of_two();
    let sentinel = usize::MAX;
    let mut data: Vec<usize> = initial.to_vec();
    data.resize(n_pow2, sentinel);

    let mut cmp = 0u32;
    let mut swp = 0u32;

    // Iterative Batcher bitonic network over n_pow2 elements
    let mut k = 2usize;
    while k <= n_pow2 {
        let mut j = k / 2;
        while j >= 1 {
            for i in 0..n_pow2 {
                let l = i ^ j;
                if l > i {
                    let ascending = (i & k) == 0;
                    cmp += 1;
                    let should_swap =
                        if ascending { data[i] > data[l] } else { data[i] < data[l] };

                    // Only emit a visible step when at least one index is in the real region
                    if i < n_orig || l < n_orig {
                        let mut vis: Vec<usize> = data[..n_orig].to_vec();
                        // Replace any sentinel that landed in visible region with 0
                        for v in vis.iter_mut() {
                            if *v == sentinel { *v = 0; }
                        }
                        let mut step = SortStep::new(vis);
                        if i < n_orig { step.comparing.push(i); }
                        if l < n_orig { step.comparing.push(l); }
                        step.comparisons = cmp;
                        step.swaps = swp;
                        steps.push(step);
                    }

                    if should_swap {
                        data.swap(i, l);
                        swp += 1;
                        if i < n_orig || l < n_orig {
                            let mut vis: Vec<usize> = data[..n_orig].to_vec();
                            for v in vis.iter_mut() {
                                if *v == sentinel { *v = 0; }
                            }
                            let mut step = SortStep::new(vis);
                            if i < n_orig { step.swapping.push(i); }
                            if l < n_orig { step.swapping.push(l); }
                            step.comparisons = cmp;
                            step.swaps = swp;
                            steps.push(step);
                        }
                    }
                }
            }
            j /= 2;
        }
        k *= 2;
    }

    // Sentinels sort to the end — first n_orig values are the sorted real values
    let sorted: Vec<usize> = data[..n_orig].to_vec();
    let mut final_step = SortStep::new(sorted);
    final_step.sorted = (0..n_orig).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);

    steps
}
