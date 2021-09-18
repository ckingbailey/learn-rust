pub fn find_mmm(num_list: Vec<f64>) -> [f64; 3] {
    // Find mean, median, and mode of the Vector list of floats.
    // Iterate over vector:
    // • For mean, keep a sum and divide it by len at the end
    // • For median, sort the input vec, then get the number at vec.len() / 2
    //   - if vec has even number of elements,
    //     average the elements at vec.len() / 2 - 1 and vec.len() / 2 + 1
    // • For mode, keep a HashMapm, num_counts, mapping nums to counts of nums
    //   and a Tuple, most_num, storing the currently known most numerous number
    //   where most_num.0 is a float from the input vec and most_num.1 the count of that float.
    //   On each iteration, update num_counts for the current vec element, n.
    //   Then check where most_num.1 is less than num_counts.get(n).
    //   If so, update most_num with n and the count of n
    let mut mmm = [0., 0., 0.];
    let mut sum = 0.;

    mmm[1] = get_median(&num_list);

    for n in &num_list {
        sum += n;
    }
    
    mmm
}

fn get_median(num_list: &Vec<f64>) -> f64 {
    // Get a sorted copy of num_list
    let len = num_list.len();
    let mut sorted = vec![0.; len];
    sorted.copy_from_slice(&num_list[..]);
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    
    // If num_list has an even number of elements,
    // len / 2 will equal the index of the "high side" of the middle 2 elements.
    // In that case, we need to average the middle 2 elements.
    // Get the elements at sorted[midpoint] and sorted[midpoint + 1]
    // and average them.
    let midpoint = len / 2;
    
    // For an odd-length vector, len / 2 will be the floor,
    // in which case midpoint * 2 will be less than len
    if (midpoint * 2) == len {
        let hi = sorted[midpoint];
        let lo = sorted[midpoint - 1];
        
        (hi + lo) / 2.
    } else {
        sorted[midpoint]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_median_odd_len_vec() {
        let test_list = vec![1., 5., 2., 3., 4.];
        let expect = 3.;

        let med = get_median(&test_list);

        assert!(med == 3., "Calculated median did not match expected value. Got: {}, expected: {}", med, expect);
    }

    #[test]
    fn test_get_median_even_len_vec() {
        let test_list = vec![3., 1., 2., 4., 6., 5.];
        let expect = 3.5;

        let med = get_median(&test_list);

        assert!(med == 3.5, "Calculated median did not match expected value. Got: {}, expected: {}", med, expect);
    }

    #[test]
    fn it_gives_expected_mode() {
        let test_list: Vec<f64> = vec![1., 1., 1., 5., 2.];
        
        // Expect mean 2.
        // Expect median 1.
        // Expect mode 1.
        let mmm = find_mmm(test_list);
        let expect_mmm = [2., 1., 1.];

        assert!(mmm == expect_mmm);
    }

    #[test]
    fn test_how_tests_work() {
        assert!([1., 2., 3.] == [1., 2., 3.]);
    }
}