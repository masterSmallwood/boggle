mod boggle;

use colored::*;
use std::char;
use std::str;
use trie_rs::Trie;
use text_io::read;
use std::string::String;

fn main() {
    println!("{}", "Generating boggle board...".green().blink());

    let board: Vec<Vec<char>> = boggle::initialize_board();
    let dict: Trie<u8> = boggle::build_dict();
    //Store a list of user's found words
    let mut found_words: Vec<String> = Vec::new();
    //Store the solution to the board
    let solution = boggle::solve(&board, &dict);

    // boggle::print_board(&board);
    print_board(&board);

    help();

    //Keep reading input until they quit the game
    let mut input: String = String::from("");
    while input != "quit-game" {
        println!("{}", "Enter a word".cyan());
        input = read!();

        if input == "quit-game" {
            std::process::exit(0)
        }

        else if input == "help" {
            help();
        }

        else if input == "my-words" {
            println!("You have found {:1} words {:?}", found_words.len(), found_words);
        }

        else if input == "solve-game" {
            println!("Found {:1} words {:?}", solution.len(), solution);
        }

        else if input == "print-board" {
            boggle::print_board(&board);
        }

        // If board contains the input word
        else if solution.contains(&input.to_uppercase()) {
            println!("{}", "That is a word!".green());
            if !found_words.contains(&input) {
                found_words.push(input.clone());
            } else {
                println!("{}", "Word already found.".red());
            }
        } else {
            println!("{}", "That is not a real word!".red());
        }
    }
}

fn help() {
    println!("{}", "Available Commands:".blue().underline());
    println!("{} {}", "quit-game:".blue() , "Quits the Boggle game. It is not saved.");
    println!("{} {}", "solve-game:".blue(), "Prints the solution to the board.");
    println!("{} {}", "my-words:".blue(), "Prints the words you've found.");
    println!("{} {}", "print-board:".blue(), "Prints the board.");
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board.iter() {
        // let str_row: String = row.into_iter().collect();
        //TODO print the board nicer
        println!("{:?}", row);
    }
}
