use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

enum Op {
    Add,
    Subtract,
    Multiply,
}

impl Distribution<Op> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Op {
        match rng.gen_range(0..3) {
            0 => Op::Add,
            1 => Op::Subtract,
            _ => Op::Multiply,
        }
    }
}
impl Op {
    fn op_char(&self) -> char {
        match self {
            Op::Add => '+',
            Op::Subtract => '-',
            Op::Multiply => '*',
        }
    }

    fn apply(&self, num1: i32, num2: i32) -> i32 {
        match self {
            Op::Add => num1 + num2,
            Op::Subtract => num1 - num2,
            Op::Multiply => num1 * num2,
        }
    }
}

fn main() {
    println!("Math Game");

    let mut question_counter = 0;
    'game: loop {
        let num1 = rand::thread_rng().gen_range(1..10);
        let num2 = rand::thread_rng().gen_range(1..10);
        let op: Op = rand::random();

        question_counter += 1;
        println!(
            "Question #{}: {} {} {} = ?",
            question_counter,
            num1,
            op.op_char(),
            num2
        );
        let result = op.apply(num1, num2);

        'answer: loop {
            let answer = 'input: loop {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Unable to read from stdin");
                input = String::from(input.trim());
                if input == "Q" {
                    break 'game;
                }

                match input.parse::<i32>() {
                    Ok(result) => break 'input result,
                    Err(_) => {
                        println!("Please enter a valid number. (enter Q to exit)");
                        continue 'input;
                    }
                };
            };

            if answer == result {
                println!("Correct Answer");
                break 'answer;
            } else {
                println!("Wrong Answer (enter Q to exit)");
            }
        }
    }
}
