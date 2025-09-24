use std::{array, thread::AccessError};

fn main() {
    let mut fen:String = "rnbqkbnr/pppppppp/1P4P1/8/8/1p4p1/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen2:String = "rnbqkbnr/pppppppp/1P4P1/8/8/1p4p1/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    println!("{}", fen);
    fen = game_turn(fen, "".to_string());
    fen2 = game_turn(fen2, "".to_string());
    println!("{}", fen);
    println!("{:?}", board(fen.clone()));
    let mut i = checklegalmoves(fen.clone(), actiontranslation("h2 h4".to_string()));
    i = checklegalmoves(fen.clone(), actiontranslation("h2 g3".to_string()));
    i = checklegalmoves(fen.clone(), actiontranslation("a2 b3".to_string()));
    i = checklegalmoves(fen2.clone(), actiontranslation("h7 g6".to_string()));
    i = checklegalmoves(fen2.clone(), actiontranslation("a7 b6".to_string()));
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
        }
        else {
            board[long][lat] = i;
            long += 1;
        }
    }
    return board;
}

fn checklegalmoves(mut fen:String, action: [[usize; 2]; 2]) -> bool {
    let turn: char = fen.split_off(72).split_at(1).0.parse::<char>().unwrap();
    let mut board: [[char; 8]; 8] = board(fen);
    if (board[action[0][0]][action[0][1]] == 'P' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'p' && turn == 'b') {
        println!("dwas");
        return legalpawn(board, action, turn);
    }
    else if (board[action[0][0]][action[0][1]] == 'N' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'n' && turn == 'b') {

    }
    else if (board[action[0][0]][action[0][1]] == 'B' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'b' && turn == 'b') {

    }
    else if (board[action[0][0]][action[0][1]] == 'R' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'r' && turn == 'b') {

    }
    else if (board[action[0][0]][action[0][1]] == 'Q' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'q' && turn == 'b') {

    }
    else if (board[action[0][0]][action[0][1]] == 'K' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'k' && turn == 'b') {

    }
    return false;
}

// can't allpasante
fn legalpawn(board:[[char; 8]; 8], action:[[usize; 2]; 2], turn: char) -> bool {
    let movedistanse_long = action[0][0] as i32 - action[1][0] as i32;
    let movedistanse_lat = action[0][1] as i32 - action[1][1] as i32;
    println!("{}", movedistanse_lat);
    println!("{}", movedistanse_long);
    println!("{:?}", action);
    if movedistanse_long > 1 || movedistanse_long < -1 || movedistanse_lat > 2 || movedistanse_lat < -2 {
        return false;
    }
    else if turn == 'w' && movedistanse_lat > 0{
        if action[0][1] == 6 && action[1][1] == 4 && movedistanse_long == 0 && board[action[0][0]][action[0][1] - 1] == '-' && board[action[0][0]][action[0][1] - 2] == '-' {
            return true;
        }
        else if movedistanse_long == 0 && movedistanse_lat == 1 && board[action[0][0]][action[0][1] - 1] == '-' {
            return true;
        }
        else if movedistanse_lat == 1 && (movedistanse_long == 1 || movedistanse_long == -1){
            if movedistanse_long == 1 && action[0][0] != 0 && board[action[0][0] - 1][action[0][1] - 1].is_uppercase() == false && board[action[0][0] - 1][action[0][1] - 1] != '-' {
                return true;
            }
            else if movedistanse_long == -1 && action[0][0] != 7 && board[action[0][0] + 1][action[0][1] - 1].is_uppercase() == false && board[action[0][0] + 1][action[0][1] - 1] != '-' {
                return true;
            }
        }

    }
    else if turn == 'b' && movedistanse_lat < 0{
        if action[0][1] == 1 && action[1][1] == 3 && movedistanse_long == 0 && board[action[0][0]][action[0][1] + 1] == '-' && board[action[0][0]][action[0][1] + 2] == '-' {
            return true;
        }
        else if movedistanse_long == 0 && movedistanse_lat == -1 && board[action[0][0]][action[0][1] + 1] == '-' {
            return true;
        }
        else if movedistanse_lat == -1 && (movedistanse_long == 1 || movedistanse_long == -1){
            if movedistanse_long == 1 && action[0][0] != 0 && board[action[0][0] - 1][action[0][1] + 1].is_uppercase() == true && board[action[0][0] - 1][action[0][1] + 1] != '-' {
                return true;
            }
            else if movedistanse_long == -1 && action[0][0] != 7 && board[action[0][0] + 1][action[0][1] + 1].is_uppercase() == true && board[action[0][0] + 1][action[0][1] + 1] != '-' {
                return true;
            }
        }
    }
    return false;
}

fn legalknight(board:[[char; 8]; 8], action:[[usize; 2]; 2]) -> bool {
    

    return false;
}

fn legalbishop(board:[[char; 8]; 8], action:[[usize; 2]; 2]) -> bool {
    if action[0][0] - action[1][0] == action[0][1] - action[1][1] {
        
    }

    return false;
}