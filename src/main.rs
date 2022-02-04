use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LetterStatus {
    Black,
    Yellow,
    Green,
}

type WordStatus = [LetterStatus; 5];

fn string_to_word_status(s: &String) -> WordStatus {
    let mut ret = [LetterStatus::Black; 5];
    for i in 0..5 {
        let status = match s.chars().nth(i).unwrap() {
            'G' => LetterStatus::Green,
            'Y' => LetterStatus::Yellow,
            'B' => LetterStatus::Black,
            _ => panic!(),
        };
        ret[i] = status;
    }
    return ret;
}

fn match_word(word: &str, candidate: &str) -> WordStatus {
    let mut char_map = [0; 26];
    let mut status = [LetterStatus::Black; 5];
    for i in 0..5 {
        let ca = word.chars().nth(i).unwrap();
        let cb = candidate.chars().nth(i).unwrap();
        if ca == cb {
            status[i] = LetterStatus::Green;
        } else {
            char_map[(cb as u8 - 'a' as u8) as usize] += 1;
        }
    }
    for i in 0..5 {
        let ca = word.chars().nth(i).unwrap();
        let cb = candidate.chars().nth(i).unwrap();
        if ca != cb {
            let code = (ca as u8 - 'a' as u8) as usize;
            if char_map[code] > 0 {
                status[i] = LetterStatus::Yellow;
                char_map[code] -= 1;
            } else {
                status[i] = LetterStatus::Black;
            }
        }
    }
    return status;
}

fn get_exp_of_word(word: &str, possible_words: &Vec<String>) -> f64 {
    let mut catagories = HashMap::<WordStatus, u64>::new();
    for candidate in possible_words {
        let status = match_word(word, candidate);
        if let Some(cnt) = catagories.get_mut(&status) {
            *cnt += 1;
        } else {
            catagories.insert(status, 1);
        }
    }
    let mut ret = 0;
    let correct = [LetterStatus::Green; 5];
    for (s, v) in catagories {
        if s != correct {
            ret += v * v;
        }
    }
    return ret as f64 / possible_words.len() as f64;
}

fn get_best_word<'a>(
    allowed_words: &'a Vec<String>,
    possible_words: &Vec<String>,
) -> (&'a String, f64) {
    let mut best = None;
    for word in allowed_words {
        let exp = get_exp_of_word(word, possible_words);
        if let Some((_, best_exp)) = best {
            if exp < best_exp {
                best = Some((word, exp));
            }
        } else {
            best = Some((word, exp));
        }
    }
    return best.unwrap();
}

fn eliminate(possible_words: &mut Vec<String>, word: &str, status: WordStatus) {
    possible_words.retain(|candidate| match_word(word, candidate) == status);
}

fn main() {
    let mut allowed_words = Vec::new();
    let mut possible_words = Vec::new();
    if let Ok(lines) = read_lines("./allowed.txt") {
        for line in lines {
            if let Ok(word) = line {
                allowed_words.push(word);
            }
        }
    }
    if let Ok(lines) = read_lines("./answers.txt") {
        for line in lines {
            if let Ok(word) = line {
                possible_words.push(word);
            }
        }
    }
    allowed_words.append(&mut possible_words.clone());
    // let good_choices = vec!["salet", "crate", "trace", "about", "share"];
    // for choice in good_choices {
    //     println!(
    //         "{}: {:.4}",
    //         choice,
    //         get_exp_of_word(choice, &possible_words)
    //     );
    // }

    let mut round = 0;
    loop {
        if round != 0 {
            let (best_word, exp) = get_best_word(&allowed_words, &possible_words);
            println!("Best word: {}, exp: {:.4}", best_word, exp);
        } else {
            // First word is fixed.
            println!("Best word: roate, exp: 60.4246");
        }
        let mut word = String::new();
        let mut status = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut word).unwrap();
        let word = word.trim();
        stdin.read_line(&mut status).unwrap();
        let status: WordStatus = string_to_word_status(&status);
        eliminate(&mut possible_words, word, status);
        println!("Possible words left: {}", possible_words.len());
        for word in &possible_words {
            println!("{}", word);
        }
        round += 1;
    }
}
