use super::SortStep;
use rand::Rng;

/// Roulette Sort: spins remaining elements like a slot machine.
/// Some spins are rigged to fail for drama; the final spin always wins.
pub fn steps(initial: &[usize]) -> Vec<SortStep> {
    let mut data = initial.to_vec();
    let n = data.len();
    let mut steps: Vec<SortStep> = Vec::new();
    let mut cmp = 0u32;
    let mut swp = 0u32;

    if n == 0 {
        steps.push(SortStep::new(data));
        return steps;
    }

    let mut sorted_target = data.clone();
    sorted_target.sort_unstable();

    let mut rng = rand::thread_rng();
    let mut confirmed: Vec<usize> = Vec::new();

    let rotate_once = |data: &mut Vec<usize>, i: usize| {
        data[i..].rotate_right(1);
    };

    for i in 0..n {
        let remaining = n - i;
        let fake_spins: usize = if remaining > 1 { rng.gen_range(1..=4) } else { 0 };

        for _ in 0..fake_spins {
            let rotations = rng.gen_range(remaining..=remaining * 2 + 4);
            for _ in 0..rotations {
                if remaining > 1 { rotate_once(&mut data, i); }
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = (i..n).collect();
                step.sorted = confirmed.clone();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }

            if data[i] == sorted_target[i] && remaining > 1 {
                rotate_once(&mut data, i);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = (i..n).collect();
                step.sorted = confirmed.clone();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }

            cmp += 1;
            let mut step = SortStep::new(data.clone());
            step.comparing = vec![i];
            step.sorted = confirmed.clone();
            step.comparisons = cmp;
            step.swaps = swp;
            steps.push(step.clone());
            steps.push(step);
        }

        if remaining > 1 {
            debug_assert!(
                data[i..].contains(&sorted_target[i]),
                "roulette: sorted_target[{i}]={} not found in remaining slice",
                sorted_target[i]
            );
            for _ in 0..remaining * 2 {
                let next_front = data[n - 1];
                if next_front == sorted_target[i] { break; }
                rotate_once(&mut data, i);
                swp += 1;
                let mut step = SortStep::new(data.clone());
                step.swapping = (i..n).collect();
                step.sorted = confirmed.clone();
                step.comparisons = cmp;
                step.swaps = swp;
                steps.push(step);
            }
            rotate_once(&mut data, i);
        }

        swp += 1;
        let mut step = SortStep::new(data.clone());
        step.swapping = (i..n).collect();
        step.sorted = confirmed.clone();
        step.comparisons = cmp;
        step.swaps = swp;
        steps.push(step);

        confirmed.push(i);
        cmp += 1;
        let mut win_step = SortStep::new(data.clone());
        win_step.comparing = vec![i];
        win_step.sorted = confirmed.clone();
        win_step.comparisons = cmp;
        win_step.swaps = swp;
        steps.push(win_step.clone());
        steps.push(win_step.clone());
        steps.push(win_step);
    }

    let mut final_step = SortStep::new(data);
    final_step.sorted = (0..n).collect();
    final_step.comparisons = cmp;
    final_step.swaps = swp;
    steps.push(final_step);
    steps
}
