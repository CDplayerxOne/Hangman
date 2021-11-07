use std::io;

fn main() {
    let mut number_of_guesses = 0;
    let mut word = String::new();
    let mut word_arr: Vec<String> = Vec::new();
    // guesses is what you have so far
    let mut guesses: Vec<String> = Vec::new();
    let mut already_guessed: Vec<String> = Vec::new();
    let mut words_vec: Vec<String> = Vec::new();
    let mut last_indices = vec![0];

    println!("Type the word you want to guess below");
    io::stdin().read_line(&mut word).expect("Not a string");

    for (index, a) in word.chars().enumerate() {
        let s = a.to_string();
        if s == " " || s == "\r" {
            if last_indices[last_indices.len() - 1] == 0 {
                words_vec.push(String::from(
                    &word[last_indices[last_indices.len() - 1]..index],
                ));
                last_indices.push(index);
            } else {
                words_vec.push(String::from(
                    &word[last_indices[last_indices.len() - 1] + 1..index],
                ));
                last_indices.push(index);
            }
        }
        if s == "\n" || s == "\r" {
            continue;
        };
        guesses.push(String::from("_"));
        word_arr.push(s);
    }

    println!("{:?}", words_vec);
    println!("{:?}", last_indices);

    loop {
        // guess is the letter you guessed
        let mut guess = String::new();
        // guess vec is a vector containing each char in guess
        // vector = array basically
        let mut guess_vec: Vec<String> = Vec::new();

        println!("\nGuess a letter or word");
        if already_guessed.len() > 0 {
            print!("Already Guessed: ");
            print_without_enters(&already_guessed, true);
        }
        io::stdin().read_line(&mut guess).expect("Not a string");

        for letter in guess.chars() {
            if letter.to_string() == "\n" || letter.to_string() == "\r" {
                continue;
            }
            guess_vec.push(letter.to_string());
        }
        if guess_vec.len() > 1 {
            if check_win(&word_arr, &guess_vec) {
                number_of_guesses += 1;
                println!("You Win!!!");
                print_without_enters(&word_arr, false);
                println!("You took {} guesses", number_of_guesses);
                break;
            } else {
                if words_vec.contains(&guess) {
                    // for (i, word) in words_vec.iter().enumerate(){
                    //     // if word == &guess {
                    //     //     for (index, letter) in guess.char()
                    //     // }
                    // }
                }

                number_of_guesses += 1;
                println!("You have {} guesses left", 6 - number_of_guesses);
                print_without_enters(&guesses, true);
                if number_of_guesses >= 6 {
                    println!("\nUh Oh! You're out of tries");
                    print_without_enters(&word_arr, false);
                    break;
                }
                continue;
            }
        } else {
            if already_guessed.contains(&guess_vec[0]) {
                println!("Already guessed");
                continue;
            }
            for (index, letter) in word_arr.iter().enumerate() {
                if letter.to_string() == guess_vec[0] {
                    guesses[index] = letter.to_string();
                }
            }
        }

        if check_win(&guesses, &word_arr) {
            number_of_guesses += 1;
            println!("\nYou Win!!!");
            print_without_enters(&word_arr, false);
            println!("You took {} guesses", number_of_guesses);
            break;
        }
        number_of_guesses += 1;
        already_guessed.push(guess_vec[0].to_string());
        if number_of_guesses >= 6 {
            println!("Uh Oh! You're out of tries");
            print_without_enters(&word_arr, false);
            break;
        }
        print_without_enters(&guesses, true);
        println!("You have {} guesses left", 6 - number_of_guesses);
    }
}

fn check_win(array1: &Vec<String>, array2: &Vec<String>) -> bool {
    let mut previous = false;
    if array1.len() != array2.len() {
        return false;
    };
    for (index, letter) in array1.iter().enumerate() {
        if letter.to_string() == array2[index] {
            previous = true;
        } else {
            previous = false;
        }
        if previous == false {
            break;
        }
    }
    previous
}

fn print_without_enters(word: &Vec<String>, spaced: bool) {
    if !spaced {
        print!("The word was ")
    }
    for (index, letter) in word.iter().enumerate() {
        if letter.to_string() == "\n" || letter.to_string() == "\r" {
            continue;
        }
        if index == word.len() - 1 {
            print!("{}\n", letter)
        } else {
            if spaced {
                print!("{} ", letter)
            } else {
                print!("{}", letter)
            }
        }
    }
}
