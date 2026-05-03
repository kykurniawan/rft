#[cfg(test)]
mod tests {
    #[test]
    fn if_else() {
        let a = 1;
        let b = 2;
        if a == b {
            println!("a is equal to b");
        } else {
            println!("a is not equal to b");
        }
    }

    #[test]
    fn if_let() {
        let condition = true;
        let value = if condition { 1 } else { 2 };
        println!("The value is {}", value);
    }

    #[test]
    fn loop_iteration() {
        let mut count = 0;
        loop {
            count += 1;
            if count == 10 {
                break;
            }
        }

        println!("Loop executed {} times", count);
    }

    #[test]
    fn loop_return() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("Loop executed {} times, final result: {}", counter, result);
    }

    #[test]
    fn while_loop() {
        let mut counter = 0;
        while counter < 10 {
            counter += 1;
            println!("Counter: {}", counter);
        }
    }

    #[test]
    fn loop_label() {
        let counter = 10;

        'outer: loop {
            println!("Outer loop");
            for i in 0..counter {
                println!("i: {}", i);
                if i == 5 {
                    break 'outer;
                }
            }
        }
    }

    #[test]
    fn loop_through_collection() {
        let numbers: [i32; 3] = [1, 2, 3];

        for number in numbers {
            println!("Number: {}", number);
        }
    }
}
