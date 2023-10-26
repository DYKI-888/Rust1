use getch_rs::{Getch, Key};

const BACK: Option<bool> = Some(false); // 裏
const FRONT: Option<bool> = Some(true); // 表
const WALL: Option<bool> = None; // 壁

//　カーソルの座標
struct Position {
    y: usize,
    x: usize,
}

struct Game {
    pos: Position,
    field: [[Option<bool>; 11]; 11],
}

impl Game {
    // 初期化
    fn new() -> Self {
        use self::{BACK as B, WALL as W};

        Self {
            pos: Position { y: 5, x: 5 },
            field: [
                [W, W, W, W, W, W, W, W, W, W, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, B, B, B, B, B, B, B, B, B, W],
                [W, W, W, W, W, W, W, W, W, W, W],
            ],
        }
    }
}

// 選択している位置を中心にマスを十字に裏返す
fn turn_over(Game { pos, field }: &mut Game) {
    let y = pos.y;
    let x = pos.x;

    let list: [(usize, usize); 5] = [(y, x), (y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

    for i in list {
        if let Some(value) = field[i.0][i.1] {
            field[i.0][i.1] = Some(!value);
        }
    }
}

// 全てのマスが表がならtrue、そうでないならfalse
fn all_front_check(Game { field, .. }: &Game) -> bool {
    for check in field {
        for i in 0..check.len() {
            if check[i] == Some(false) {
                return false;
            }
        }
    }
    return true;
}

// 描画
fn draw(Game { pos, field }: &Game) {
    // 画面クリア
    println!("\x1b[2J\x1b[H");

    for y in 1..10 {
        for x in 1..10 {
            if pos.y == y && pos.x == x {
                print!("🟥")
            } else if field[y][x] == FRONT {
                print!("⬜️")
            } else if field[y][x] == BACK {
                print!("⬛️")
            } else {
                print!("  ")
            }
        }
        println!()
    }
}

fn main() {
    let mut game = Game::new();

    let key = Getch::new();

    // カーソルの削除
    print!("\x1b[?25l");

    // メインループ
    loop {
        draw(&game);

        match key.getch() {
            // 左移動
            Ok(Key::Char('a')) | Ok(Key::Left) => {
                if game.pos.x != 1 {
                    game.pos.x -= 1
                }
            }
            // 右移動
            Ok(Key::Char('d')) | Ok(Key::Right) => {
                if game.pos.x != 9 {
                    game.pos.x += 1
                }
            }
            // 上移動
            Ok(Key::Char('w')) | Ok(Key::Up) => {
                if game.pos.y != 1 {
                    game.pos.y -= 1
                }
            }
            // 下移動
            Ok(Key::Char('s')) | Ok(Key::Down) => {
                if game.pos.y != 9 {
                    game.pos.y += 1
                }
            }
            // 裏返す
            Ok(Key::Char(' ')) => turn_over(&mut game),
            // 終了
            Ok(Key::Char('q')) => break,
            _ => (),
        }

        // 全て表ならループを終了する
        if all_front_check(&game) {
            break;
        }
    }

    // カーソルを表示する
    println!("\x1b[?25h");
}
