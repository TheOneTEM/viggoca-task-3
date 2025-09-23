use std::{array, thread::AccessError};

fn main() {
    let mut fen:String = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string();
    println!("{}", fen);
    fen = game_turn(fen, "".to_string());
    println!("{}", fen);
    fen = retranslate(fen);
    println!("{}", fen);
}
fn game_turn(mut fen:String, action: String) -> String{
    fen = translate(fen);
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

fn checkleagalmoves(fen:String, action: String) -> bool {
    
    for i in fen.chars() {

    }
}
fn actiontranslation(action: String) -> [i32 ; 2] {
    let mut translated: [i32; 2] = [0, 0];
    

    return translated
}