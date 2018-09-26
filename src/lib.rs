pub fn euclidian_rythm(out: &mut [bool], pulses: usize, steps: usize) {
    if out.len() < steps {
        panic!("output array must be at least as long as steps");
    }
    if steps > 64 {
        panic!("64 is the maximum number of steps");
    }
    let mut pattern : [bool; 64] = [false;64];
    let mut counts : [usize; 64] = [0;64];
    let mut remainders : [usize; 64] = [0;64];

    let mut pulses = pulses;

    //clamp
    if pulses > steps {
        pulses = steps;
    }

    let mut divisor = steps - pulses;
    let mut level = 0;

    remainders[0] = pulses;

    // Run the euclid algorithm, store all the intermediate results
    loop {
        let r = remainders[level];
        counts[level] = divisor / r;
        remainders[level + 1] = divisor % r;

        divisor = remainders[level];
        level += 1;

        if remainders[level] <= 1 {
            break;
        }
    }
    counts[level] = divisor;

    // Build the pattern
    fn build(
        counts: &[usize],
        pattern: &mut [bool],
        remainders: &[usize],
        level: isize,
        index: usize
    ) -> usize {
        let mut index = index;
        if level == -1 {
            pattern[index] = false;
            index + 1
        } else if level == -2 {
            pattern[index] = true;
            index + 1
        } else {
            for _ in 0..counts[level as usize] {
                index = build(counts, pattern, remainders, level - 1, index);
            }
            if remainders[level as usize] != 0 {
                index = build(counts, pattern, remainders, level - 2, index);
            }
            index
        }
    }

    let _ = build(
        &counts,
        &mut pattern,
        &remainders,
        level as isize,
        0
    );

    // Put a 1 on the first step
    let index_first_one = pattern.iter().position(|&x| x == true).unwrap();
    for i in 0..steps {
        out[i] = pattern[(i + index_first_one) % steps];
    }
}

#[cfg(test)]
mod tests {
    use euclidian_rythm;

	fn test_case(pulses: usize, steps: usize, value: &Vec<usize>) {
        let mut pattern = [false; 64];
        euclidian_rythm(&mut pattern, pulses, steps);

		let res = pattern.iter().take(steps).map(|x| if *x { 1 } else { 0 }).collect::<Vec<usize>>();
		assert_eq!(value, &res, "pulses: {} steps {}", pulses, steps);
	}

    //patterns from http://cgm.cs.mcgill.ca/~godfried/publications/banff.pdf
    #[test]
    fn patterns() {
        let mut pattern = [false; 64];
        test_case(2, 5, &vec![1, 0, 1, 0, 0]);
        //test_case(3, 4, &vec![1, 0, 1, 1]);
        test_case(3, 5, &vec![1, 0, 1, 0, 1]);
        test_case(3, 7, &vec![1, 0, 1, 0, 1, 0, 0]);
        test_case(3, 8, &vec![1, 0, 0, 1, 0, 0, 1, 0]);
        test_case(4, 7, &vec![1, 0, 1, 0, 1, 0, 1]);
        test_case(4, 9, &vec![1, 0, 1, 0, 1, 0, 1, 0, 0]);
        test_case(4, 11, &vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]);
        //test_case(5, 6, &vec![1, 0, 1, 1, 1, 1]);
        //test_case(5, 7, &vec![1, 0, 1, 1, 0, 1, 1]);
        test_case(5, 8, &vec![1, 0, 1, 1, 0, 1, 1, 0]);
        test_case(5, 9, &vec![1, 0, 1, 0, 1, 0, 1, 0, 1]);
    }
}

