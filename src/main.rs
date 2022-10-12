use colored::Colorize;
use std::io;

const ANSWER: &str = "HELLO";
const GREEN: (u8, u8, u8) = (100, 169, 101);
const YELLOW: (u8, u8, u8) = (200, 182, 83);
const GRAY: (u8, u8, u8) = (120, 124, 127);
const WORD_COUNT: usize = 5;
const MAX_PROMPT_COUNT: usize = 6;

#[derive(PartialEq)]
enum LetterStatus {
    Collect,
    PartiallyCollect,
    Wrong,
}

fn print_title() {
    print_collect(&String::from("TERMIDLE"));
    println!("{}", "\n");
}

fn prompt() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn print_collect(text: &String) {
    print!("{}", text.white().on_truecolor(GREEN.0, GREEN.1, GREEN.2));
}

fn print_partially_collect(letter: &String) {
    print!(
        "{}",
        letter.white().on_truecolor(YELLOW.0, YELLOW.1, YELLOW.2)
    );
}

fn print_wrong(letter: &String) {
    print!("{}", letter.white().on_truecolor(GRAY.0, GRAY.1, GRAY.2));
}

fn print_empty() {
    println!("{}", "_____");
}

fn print_line_break() {
    print!("{}", "\n");
}

fn main() {
    print_title();

    let mut guessed_histories: Vec<Vec<(char, LetterStatus)>> = Vec::new();

    loop {
        for history in &guessed_histories {
            for (letter, status) in history {
                match status {
                    LetterStatus::Collect => {
                        print_collect(&letter.to_string());
                    }
                    LetterStatus::PartiallyCollect => {
                        print_partially_collect(&letter.to_string());
                    }
                    LetterStatus::Wrong => {
                        print_wrong(&letter.to_string());
                    }
                }
            }
            print_line_break();
        }

        for _ in 0..MAX_PROMPT_COUNT - guessed_histories.len() {
            print_empty();
        }

        if guessed_histories.len() > 5 {
            println!("{}", "Game over...");
            break;
        }

        println!(
            "Guess a five-letter word! {}/{}",
            guessed_histories.len() + 1,
            MAX_PROMPT_COUNT
        );

        let guessed_word = prompt();

        if guessed_word.len() != WORD_COUNT + 1 {
            println!("{}", "The answer must be 5 letters! Try again...");
            continue;
        }
        let mut colored_letters: Vec<(char, LetterStatus)> = Vec::new();
        for (guessed_index, guessed_letter) in
            guessed_word.trim_end().to_uppercase().chars().enumerate()
        {
            if guessed_letter == ANSWER.chars().nth(guessed_index).unwrap() {
                // collect all
                colored_letters.push((guessed_letter, LetterStatus::Collect));
            } else if ANSWER.contains(guessed_letter) {
                // partially collect
                colored_letters.push((guessed_letter, LetterStatus::PartiallyCollect));
            } else {
                // wrong
                colored_letters.push((guessed_letter, LetterStatus::Wrong));
            }
        }

        if colored_letters
            .iter()
            .all(|(_, status)| matches!(status, LetterStatus::Collect))
        {
            println!(
                "{}",
                guessed_word
                    .to_uppercase()
                    .trim_end()
                    .white()
                    .on_truecolor(GREEN.0, GREEN.1, GREEN.2)
            );

            println!("{}", "Congratulations!!");
            break;
        }

        guessed_histories.push(colored_letters);
    }
}
