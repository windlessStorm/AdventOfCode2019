pub fn day4() {
    let mut count_part1 = 0;
    let mut count_part2 = 0;
    for i in 109165..576723 {
        // println!("checking number: {}  ", i);
        let str_num: Vec<char> = i.to_string().chars().collect();
        let mut stack: Vec<char> = Vec::new();
        let mut increasing_sequence = true;
        let mut passed_two = false;
        let mut match_flag = false;
        let mut instance = 1;
        // print!("{:?} ", str_num);
        
        for c in str_num {
            // println!("cur: {}", c);
            if let Some(last) = stack.pop() {
                // println!("last: {}", last);
                if last == c {
                    match_flag = true;
                    instance += 1;
                    // println!("match!! Instance count: {}", instance);
                    stack.push(last);
                    stack.push(c);
                    // println!("Cur Stack: {:?}", stack);
                    continue;
                } else if last.to_digit(10).unwrap() < c.to_digit(10).unwrap()  {
                    if instance == 2 {
                        passed_two = true;
                    }
                    instance = 1;
                    // println!("No match!! Instance count: {}", instance);
                    stack.push(last);
                    stack.push(c);
                    // println!("Cur Stack: {:?}", stack);
                } else {
                    // println!("Sequence is not an increasing sequence!!");
                    increasing_sequence = false;
                    break;
                }
            } else {
                // println!("Stack is empty. Cannot pop!");
                stack.push(c);
                // println!("Cur Stack: {:?}", stack);
            }
        }
        if instance == 2 { 
            passed_two = true;   
        }
        if increasing_sequence && match_flag {
            count_part1 += 1;
            // println!("part1 good!! Counter: {}", count_part1);
        }
        if passed_two && increasing_sequence {
            count_part2 += 1;
            // println!("part2 good!! Counter: {}", count_part2);
        }
    }
    println!("D41: {}\nD42: {}", count_part1, count_part2);
}