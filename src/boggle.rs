use rand::Rng;
use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use trie_rs::{TrieBuilder, Trie};
use rand::seq::SliceRandom;

const EMPTY: u8 = 0;
const VISITED: u8 = 1;

//Solve the boggle board and return all words found
pub fn solve(board: &Vec<Vec<char>>, dict: &Trie<u8>) -> Vec<String> {
    let mut all_words: Vec<String> = Vec::new();
    for row in 0..4 {
        for col in 0..4 {
            let mut visited: Vec<Vec<u8>> = vec![vec![EMPTY; 4]; 4];
            visited[row][col] = VISITED;
            let word = String::from(board[row][col]);
            // println!("Search for letter {}", word);
            let letter: Letter = Letter { row: row as i8, col: col as i8 };
            all_words.append(&mut find_words(&word, &letter, &mut visited, &board, &dict))
        }
    }
    all_words.sort();
    all_words.dedup();

    return all_words;
}

fn find_words(word: &String, letter: &Letter, visited: &mut Vec<Vec<u8>>, board: &Vec<Vec<char>>, dict: &Trie<u8>) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    for letter in unvisited_neighbours(&letter, visited) {
        visited[letter.row as usize][letter.col as usize] = VISITED;
        let mut new_word = word.clone();
        new_word.push(board[letter.row as usize][letter.col as usize]);
        // println!("{}", new_word);
        if is_word(dict, &new_word) {
            words.push( new_word.clone());
        }
        if is_word_prefix(dict, &new_word) {
            words.append(&mut find_words(&new_word, &letter, visited, board, dict));
        }

        visited[letter.row as usize][letter.col as usize] = EMPTY;
    }

    return words;
}

fn unvisited_neighbours(letter: &Letter, visited: &Vec<Vec<u8>>) -> Vec<Letter> {
    let mut neighbours: Vec<Letter> = Vec::new();
    let lower_bound: i8 = -1;
    let upper_bound: i8 = 4;

    for row in letter.row -1 ..= letter.row + 1 {
        for col in letter.col -1 ..= letter.col + 1 {
            if (row < upper_bound && row > lower_bound) && (col < upper_bound && col > lower_bound) {
                if visited[row as usize][col as usize] != VISITED {
                    neighbours.push(Letter {row, col})
                }
            }
        }
    }

    return neighbours;
}

// Does the search word match a word in the dictionary.
fn is_word(trie: &Trie<u8>, word: &String) -> bool {
    if word.len() < 4 {
        return false;
    }

    //TODO if the word has a Q in it, replace it with 'QU' before checking if its a word
    return trie.exact_match(word);
}

// Is the search word a prefix of other words.
fn is_word_prefix(trie: &Trie<u8>, word: &String) -> bool {
    return trie.predictive_search(word).len() > 0;
}

pub fn build_dict() -> Trie<u8> {
    if let Ok(lines) = read_lines("./bogglewords.txt") {
        let mut builder: TrieBuilder<u8> = TrieBuilder::new();
        for line in lines {
            if let Ok(word) = line {
                // println!("{}", word);
                builder.push(word);
            }
        }

        let trie = builder.build();

        return trie;
    }

    // if theres an error return an empty trie
    let builder = TrieBuilder::new();
    let trie = builder.build();
    return trie;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn print_board(board: &Vec<Vec<char>>) {
    for row in board.iter() {
        println!("{:?}", row);
    }
}

//Create a new boggle board with random letters.
pub fn initialize_board() -> Vec<Vec<char>>{
    let mut dice = get_dice();
    let mut board = vec![vec!['_'; 4]; 4];
    for row in 0..4 {
        for col in 0..4 {
            let mut rng = rand::thread_rng();
            let letter = dice.pop().unwrap().choose(&mut rng).unwrap().clone();
            // board[row][col] = random_letter();
            board[row][col] = letter;
        }
    }

    return board;
}

fn get_dice() -> Vec<Vec<char>> {
    let die0 = vec!['R', 'I', 'F', 'O', 'B', 'X'];
    let die1 = vec!['I', 'F', 'E', 'H', 'E', 'Y'];
    let die2 = vec!['D', 'E', 'N', 'O', 'W', 'S'];
    let die3 = vec!['U', 'T', 'O', 'K', 'N', 'D'];
    let die4 = vec!['H', 'M', 'S', 'R', 'A', 'O'];
    let die5 = vec!['L', 'U', 'P', 'E', 'T', 'S'];
    let die6 = vec!['A', 'C', 'I', 'T', 'O', 'A'];
    let die7 = vec!['Y', 'L', 'G', 'K', 'U', 'E'];
    let die8 = vec!['S', 'B', 'M', 'J', 'O', 'A'];
    // let die8 = vec!['Q', 'B', 'M', 'J', 'O', 'A'];
    let die9 = vec!['E', 'H', 'I', 'S', 'P', 'N'];
    let die10 = vec!['V', 'E', 'T', 'I', 'G', 'N'];
    let die11 = vec!['B', 'A', 'L', 'I', 'Y', 'T'];
    let die12 = vec!['E', 'Z', 'A', 'V', 'N', 'D'];
    let die13 = vec!['R', 'A', 'L', 'E', 'S', 'C'];
    let die14 = vec!['U', 'W', 'I', 'L', 'R', 'G'];
    let die15 = vec!['P', 'A', 'C', 'E', 'M', 'D'];

    let dice = vec![die0,die1,die2,die3,die4,die5,die6,die7,die8,die9,die10,die11,die12,die13,die14,die15];

    return dice;
}

//Generate a random char in the alphabet.
fn random_letter() -> char {
    let mut rng = rand::thread_rng();
    let letter: char = rng.gen_range('A'..'Z') as char;

    return letter;
}

#[derive(Debug)]
struct Letter {
    row: i8,
    col: i8,
}
