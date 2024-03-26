use crossterm::event::KeyCode;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::process::exit;
use std::process::Command;
//use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let ( game, mut direction) = init();
    let game = Arc::new(Mutex::new(game));
    let game_clone = Arc::clone(&game);

    let mut score: u32 = 0;
    // thread for music
    thread::spawn(|| {
        loop {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let file = BufReader::new(File::open("music.mp3").unwrap());
            let source = Decoder::new(file).unwrap();
            let _ = stream_handle.play_raw(source.convert_samples());
            thread::sleep(Duration::from_secs(183));
        }
    });

    thread::spawn(move || loop {
        if score % 50 == 0 && score != 0 {
            clear_screen();
        }
        score = score + 1;
        let cloned_game = Arc::clone(&game_clone);
        let mut game = cloned_game.lock().unwrap();

        print_game(&game, &score);

        *game = ball_move(&mut game.clone(), &mut direction);
        drop(game);

        let time: u64 = {
            if score < 50 {
                250
            } else if score < 100 {
                200
            } else if score < 150 {
                150
            } else {
                100
            }
        };
        sleep(Duration::from_millis(time));
    });

    loop {
        let cloned_game = Arc::clone(&game);

        while let Ok(event_result) = crossterm::event::poll(Duration::from_millis(100)) {
            if event_result {
                if let crossterm::event::Event::Key(event) = crossterm::event::read().unwrap() {
                    let mut game = cloned_game.lock().unwrap();
                    match event.code {
                        KeyCode::Left => *game = move_left(&mut game),
                        KeyCode::Right => *game = move_right(&mut game),
                        _ => (),
                    }
                }
            }
        }
    }
}

fn print_game(game: &Vec<Vec<u8>>, score: &u32) {
    for i in game.iter() {
        for j in i.iter() {
            if j == &0 {
                print!(" ");
            }
            if j == &1 {
                print!("#");
            }
            if j == &2 {
                print!("_");
            }
            if j == &3 {
                print!("O");
            }
            if j == &4 {
                print!("|");
            }
        }

        println!("");
    }
    print!("Score: {}", score);
    println!("");
}

fn move_left(game: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for i in 0..game[29].len() {
        if game[29][i] == 2 && i > 0 {
            game[29][i - 1] = 2;
            game[29][i] = 0;
        }
    }
    game.to_vec()
}

fn move_right(game: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    for i in (0..game[29].len()).rev() {
        if game[29][i] == 2 && i < game[29].len() - 1 {
            game[29][i + 1] = 2;
            game[29][i] = 0;
        }
    }
    game.to_vec()
}

