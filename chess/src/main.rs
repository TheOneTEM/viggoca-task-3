use std::{array, thread::AccessError};

fn main() {
    let mut fen:String = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();
    println!("{}", fen);
    fen = game_turn(fen, "".to_string());
    println!("{}", fen);
    println!("{:?}", board(fen.clone()));
    let i = checklegalmoves(fen.clone(), actiontranslation("e4 d6".to_string()));
    fen = retranslate(fen);
    println!("{}", fen);
    println!("{:?}", actiontranslation("e4 d6".to_string()));
    
}
fn game_turn(mut fen:String, action: String) -> String{
    fen = translate(fen);
    let actiontrans: [[usize; 2]; 2] = actiontranslation(action);
    return fen;
}

fn translate(fen:String) -> String{
    let mut board:String = "".to_string();
    let mut boardstop:bool = false;
    for i in fen.chars() {
        if i == ' ' && boardstop == false{
            boardstop = true;
            board += &i.to_string()
        }
        else if boardstop == false {
            match i {
                '8' => board += "--------",
                '7' => board += "-------",
                '6' => board += "------",
                '5' => board += "-----",
                '4' => board += "----",
                '3' => board += "---",
                '2' => board += "--",
                '1' => board += "-",
                _ => board += &i.to_string(),
            }
        }
        else {
            board += &i.to_string()
        }
    }
    return board;
}
fn retranslate(fen:String) -> String {
    let mut board:String = "".to_string();
    let mut boardstop:bool = false;
    for i in fen.chars() {
        if i == ' ' && boardstop == false{
            boardstop = true;
            board += &i.to_string();
            board = board.replace("--------", "8");
            board = board.replace("-------", "7");
            board = board.replace("------", "6");
            board = board.replace("-----", "5");
            board = board.replace("----", "4");
            board = board.replace("---", "3");
            board = board.replace("--", "2");
            board = board.replace("-", "1");
        }
        else {
            board += &i.to_string()
        }
    }
    return board;
}
fn actiontranslation(action: String) -> [[usize; 2]; 2] {
    let mut translated: [[usize; 2]; 2] = [[0, 0], [0, 0]];
    let mut which:usize = 0;
    for i in action.chars() {
        if i == ' ' {
            which += 1;
        }
        else {
            match i {
                'a' => translated[which][0] += 0,
                'b' => translated[which][0] += 1,
                'c' => translated[which][0] += 2,
                'd' => translated[which][0] += 3,
                'e' => translated[which][0] += 4,
                'f' => translated[which][0] += 5,
                'g' => translated[which][0] += 6,
                'h' => translated[which][0] += 7,
                _ => translated[which][1] += 8 - (i as usize - 0x30),
            }
        }
    }
    return translated
}
fn board(fen:String) -> [[char; 8]; 8] {
    let mut board: [[char; 8]; 8] = [['-'; 8]; 8];
    let mut lat = 0;
    let mut long = 0;
    for i in fen.chars() {
        if i == ' ' {break}
        else if i == '/' {
            lat += 1;
            long = 0;
            println!("d");
        }
        else {
            board[long][lat] = i;
            long += 1;
            print!("{} ", i);
        }
    }
    return board;
}

fn checklegalmoves(mut fen:String, action: [[usize; 2]; 2]) -> bool {
    let turn: char = fen.split_off(72).split_at(1).0.parse::<char>().unwrap();
    let mut board: [[char; 8]; 8] = board(fen);
    println!("{:?}", board);
    println!("{:?}", action);
    println!("{}", action[0][0]);
    println!("{}", action[0][1]);
    println!("{}", board[action[0][0]][action[0][1]]);
    println!("{}", board[5][5]);
    if board[action[0][0]][action[0][1]] == 'P'   {
        println!("dww");
        return legalpawn(board, action, turn);
    }
    else if board[action[0][0]][action[0][1]] == 'N' || board[action[0][0]][action[0][1]] == 'n' {

    }
    return false;
}

fn legalpawn(board:[[char; 8]; 8], action: [[usize; 2]; 2], turn: char) -> bool {
    
    
    if turn == 'w' {

        if action[0][1] == 6 {

        }
    }
    
    return false;
}

fn legalknight(board:[[char; 8]; 8], action: [[usize; 2]; 2]) -> bool {


    return false;
}