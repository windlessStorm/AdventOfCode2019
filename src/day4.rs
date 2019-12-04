pub fn day4() {
    let mut count_part1 = 0;
    let mut count_part2 = 0;

    for n in 109165..576723 {
        let password: Vec<char> = n.to_string()
                                        .chars()
                                            .collect();

        let (part1, part2) = eval_password(password.clone());
        if part1 { count_part1+=1 }
        if part2 { count_part2+=1 }
    }
    println!("D41: count = {}", count_part1);
    println!("D42: count = {}", count_part2);
}

fn eval_password(password: Vec<char>) -> (bool, bool) {
    let mut stack: Vec<char> = Vec::new();
    let mut increasing_sequence = true;
    let mut only_two = false;
    let mut match_flag = false;
    let mut instance = 1;
    let mut passed_two = false;
    let mut passed_one = false;
    
    for c in password {
        if let Some(last) = stack.pop() {
            if last == c {
                match_flag = true;
                instance += 1;
                stack.push(last);
                stack.push(c);
                continue;
            } else if last.to_digit(10).unwrap() < c.to_digit(10).unwrap()  {
                if instance == 2 { only_two = true }
                instance = 1;
                stack.push(last);
                stack.push(c);
            } else {
                increasing_sequence = false;
                break;
            }
        } else { stack.push(c) }
    }
    if instance == 2 { only_two = true }
    if increasing_sequence && match_flag { passed_one = true }
    if only_two && increasing_sequence { passed_two = true }
    
    (passed_one, passed_two)
}