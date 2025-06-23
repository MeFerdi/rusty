use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let answer = "The letter e";
    let mut trial_count = 0;

    loop {
        println!("{}", riddle);
        trial_count += 1;

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        if user_input.trim().eq_ignore_ascii_case(answer.trim()) {
            println!("Number of trials: {}", trial_count);
            break;
        }
    }
}