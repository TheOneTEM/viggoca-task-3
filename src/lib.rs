

// "Unit tests"

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
    fen = game_turn(fen, "h2 h5".to_string());
    println!("{}", fen);
    println!(" ");
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
pub fn game_turn(mut fen:String, action: String) -> String{
    fen = translate(fen);
    let actiontrans: [[usize; 2]; 2] = actiontranslation(action);
    if checklegalmoves(fen.clone(), actiontrans) == true {
        fen = makemove(fen, actiontrans);
    }
    else {
        fen = retranslate(fen);
    }
    fen = retranslate(fen);
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
        else if boardstop == true && (i == 'w' || i == 'b') {
            if i == 'w' {
                board += &"b".to_string();
            }
            else if i == 'b' {
                board += &"w".to_string();
            }
        }
        else{
            board += &i.to_string()
        }
    }
    return board;
}
fn actiontranslation(action:String) -> [[usize; 2]; 2] {
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
fn makemove(fen:String, action:[[usize; 2]; 2]) -> String{
    let originalpos: usize = 1 + action[0][0] + action[0][1] * 9;
    let newpos: usize = 1 + action[1][0] + action[1][1] * 9;
    let mut count: usize = 1;
    let mut piece: char = ' ';
    for i in fen.chars() {
        if count == originalpos {
            piece = i;
        }
        count += 1;
    }
    count = 1;
    let mut board:String = "".to_string();
    for i in fen.chars() {
        if count == newpos {
            board += &piece.to_string();
        }
        else if count == originalpos {
            board += "-";
        }
        else {
            board += &i.to_string();
        }
        count += 1;
    }
    return board;
}

fn checklegalmoves(mut fen:String, action:[[usize; 2]; 2]) -> bool {
    let turn: char = fen.split_off(72).split_at(1).0.parse::<char>().unwrap();
    let board: [[char; 8]; 8] = board(fen);
    if (board[action[0][0]][action[0][1]] == 'P' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'p' && turn == 'b') {
        if legalpawn(board, action, turn) == true && check(board, action, turn) == false {return true;}
    }
    else if (board[action[0][0]][action[0][1]] == 'N' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'n' && turn == 'b') {
        if legalknight(board, action) == true && check(board, action, turn) == false {return true;}
    }
    else if (board[action[0][0]][action[0][1]] == 'B' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'b' && turn == 'b') {
        if legalbishop(board, action) == true && check(board, action, turn) == false {return true;}
    }
    else if (board[action[0][0]][action[0][1]] == 'R' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'r' && turn == 'b') {
        if legalrook(board, action) == true && check(board, action, turn) == false {return true;}
    }
    else if (board[action[0][0]][action[0][1]] == 'Q' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'q' && turn == 'b') {
        if legalqueen(board, action) == true && check(board, action, turn) == false {return true;}
    }
    else if (board[action[0][0]][action[0][1]] == 'K' && turn == 'w') || (board[action[0][0]][action[0][1]] == 'k' && turn == 'b') {
        if legalking(board, action) == true && check(board, action, turn) == false {return true;}
    }
    return false;
}
// can't allpasante
fn legalpawn(board:[[char; 8]; 8], action:[[usize; 2]; 2], turn:char) -> bool {
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
    println!("wda");
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
// can't castle
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

fn check(mut board:[[char; 8]; 8], action:[[usize; 2]; 2], turn:char) -> bool {
    board[action[1][0]][action[1][1]] = board[action[0][0]][action[0][1]];
    board[action[0][0]][action[0][1]] = '-';
    let kingpos: [usize; 2] = findking(board, turn);
    if kingpos == [9, 9] {return false;}
    let knight: ((usize, usize), (usize, usize)) = ((1, 2), (2, 1));
    let mut i: usize = 0;
    if turn == 'w' {
        while i < 8 {
            //rook
            if (kingpos[0] + i < 8) && (board[kingpos[0] + i][kingpos[1]] == 'r' || board[kingpos[0] + i][kingpos[1]] == 'q') {
                return true;
            }
            if (kingpos[1] + i < 8) && (board[kingpos[0]][kingpos[1] + i] == 'r' || board[kingpos[0]][kingpos[1] + i] == 'q') {
                return true;
            }
            if (kingpos[0] as i32 - i as i32 >= 0) && (board[kingpos[0] - i][kingpos[1]] == 'r' || board[kingpos[0] - i][kingpos[1]] == 'q') {
                return true;
            }
            if (kingpos[1]  as i32 - i as i32 >= 0) && (board[kingpos[0]][kingpos[1] - i] == 'r' || board[kingpos[0]][kingpos[1] - i] == 'q') {
                return true;
            }
            //bishop
            if (kingpos[0] + i < 8) && (kingpos[1] + i < 8) && (board[kingpos[0] + i][kingpos[1] + i] == 'b' || board[kingpos[0] + i][kingpos[1] + i] == 'q') {
                return true;
            }
            if (kingpos[0]  as i32 - i as i32 >= 0) && (kingpos[1] + i < 8) && (board[kingpos[0] - i][kingpos[1] + i] == 'b' || board[kingpos[0] - i][kingpos[1] + i] == 'q') {
                return true;
            }
            if (kingpos[0] + i < 8) && (kingpos[1]  as i32 - i as i32 >= 0) && (board[kingpos[0] + i][kingpos[1] - i] == 'b' || board[kingpos[0] + i][kingpos[1] - i] == 'q') {
                return true;
            }
            if (kingpos[0]  as i32 - i as i32 >= 0) && (kingpos[1]  as i32 - i as i32 >= 0) && (board[kingpos[0] - i][kingpos[1] - i] == 'b' || board[kingpos[0] - i][kingpos[1] - i] == 'q') {
                return true;
            }
            i += 1;
        }
        i -= 1;
        //vertical knight
        if kingpos[0] + knight.0.0 < 8 && kingpos[1] + knight.0.1 < 8 && board[kingpos[0] + knight.0.0][kingpos[1] + knight.0.1] == 'n' {
            return true;
        }
        if kingpos[0] as i32 - knight.0.0 as i32 >= 0 && kingpos[1] + knight.0.1 < 8 && board[kingpos[0] - knight.0.0][kingpos[1] + knight.0.1] == 'n' {
            return true;
        }
        if kingpos[0] + knight.0.0 < 8 && kingpos[1] as i32 - knight.0.1 as i32 >= 0 && board[kingpos[0] + knight.0.0][kingpos[1] - knight.0.1] == 'n' {
            return true;
        }
        if kingpos[0] as i32 - knight.0.0 as i32 >= 0 && kingpos[1]as i32  - knight.0.1 as i32 >= 0 && board[kingpos[0] - knight.0.0][kingpos[1] - knight.0.1] == 'n' {
            return true;
        }
        //horisontal knight
        if kingpos[0] + knight.1.0 < 8 && kingpos[1] + knight.1.1 < 8 && board[kingpos[0] + knight.1.0][kingpos[1] + knight.1.1] == 'n' {
            return true;
        }
        if kingpos[0] as i32 - knight.1.0 as i32 >= 0 && kingpos[1] + knight.1.1 < 8 && board[kingpos[0] - knight.1.0][kingpos[1] + knight.1.1] == 'n' {
            return true;
        }
        if kingpos[0] + knight.1.0 < 8 && kingpos[1] as i32 - knight.1.1 as i32 >= 0 && board[kingpos[0] + knight.1.0][kingpos[1] - knight.1.1] == 'n' {
            return true;
        }
        if kingpos[0] as i32 - knight.1.0 as i32 >= 0 && kingpos[1] as i32 - knight.1.1 as i32 >= 0 && board[kingpos[0] - knight.1.0][kingpos[1] - knight.1.1] == 'n' {
            return true;
        }
        //pawn
        if kingpos[0] + 1 < 8 && kingpos[1] as i32 - 1 >= 0 && board[kingpos[0] + 1][kingpos[1] - 1] == 'p' {
            return true;
        }
        if kingpos[0] as i32 - 1 >= 0 && kingpos[1] as i32 - 1 >= 0 && board[kingpos[0] - 1][kingpos[1] - 1] == 'p' {
            return true;
        }
    }
    else if turn == 'b'{
        while i < 8 {
            //rook
            if (kingpos[0] + i < 8) && (board[kingpos[0] + i][kingpos[1]] == 'R' || board[kingpos[0] + i][kingpos[1]] == 'Q') {
                return true;
            }
            if (kingpos[1] + i < 8) && (board[kingpos[0]][kingpos[1] + 1] == 'R' || board[kingpos[0]][kingpos[1] + i] == 'Q') {
                return true;
            }
            if (kingpos[0] as i32 - i as i32 >= 0) && (board[kingpos[0] - i][kingpos[1]] == 'R' || board[kingpos[0] - i][kingpos[1]] == 'Q') {
                return true;
            }
            if (kingpos[1] as i32 - i as i32 >= 0) && (board[kingpos[0]][kingpos[1] - i] == 'R' || board[kingpos[0]][kingpos[1] - i] == 'Q') {
                return true;
            }
            //bishop
            if (kingpos[0] + i < 8) && (kingpos[1] + i < 8) && (board[kingpos[0] + i][kingpos[1] + i] == 'B' || board[kingpos[0] + i][kingpos[1] + i] == 'Q') {
                return true;
            }
            if (kingpos[0] as i32 - i as i32 >= 0) && (kingpos[1] + i < 8) && (board[kingpos[0] - i][kingpos[1] + i] == 'B' || board[kingpos[0] - i][kingpos[1] + i] == 'Q') {
                return true;
            }
            if (kingpos[0] + i < 8) && (kingpos[1] as i32 - i as i32 >= 0) && (board[kingpos[0] + i][kingpos[1] - i] == 'B' || board[kingpos[0] + i][kingpos[1] - i] == 'Q') {
                return true;
            }
            if (kingpos[0] as i32 - i as i32 >= 0) && (kingpos[1] as i32 - i as i32 >= 0) && (board[kingpos[0] - i][kingpos[1] - i] == 'B' || board[kingpos[0] - i][kingpos[1] - i] == 'Q') {
                return true;
            }
            i += 1;
        }
        //vertical knight
        if kingpos[0] + knight.0.0 < 8 && kingpos[1] + knight.0.1 < 8 && board[kingpos[0] + knight.0.0][kingpos[1] + knight.0.1] == 'N' {
            return true;
        }
        if kingpos[0] as i32 - knight.0.0 as i32 >= 0 && kingpos[1] + knight.0.1 < 8 && board[kingpos[0] - knight.0.0][kingpos[1] + knight.0.1] == 'N' {
            return true;
        }
        if kingpos[0] + knight.0.0 < 8 && kingpos[1] as i32 - knight.0.1 as i32 >= 0 && board[kingpos[0] + knight.0.0][kingpos[1] - knight.0.1] == 'N' {
            return true;
        }
        if kingpos[0] as i32 - knight.0.0 as i32 >= 0 && kingpos[1] as i32 - knight.0.1 as i32 >= 0 && board[kingpos[0] - knight.0.0][kingpos[1] - knight.0.1] == 'N' {
            return true;
        }
        //horisontal knight
        if kingpos[0] + knight.1.0 < 8 && kingpos[1] + knight.1.1 < 8 && board[kingpos[0] + knight.1.0][kingpos[1] + knight.1.1] == 'N' {
            return true;
        }
        if kingpos[0] as i32 - knight.1.0 as i32 >= 0 && kingpos[1] + knight.1.1 < 8 && board[kingpos[0] - knight.1.0][kingpos[1] + knight.1.1] == 'N' {
            return true;
        }
        if kingpos[0] + knight.1.0 < 8 && kingpos[1] as i32 - knight.1.1 as i32 >= 0 && board[kingpos[0] + knight.1.0][kingpos[1] - knight.1.1] == 'N' {
            return true;
        }
        if kingpos[0] as i32 - knight.1.0 as i32 >= 0 && kingpos[1] as i32 - knight.1.1 as i32 >= 0 && board[kingpos[0] - knight.1.0][kingpos[1] - knight.1.1] == 'N' {
            return true;
        }
        //pawn
        if kingpos[0] + 1 < 8 && kingpos[1] as i32 - 1 >= 0 && board[kingpos[0] + 1][kingpos[1] - 1] == 'P' {
            return true;
        }
        if kingpos[0] as i32 - 1 >= 0 && kingpos[1] as i32 - 1 >= 0 && board[kingpos[0] - 1][kingpos[1] - 1] == 'P' {
            return true;
        }
    }
    return false
}

fn findking(board:[[char; 8]; 8], turn:char) -> [usize; 2] {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < 8 {
        j = 0;
        while j < 8 {
            if (turn == 'w' && board[i][j] == 'K') || (turn == 'b' && board[i][j] == 'k') {
                return [i, j];
            }
            j += 1;
        }
        i += 1;
    }
    return [9, 9]
}