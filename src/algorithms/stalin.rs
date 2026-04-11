use super::SortStep;

/// Stalin Sort: scans left to right and "eliminates" any element that
/// isn't in non-decreasing order — exiling it to the end of the array.
/// After the purge, the exiled elements are quietly re-integrated using
/// insertion sort, because the economy collapsed without them.
///
/// Array length never changes (exiled elements are swapped to a holding
/// zone at the tail), so all animation invariants are preserved.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();

    // ── Phase 1: purge pass ───────────────────────────────────────────────
    // Walk the "loyal" front of the array. Dissenters get swapped to the
    // tail one by one. `front` is the write cursor for loyal elements;
    // `exile_start` tracks where exiles begin.
    let mut front = 0usize;   // next loyal slot
    let mut exile_start = n;  // first exile slot (grows left from n)
    let mut last_loyal: Option<usize> = None;

    let mut read = 0usize;
    while read < exile_start {
        // Interrogate this element
        let mut step = SortStep::new(data.clone());
        step.comparing = vec![read];
        steps.push(step);

        let loyal = match last_loyal {
            None => true,
            Some(prev) => data[read] >= prev,
        };

        if loyal {
            last_loyal = Some(data[read]);
            if read != front {
                // Slide loyal element into its slot
                data.swap(read, front);
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![front, read];
                steps.push(step);
            }
            front += 1;
            read += 1;
        } else {
            // Exile: swap dissenter to the tail of the unsorted region
            exile_start -= 1;
            data.swap(read, exile_start);
            let mut step = SortStep::new(data.clone());
            step.swapping = vec![read, exile_start];
            steps.push(step);
            // Don't advance `read` — the swapped-in element needs checking
        }
    }

    // ── Phase 2: re-integration (insertion sort on the whole array) ───────
    // The loyal prefix data[0..front] is already sorted. Insert exiles back.
    for i in front..n {
        let key = data[i];
        let mut j = i;

        while j > 0 {
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![j - 1, j];
            steps.push(step);

            if data[j - 1] > key {
                data[j] = data[j - 1];
                let mut step = SortStep::new(data.clone());
                step.swapping = vec![j - 1, j];
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
    steps.push(final_step);
    steps
}
