pub fn day4() {
    let ( mut count_part1, mut count_part2) = (0, 0);

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
    let mut stack: Vec<u32> = Vec::new();
    let mut instance = 1;
    let ( mut passed_one, mut passed_two) = (false, false);
    
    for cur in password {
        let cur = cur.to_digit(10).unwrap();
        if let Some(last) = stack.pop() {
            if last == cur {
                passed_one = true;
                instance += 1;
                stack.push(cur);
            } else if last < cur  {
                if instance == 2 { passed_two = true }
                instance = 1;
                stack.push(cur);
            } else { // non increasing sequence, return
               return (false,false);
            }
        } else { stack.push(cur) }
    }
    if instance == 2 { passed_two = true} // when the repeating sequence of 2 repeats is at the end
    (passed_one, passed_two)
}