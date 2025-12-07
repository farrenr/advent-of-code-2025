pub fn question2(input: Vec<String>) {
    let mut ranges:Vec<Vec<&str>> = Vec::new();

    for range_string in input[0].split(',') {
        let min_max: Vec<&str> = range_string.split('-').collect();
        ranges.push(min_max);
    }

    let mut invalid_total:u64 = 0;
    for range in ranges {
        let min = range[0].parse::<u64>().unwrap();
        let max = range[1].parse::<u64>().unwrap();

        for i in min..=max {
            let digits = i.checked_ilog10().unwrap_or(0) + 1;
            if digits % 2 != 0 {
                continue;
            }
            let middle: usize = (digits / 2) as usize;
            let i_string = i.to_string();
            let first = i_string.chars().take(middle).collect::<String>();
            let last = i_string.chars().skip(middle).take(middle).collect::<String>();
            if first == last {
                invalid_total += i;
            }
        }
    }

    println!("The answer is {}", invalid_total);
}
