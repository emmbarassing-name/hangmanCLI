use rand::{self, Rng};
use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io;
use std::io::Read;
use std::iter::FromIterator;
use std::{thread, time};

fn read_line() -> String {
    let mut userInput = String::new();
    io::stdin()
        .read_line(&mut userInput)
        .expect("Failed to read line");

    return userInput;
}

fn pause_execution(timeout: u64) {
    let timeoutDuration = time::Duration::from_millis(timeout);

    thread::sleep(timeoutDuration);
}

fn generate_word_pool(dictionaryPath: String) -> Vec<String> {
    let mut wordFileData = String::new();
    wordFileData = read_to_string(dictionaryPath).expect("Unable to read string");

    let dictionarySplit = wordFileData.split("\n");
    let mut dictionaryVector = Vec::new();
    for dictionaryWord in dictionarySplit {
        dictionaryVector.push(String::from(dictionaryWord))
    }
    return dictionaryVector;
}

//TODO figure out how to do this casting to an array or something idk maybe something like .collect-ing into an array?
fn generate_target_word(wordPool: Vec<String>) -> String {
    let mut wordPoolLen = 0;
    let mut targetWord = String::new();
    let worldPoolForForLength = wordPool.to_vec();

    for _word in worldPoolForForLength {
        wordPoolLen += 1;
    }

    let mut rng = rand::thread_rng();
    let randomIndex = rng.gen_range(1..=wordPoolLen);

    let mut index = 0;

    for potentialTarget in wordPool {
        if (index == randomIndex) {
            targetWord = potentialTarget;
            break;
        }

        index += 1
    }

    return targetWord;
}

fn start_game() {
    println!("Looks like this is the end of the line for you pardner.");
    println!("...");
    println!("unless you can guess the word");
}

fn inputDictionaryFilePath() -> String {
    println!("Input the full file path of the dictionary you'd like to use");
    //TODO fix this function to make dynamic dictionaries possible
    // return read_line();

    return String::from("/home/emm/twitterBot/hangman/src/en.txt");
}

fn computeGuess(
    currentGuesses: &mut HashSet<char>,
    lettersToGuess: &mut HashSet<char>,
    currentGuess: &String,
) {
    let letterGuesseed = currentGuess.chars().nth(0).unwrap();

    let mut isValid = lettersToGuess.contains(&letterGuesseed);

    if isValid {
        currentGuesses.insert(letterGuesseed);
    }
}

fn generateHashSetForGuesses(targetWord: String) -> HashSet<char> {
    let mut usedLetters: HashSet<char> = HashSet::new();

    for letter in targetWord.chars() {
        usedLetters.insert(letter);
    }

    return usedLetters;
}

fn printRemainingWord(userGuesses: &HashSet<char>, targetWord: String) {
    let mut stringToPrint = String::new();

    for letter in targetWord.chars() {
        if (userGuesses.contains(&letter)) {
            stringToPrint += &String::from(letter)
        } else {
            stringToPrint += &String::from(" _ ")
        }
    }
    println!("{}", stringToPrint)
}

fn guessLoop(targetWord: String) {
    let mut lettersToGuess = generateHashSetForGuesses(targetWord.clone());
    let mut userGuesses: HashSet<char> = HashSet::new();

    loop {
        println!("guess a letter pardner!");
        let currentGuess = read_line();
        //userGuesses is mutated in here rather than regenerated and returned
        computeGuess(&mut userGuesses, &mut lettersToGuess, &currentGuess);

        printRemainingWord(&userGuesses, targetWord.clone());

        let isWin: bool = userGuesses.len() == lettersToGuess.len();
        if (isWin) {
            println!("aw shucks, you win");
            break;
        }
    }
}

fn main() {
    let dictionaryPath = inputDictionaryFilePath();
    let wordPool = generate_word_pool(dictionaryPath);
    let targetWord = generate_target_word(wordPool);
    start_game();
    println!("{}", targetWord);

    guessLoop(targetWord)
}