fn ball_move(game: &mut Vec<Vec<u8>>, direction: &mut String) -> Vec<Vec<u8>> {
    let (i, j) = find_ball(&game);
   
    if i == 28 && game[i + 1][j] != 2 && game[i + 1][j + 1] != 2 && game[i + 1][j - 1] != 2 {
        lose();
    }

    // ! directions

    //dirs

    if i != 29
        && j != 0
        && game[i - 1][j] == 1
        && game[i - 1][j + 1] == 1
        && game[i - 1][j - 1] == 1
    {
        game[i - 1][j] = 0;
        game[i - 1][j + 1] = 0;
        game[i - 1][j - 1] = 0;
        *direction = String::from("down-left")
    }
    if i != 29 && j != 29 && game[i - 1][j + 1] == 1 {
        game[i - 1][j + 1] = 0;
        *direction = String::from("down-right")
    }
    if i != 0 && j != 0 && game[i + 1][j - 1] == 1 {
        game[i + 1][j - 1] = 0;
        *direction = String::from("up-left")
    }

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
    if i != 29 && game[i + 1][j + 1] == 2 && game[i + 1][j - 1] == 2 {
        *direction = String::from("up");
    }
    if i != 29 && game[i + 1][j] == 2 && game[i + 1][j - 1] == 0 {
        *direction = String::from("up-left");
    }
    if i != 29 && game[i + 1][j] == 2 && game[i + 1][j + 1] == 0 {
        *direction = String::from("up-right");
    }
    if i == 28 && game[i + 1][j - 1] == 2 && game[i + 1][j] != 2 {
        *direction = String::from("up-left");
    }
    if i == 28 && game[i + 1][j + 1] == 2 && game[i + 1][j] != 2 {
        *direction = String::from("up-right");
    }

    //double directions

    if i != 0 && j != 0 && game[i + 1][j - 1] == 1 {
        game[i - 1][j - 1] = 0;
        *direction = String::from("up-right")
    }
    if i != 0 && j != 29 && game[i + 1][j + 1] == 1 {
        game[i - 1][j + 1] = 0;
        *direction = String::from("up-left")
    }
    if i != 29 && j != 29 && game[i + 1][j + 1] == 1 {
        game[i + 1][j + 1] = 0;
        *direction = String::from("down-left")
    }
    if i != 29 && j != 0 && game[i - 1][j - 1] == 1 {
        game[i + 1][j - 1] = 0;
        *direction = String::from("down-right")
    }

    //borders

    if i == 0 {
        *direction = String::from("down")
    }
    if j == 29 {
        *direction = String::from("down-left")
    }
    if j == 0 {
        *direction = String::from("down-right")
    }

    // corners
    if i == 0 && j == 0 {
        *direction = String::from("down-right")
    }
    if i == 0 && j == 29 {
        *direction = String::from("down-left")
    }
    if i != 29 && j != 0 && i != 0 && j != 29 && game[i - 1][j - 1] == 1 {
        *direction = String::from("down-right")
    }
    if i != 29 && j != 0 && i != 0 && j != 29 && game[i + 1][j + 1] == 1 {
        *direction = String::from("down-left")
    }
    // moves

    if i != 0 && *direction == "up" {
        if game[i - 1][j] != 4 {
            game[i - 1][j] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("down");
        }
    }
    if i != 29 && *direction == "down" {
        if game[i + 1][j] != 4 {
            game[i + 1][j] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("up");
        }
    }
    if j != 0 && *direction == "left" {
        if game[i][j - 1] != 4 {
            game[i][j - 1] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("down-right");
        }
    }
    if j != 29 && *direction == "right" {
        if game[i][j + 1] != 4 {
            game[i][j + 1] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("down-left");
        }
    }

    if i != 0 && j != 0 && *direction == "up-left" {
        if game[i - 1][j - 1] != 4 {
            game[i - 1][j - 1] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("up-right");
        }
    }
    if i != 0 && j != 29 && *direction == "up-right" {
        if game[i - 1][j + 1] != 4 {
            game[i - 1][j + 1] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("up-left");
        }
    }
    if i != 29 && j != 29 && *direction == "down-right" {
        if game[i + 1][j + 1] != 4 {
            game[i + 1][j + 1] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("down-left");
        }
    }
    if i != 29 && j != 0 && *direction == "down-left" {
        if game[i + 1][j - 1] != 4 {
            game[i + 1][j - 1] = 3;
            game[i][j] = 0;
        } else {
            *direction = String::from("down-right");
        }
    }

    game.to_vec()
}

fn find_ball(game: &Vec<Vec<u8>>) -> (usize, usize) {
    for i in 0..29 {
        for j in 0..29 {
            if game[i][j] == 3 {
                return (i as usize, j as usize);
            }
        }
    }
    (0, 0)
}

fn init() -> (Vec<Vec<u8>>, String) {
    let mut game: Vec<Vec<u8>> = vec![vec![0; 30]; 30];

    let bricks: Vec<Vec<u8>> = vec![vec![1; 24]; 7];

    let mut bar: Vec<u8> = vec![0; 30];
    for i in 0..30 {
        if i == 14 || i == 15 || i == 16 {
            bar[i] = 2;
        }
    }
    game[29] = bar;

    for i in 0..7 {
        for j in 0..24 {
            game[i + 3][j + 3] = bricks[i][j];
        }
    }

    let ball: Vec<u8> = vec![3];
    game[28][15] = ball[0];

    //borders
    for i in 0..game.len() {
        for j in 0..game[i].len() {
            if i == 0 || j == 0 || j == 29 {
                game[i][j] = 4;
            }
        }
    }

    let direction: String = String::from("up");

    (game, direction)
}

fn lose() {
    println!("");
    println!("YOU LOSE");
    thread::sleep(Duration::from_millis(3000));
    exit(0);
}

#[cfg(target_os = "windows")]
fn clear_screen() {
    let output = Command::new("cmd")
        .args(&["/C", "cls"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to clear screen");
}

#[cfg(not(target_os = "windows"))]
fn clear_screen() {
    let mut clear: Command = Command::new("clear");

    clear.spawn().unwrap();
}
