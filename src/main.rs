use std::process::exit;

use std::thread::sleep;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::process::Command;

fn main() {
    let (mut game, mut direction) = init();
    let mut game = Arc::new(Mutex::new(game));
    let game_clone = Arc::clone(&game);
    //Arc<Mutex<Vec<Vec<u8>>>>
    thread::spawn(move || {
        loop {
            let cloned_game = Arc::clone(&game_clone);
            let mut game = cloned_game.lock().unwrap();

            print_game(&game);
            /*let mut clear = Command::new("clear");
            clear.status().unwrap();*/
            *game = ball_move(&mut game.clone(), &mut direction);
            drop(game);

            sleep(Duration::from_millis(100));
        }
    });

    
loop{
    let cloned_game = Arc::clone(&game);
    
    let mut stdout = std::io::stdout()/*.into_raw_mode().unwrap()*/;
    let stdin = std::io::stdin();

    for key in stdin.keys() {
        if let Ok(event) = key {
            let mut game = cloned_game.lock().unwrap();
            match event {
                Key::Left => *game = move_left(&mut game),
                Key::Right => *game =  move_right(&mut game),
                _ => (),
            }
        }
    }
    drop(stdout);
}


}



fn print_game(game: &Vec<Vec<u8>>){
    for i in game.iter(){
        for j in i.iter(){
            if j == &0 {
                print!(" ");
            }
            if j == &1{
                print!("#");
            }
            if j == &2{
                print!("_");
            }
            if j == &3{
                print!("O");
            }
            if j == &4{
                print!("|");
            }
        }
        println!("");
    }
}

fn move_left(game: &mut Vec<Vec<u8>> ) -> Vec<Vec<u8>> {
    for i in 0..game[29].len() {
        if game[29][i] == 2 && i > 0 {
            game[29][i - 1] = 2;
            game[29][i] = 0;
        }
    }
    game.to_vec()
}

fn move_right(game: &mut Vec<Vec<u8>> ) -> Vec<Vec<u8>> {
    for i in (0..game[29].len()).rev() {
        if game[29][i] == 2 && i < game[29].len() - 1 {
            game[29][i + 1] = 2;
            game[29][i] = 0;
        }
    }
    game.to_vec()
}

