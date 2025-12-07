pub fn question1(input: Vec<String>) {
    let mut zero_count: i32 = 0;
    let mut total: i32 = 50;

    for line in input {
        let direction = line.chars().nth(0).unwrap();
        let mut click = 1;
        if direction == 'L' {
            click = -1;
        }

        let amount = line[1..].parse::<i32>().unwrap();
        for _ in 0..amount {
            total += click;
            if total == -1 {
                total = 99;
            } else if total == 100 {
                total = 0;
            }
        }

        if total == 0 {
            zero_count += 1;
        }
    }

    println!("The answer is {}", zero_count);
}
