use std::{array, thread::AccessError};
// todo castling check checkmate and and maby alpasante
fn main() {
    let mut fen:String = "rnbqkbnr/pppppppp/1P4P1/8/8/1p4p1/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen2:String = "rnbqkbnr/pppppppp/1P4P1/8/8/1p4p1/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // bishop
    let mut fen3:String = "rnbqkbnr/pppppppp/5p2/2p1P3/3B4/2p1p3/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen4:String = "rnbqkbnr/pppppppp/5P2/2P5/3b4/2P1P3/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // rook
    let mut fen5:String = "rnbqkbnr/pppppppp/1p6/1p6/pR5p/1p6/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen6:String = "rnbqkbnr/pppppppp/1P6/1p6/Pr5P/1P6/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // queen
    let mut fen7:String = "rnbqkbnr/pppppppp/5p2/2p1P3/3Q4/2p1p3/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen8:String = "rnbqkbnr/pppppppp/5P2/2P5/3q4/2P1P3/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // 
    let mut fen9:String = "rnbqkbnr/pppppppp/1p6/1p6/pQ5p/1p6/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen10:String = "rnbqkbnr/pppppppp/1P6/1p6/Pq5P/1P6/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // king
    let mut fen11:String = "rnbqkbnr/pppppppp/5p2/2p1P3/3K4/2p1p3/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen12:String = "rnbqkbnr/pppppppp/5P2/2P5/3k4/2P1P3/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // 
    let mut fen13:String = "rnbqkbnr/pppppppp/1p6/1p6/pK5p/1p6/PPPPPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen14:String = "rnbqkbnr/pppppppp/1P6/1p6/Pk5P/1P6/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string();
    // knight
    let mut fen15:String = "rnbqkbnr/pppppppp/1p1p4/p3p3/2NP4/p3p3/PpPpPPPP/RNBQKBNR w KQkq e3 0 1".to_string();
    let mut fen16:String = "rnbqkbnr/pppppppp/1P1P4/P3P3/2np4/P3P3/pbpbpppp/RNBQKBNR b KQkq e3 0 1".to_string();
    //
    println!("{}", fen);
    fen = game_turn(fen, "".to_string());
    fen2 = game_turn(fen2, "".to_string());
    fen3 = game_turn(fen3, "".to_string());
    fen4 = game_turn(fen4, "".to_string());
    fen5 = game_turn(fen5, "".to_string());
    fen6 = game_turn(fen6, "".to_string());
    fen7 = game_turn(fen7, "".to_string());
    fen8 = game_turn(fen8, "".to_string());
    fen9 = game_turn(fen9, "".to_string());
    fen10 = game_turn(fen10, "".to_string());
    fen11 = game_turn(fen11, "".to_string());
    fen12 = game_turn(fen12, "".to_string());
    fen13 = game_turn(fen13, "".to_string());
    fen14 = game_turn(fen14, "".to_string());
    fen15 = game_turn(fen15, "".to_string());
    fen16 = game_turn(fen16, "".to_string());
    println!("{}", fen);
    println!("{:?}", board(fen.clone()));
    let mut i = checklegalmoves(fen.clone(), actiontranslation("h2 h4".to_string()));
    if true == false {
        i = checklegalmoves(fen.clone(), actiontranslation("h2 g3".to_string()));
        i = checklegalmoves(fen.clone(), actiontranslation("a2 b3".to_string()));
        i = checklegalmoves(fen2.clone(), actiontranslation("h7 g6".to_string()));
        i = checklegalmoves(fen2.clone(), actiontranslation("a7 b6".to_string()));
    }
    if true == false {
        println!("bishop");
        i = checklegalmoves(fen3.clone(), actiontranslation("d4 c5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen3.clone(), actiontranslation("d4 f6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen3.clone(), actiontranslation("d4 c3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen3.clone(), actiontranslation("d4 e3".to_string()));
        println!("{}", i);
        println!("bishop");
        i = checklegalmoves(fen4.clone(), actiontranslation("d4 c5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen4.clone(), actiontranslation("d4 f6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen4.clone(), actiontranslation("d4 c3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen4.clone(), actiontranslation("d4 e3".to_string()));
        println!("{}", i);
    }
    if true == false {
        println!("rook");
        i = checklegalmoves(fen5.clone(), actiontranslation("b4 a4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen5.clone(), actiontranslation("b4 h4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen5.clone(), actiontranslation("b4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen5.clone(), actiontranslation("b4 b3".to_string()));
        println!("{}", i);
        println!("rook");
        i = checklegalmoves(fen6.clone(), actiontranslation("b4 a4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen6.clone(), actiontranslation("b4 h4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen6.clone(), actiontranslation("b4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen6.clone(), actiontranslation("b4 b3".to_string()));
        println!("{}", i);
    }
    if true == false {
        println!("queen, bishop");
        i = checklegalmoves(fen7.clone(), actiontranslation("d4 c5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen7.clone(), actiontranslation("d4 f6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen7.clone(), actiontranslation("d4 c3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen7.clone(), actiontranslation("d4 e3".to_string()));
        println!("{}", i);
        println!("queen, bishop");
        i = checklegalmoves(fen8.clone(), actiontranslation("d4 c5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen8.clone(), actiontranslation("d4 f6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen8.clone(), actiontranslation("d4 c3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen8.clone(), actiontranslation("d4 e3".to_string()));
        println!("{}", i);
        println!("queen, rook");
        i = checklegalmoves(fen9.clone(), actiontranslation("b4 a4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen9.clone(), actiontranslation("b4 h4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen9.clone(), actiontranslation("b4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen9.clone(), actiontranslation("b4 b3".to_string()));
        println!("{}", i);
        println!("queen, rook");
        i = checklegalmoves(fen10.clone(), actiontranslation("b4 a4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen10.clone(), actiontranslation("b4 h4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen10.clone(), actiontranslation("b4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen10.clone(), actiontranslation("b4 b3".to_string()));
        println!("{}", i);
    }
    if true == false {
        println!("king, bishop");
        i = checklegalmoves(fen11.clone(), actiontranslation("d4 c5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen11.clone(), actiontranslation("d4 f6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen11.clone(), actiontranslation("d4 c3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen11.clone(), actiontranslation("d4 e3".to_string()));
        println!("{}", i);
        println!("king, bishop");
        i = checklegalmoves(fen12.clone(), actiontranslation("d4 c5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen12.clone(), actiontranslation("d4 f6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen12.clone(), actiontranslation("d4 c3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen12.clone(), actiontranslation("d4 e3".to_string()));
        println!("{}", i);
        println!("king, rook");
        i = checklegalmoves(fen13.clone(), actiontranslation("b4 a4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen13.clone(), actiontranslation("b4 h4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen13.clone(), actiontranslation("b4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen13.clone(), actiontranslation("b4 b3".to_string()));
        println!("{}", i);
        println!("king, rook");
        i = checklegalmoves(fen14.clone(), actiontranslation("b4 a4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen14.clone(), actiontranslation("b4 h4".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen14.clone(), actiontranslation("b4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen14.clone(), actiontranslation("b4 b3".to_string()));
        println!("{}", i);
    }
    if true == false {
        println!("vertical");
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 b2".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 d2".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 d6".to_string()));
        println!("{}", i);
        println!("horisontal");
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 a3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 e3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 a5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen15.clone(), actiontranslation("c4 e5".to_string()));
        println!("{}", i);

        println!("vertical");
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 b2".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 d2".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 b6".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 d6".to_string()));
        println!("{}", i);
        println!("horisontal");
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 a3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 e3".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 a5".to_string()));
        println!("{}", i);
        i = checklegalmoves(fen16.clone(), actiontranslation("c4 e5".to_string()));
        println!("{}", i);
    }
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
            println!("")
        }
        else {
            board[long][lat] = i;
            long += 1;
            print!("{}", i)
        }
    }
    println!("");
    return board;
}

fn checklegalmoves(mut fen:String, action: [[usize; 2]; 2]) -> bool {
    let turn: char = fen.split_off(72).split_at(1).0.parse::<char>().unwrap();
    let mut board: [[char; 8]; 8] = board(fen);
    if (board[action[0][0]][action[0][1]] == 'P' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'p' && turn == 'b') {
        return legalpawn(board, action, turn);
    }
    else if (board[action[0][0]][action[0][1]] == 'N' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'n' && turn == 'b') {
        return legalknight(board, action)
    }
    else if (board[action[0][0]][action[0][1]] == 'B' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'b' && turn == 'b') {
        return legalbishop(board, action)
    }
    else if (board[action[0][0]][action[0][1]] == 'R' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'r' && turn == 'b') {
        return legalrook(board, action)
    }
    else if (board[action[0][0]][action[0][1]] == 'Q' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'q' && turn == 'b') {
        return legalqueen(board, action)
    }
    else if (board[action[0][0]][action[0][1]] == 'K' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'k' && turn == 'b') {
        return legalking(board, action);
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
    let movedistanse_long = action[0][0] as i32 - action[1][0] as i32;
    let movedistanse_lat = action[0][1] as i32 - action[1][1] as i32;

    if (movedistanse_lat == 2 || movedistanse_lat == -2) && (movedistanse_long == 1 || movedistanse_long == -1) {
        if board[action[1][0]][action[1][1]] == '-' || board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
            return true;
        }
    }
    else if (movedistanse_lat == 1 || movedistanse_lat == -1) && (movedistanse_long == 2 || movedistanse_long == -2) {
        if board[action[1][0]][action[1][1]] == '-' || board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
            return true;
        }
    }
    return false;
}

fn legalbishop(board:[[char; 8]; 8], action:[[usize; 2]; 2]) -> bool {
    let movedistanse_long = action[0][0] as i32 - action[1][0] as i32;
    let movedistanse_lat = action[0][1] as i32 - action[1][1] as i32;
    let mut i = 1;

    if movedistanse_long == movedistanse_lat || movedistanse_long == -movedistanse_lat {
        while i * i <= movedistanse_lat * movedistanse_lat {
            if movedistanse_lat < 0 && movedistanse_long < 0 {
                if board[action[0][0] + i as usize][action[0][1] + i as usize] == '-' {
                    i += 1;
                }
                else if action[0][0] + i as usize == action[1][0] && action[0][1] + i as usize == action[1][1] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    break;
                }
            }
            else if movedistanse_lat > 0 && movedistanse_long < 0 {
                if board[action[0][0] + i as usize][action[0][1] - i as usize] == '-' {
                    i += 1;
                }
                else if action[0][0] + i as usize == action[1][0] && action[0][1] - i as usize == action[1][1] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    break;
                }
            }
            else if movedistanse_lat < 0 && movedistanse_long > 0 {
                if board[action[0][0] - i as usize][action[0][1] + i as usize] == '-' {
                    i += 1;
                }
                else if action[0][0] - i as usize == action[1][0] && action[0][1] + i as usize == action[1][1] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    break;
                }
            }
            else if movedistanse_lat > 0 && movedistanse_long > 0 {
                if board[action[0][0] - i as usize][action[0][1] - i as usize] == '-' {
                    i += 1;
                }
                else if action[0][0] - i as usize == action[1][0] && action[0][1] - i as usize == action[1][1] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    break;
                }
            }
        }
    }
    return false;
}

fn legalrook(board:[[char; 8]; 8], action:[[usize; 2]; 2]) -> bool {
    let movedistanse_long = action[0][0] as i32 - action[1][0] as i32;
    let movedistanse_lat = action[0][1] as i32 - action[1][1] as i32;
    let mut i = 1;

    if movedistanse_lat == 0 && movedistanse_long != 0 {
        while i * i <= movedistanse_long * movedistanse_long {
            if movedistanse_long > 0 {
                if board[action[0][0] - i as usize][action[0][1]] == '-' {
                    i += 1;
                }
                else if action[0][0] - i as usize == action[1][0] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    return false;
                }
            }
            else if movedistanse_long < 0 {
                if board[action[0][0] + i as usize][action[0][1]] == '-' {
                    i += 1;
                }
                else if action[0][0] + i as usize == action[1][0] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    return false;
                }
            }
        }
    }
    else if movedistanse_long == 0 {
        while i * i <= movedistanse_lat * movedistanse_lat {
            if movedistanse_lat > 0 {
                if board[action[0][0]][action[0][1] - i as usize] == '-' {
                    i += 1;
                }
                else if action[0][1] - i as usize == action[1][1] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    return false;
                }
            }
            if movedistanse_lat < 0 {
                if board[action[0][0]][action[0][1] + i as usize] == '-' {
                    i += 1;
                }
                else if action[0][1] + i as usize == action[1][1] {
                    if board[action[1][0]][action[1][1]] != '-' && board[action[0][0]][action[0][1]].is_uppercase() != board[action[1][0]][action[1][1]].is_uppercase() {
                        return true;
                    }
                    else if board[action[1][0]][action[1][1]] == '-' {
                        return true;
                    }
                }
                else {
                    return false;
                }
            }
        }
        
    }
    return false;
}

fn legalqueen(board:[[char; 8]; 8], action:[[usize; 2]; 2]) -> bool {
    if legalbishop(board, action) == true || legalrook(board, action) == true {
        return true;
    }
    else {
        return false;
    }
}

fn legalking(board:[[char; 8]; 8], action:[[usize; 2]; 2]) -> bool {
    let movedistanse_long = action[0][0] as i32 - action[1][0] as i32;
    let movedistanse_lat = action[0][1] as i32 - action[1][1] as i32;

    if movedistanse_lat < -1 || movedistanse_lat > 1 || movedistanse_long < -1 || movedistanse_long > 1 {
        return false;
    }
    else {
        legalqueen(board, action)
    }
}

fn check() {

}