fn ball_move(game: &mut Vec<Vec<u8>>, direction: &mut String) -> Vec<Vec<u8>> {
    let  (mut i, mut j) = find_ball(&game);
    // let (i,j) = (i as usize,j as usize);

    if i == 28 && game[i+1][j] != 2{
        lose();
    }

//directions

   // simple directions
if i != 29 && game[i + 1][j] == 1 {
    game[i + 1][j] = 0;
    *direction = String::from("down");
}
if i != 0 && game[i - 1][j] == 1 {
    game[i - 1][j] = 0;
    *direction = String::from("up");
}
if j != 29 && game[i][j + 1] == 1 {
    game[i][j + 1] = 0;
    *direction = String::from("left");
}
if j != 0 && game[i][j - 1] == 1 {
    game[i][j - 1] = 0;
    *direction = String::from("right");
}
if i != 29 && game[i + 1][j+1] == 2 && game[i+1][j-1] == 2{
    *direction = String::from("up");
}
if i != 29 && game[i + 1][j] == 2 && game[i+1][j-1] == 0 {
    *direction = String::from("up-left");
}
if i != 29 && game[i + 1][j] == 2 && game[i+1][j+1] == 0 {
    *direction = String::from("up-right");
}

    //double directions

    if i != 0 && j != 0 && game[i+1][j-1] == 1{
        game[i-1][j-1] = 0;
        *direction = String::from("up-right")
    }
    if i != 0 && j != 29 && game[i+1][j+1] == 1{
        game[i-1][j+1] = 0;
        *direction = String::from("up-left")
    }
    if i != 29 && j != 29 && game[i+1][j+1] == 1{
        game[i+1][j+1] = 0;
        *direction = String::from("down-left")
    }
    if i != 29 && j != 0 && game[i-1][j-1] == 1{
        game[i+1][j-1] = 0;
        *direction = String::from("down-right")
    }

    //dirs

    if i != 29 && j != 0 && game[i-1][j] == 1 && game[i-1][j+1] == 1 && game[i-1][j-1] == 1{
        game[i-1][j] = 0;
        game[i-1][j+1] = 0;
        game[i-1][j-1] = 0;
        *direction = String::from("down")
    }

    //borders

    if i == 0{
        *direction = String::from("down")
    }
    if j == 29{
        *direction = String::from("down-left")
    }
    if j == 0{
        *direction = String::from("down-right")
    }

    // corners
    if i == 0 && j == 0{
        *direction = String::from("down-right")
    }
    if i == 0 && j == 29{
        *direction = String::from("down-left")
    }

     // moves

     if i != 0 && *direction == "up" {
        if game[i-1][j] != 4{
            game[i-1][j] = 3;
            game[i][j] = 0;
        }
        else{
            *direction = String::from("down");
        }
    }
    if i != 29 && *direction == "down" {
        if game[i+1][j] != 4{
            game[i+1][j] = 3;
            game[i][j] = 0;
        }
        else{
            *direction = String::from("up");
        }
    }
    if j != 0 && *direction == "left"{
        if game[i][j-1] != 4{
            game[i][j-1] = 3;
            game[i][j] = 0;
        }
        else{
            *direction = String::from("down-right");
        }

    }
    if j != 29 && *direction == "right"{
        if game[i][j+1] != 4{
            game[i][j+1] = 3;
            game[i][j] = 0;
        }
        else{
            *direction = String::from("down-left");
        }

    }

    if i != 0 && j != 0 && *direction == "up-left"{
        if game[i-1][j-1] != 4{
        game[i-1][j-1] = 3;
        game[i][j] = 0;
        }
        else{
            *direction = String::from("up-right");
        }
    }
    if i != 0 && j != 29 && *direction == "up-right"{
        if game[i-1][j+1] != 4{
            game[i-1][j+1] = 3;
            game[i][j] = 0;
        }
        else{
            *direction = String::from("up-left");
        }

    }
    if i != 29 && j != 29 && *direction == "down-right"{
        if game[i+1][j+1] != 4{
        game[i+1][j+1] = 3;
        game[i][j] = 0;
        }
        else{
            *direction = String::from("down-left");
        }
    }
    if i != 29 && j != 0 && *direction == "down-left"{
        if game[i+1][j-1] != 4{
            game[i+1][j-1] = 3;
            game[i][j] = 0;
        }
        else{
            *direction = String::from("down-right");
        }
}


    game.to_vec()
}



fn find_ball(game: &Vec<Vec<u8>>) -> (usize, usize){
    for i in 0..29{
        for j in 0..29{
            if game[i][j] == 3{
                return (i as usize,j as usize);
            }
        }
    }
    (0,0)
}

fn init() -> (Vec<Vec<u8>>, String){
    let mut game: Vec<Vec<u8>> = vec![vec![0; 30]; 30];

    let bricks: Vec<Vec<u8>> = vec![vec![1;24]; 7];

    let mut bar: Vec<u8> = vec![0;30];
    for i in 0..30{
        if i == 14 || i == 15 || i == 16{
        
            bar[i] = 2;
        }
    }
    game[29] = bar;

    for i in 0..7 {
        for j in 0..24 {
            game[i+3][j+3] = bricks[i][j];
        }
    }

    let ball: Vec<u8> = vec![3];
    game [28][15] = ball[0];


    //borders
    for i in 0..game.len(){
        for j in 0..game[i].len(){
            if i == 0 || j == 0 || j == 29 {
                game[i][j] = 4;                                            }

        }
    }

    let mut direction: String = String::from("up");

    (game, direction)

}

fn lose(){
    println!("");
    println!("YOU LOSE");
   // sleep(Duration::from_millis(3000));
    exit(0);

}
