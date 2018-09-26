pub fn euclidian_rythm(pattern: &mut [bool], pulses: usize, steps: usize) {
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
        pattern,
        &remainders,
        level as isize,
        0
    );

    // Put a 1 on the first step
    let index_first_one = pattern.iter().position(|&x| x == true).unwrap();
    pattern.rotate_left(index_first_one);
}

#[cfg(test)]
mod tests {
    use euclidian_rythm;
    #[test]
    fn it_works() {
        let mut pattern = [false; 8];
        let pulses = 4;
        let steps = pattern.len();
        euclidian_rythm(&mut pattern, pulses, steps);
        println!("{:?}", pattern);

        let mut pattern = [false; 9];
        let pulses = 3;
        let steps = pattern.len();
        euclidian_rythm(&mut pattern, pulses, steps);
        println!("{:?}", pattern);

        let mut pattern = [false; 12];
        let pulses = 7;
        let steps = pattern.len();
        euclidian_rythm(&mut pattern, pulses, steps);
        println!("{:?}", pattern);

        let mut pattern = [false; 13];
        let pulses = 5;
        let steps = pattern.len();
        euclidian_rythm(&mut pattern, pulses, steps);
        println!("{:?}", pattern);

        let mut pattern = [false; 31];
        let pulses = 7;
        let steps = pattern.len();
        euclidian_rythm(&mut pattern, pulses, steps);
        println!("{:?}", pattern);
    }
}
