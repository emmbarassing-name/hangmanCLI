use rand::{self, Rng};
use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::io::Read;
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

    let mut i = 0;
    loop {
        if i >= 3 {
            break;
        } else {
            i += 1;
        }
        print!(".");
        // pause_execution(500);
    }

    println!("");
    //TODO starting line
    println!("And now the starting line");
}

fn inputDictionaryFilePath() -> String {
    println!("Input the full file path of the dictionary you'd like to use");
    // return read_line();

    return (String::from("/home/emm/twitterBot/hangman/src/en.txt"));
}

fn main() {
    let dictionaryPath = inputDictionaryFilePath();
    let wordPool = generate_word_pool(dictionaryPath);
    let targetWord = generate_target_word(wordPool);
    start_game();
    println!("{}", targetWord);
}
