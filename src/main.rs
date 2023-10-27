use getch_rs::{Getch, Key};
use std::time::Instant;

//　カーソルの座標
struct Position {
    y: usize,
    x: usize,
}

struct Game {
    pos: Position,
    field: Field,
}

impl Game {
    // 初期化
    fn new() -> Self {
        Self {
            pos: Position { y: 1, x: 1 },
            field: Game::new_field(),
        }
    }

    fn new_field() -> Field {
        println!("サイズを入力してください");
        let size: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            let size: usize = line.trim().parse().unwrap();
            size + 1
        };

        let mut buf: Field = vec![];

        for y in 0..=size {
            buf.push(vec![]);

            for x in 0..=size {
                let value = if y == 0 || x == 0 || y == size || x == size {
                    WALL
                } else {
                    BACK
                };

                buf[y].push(value)
            }
        }

        return buf;
    }

    fn wall(&self) -> usize {
        self.field.len() - 2
    }
}

const BACK: Option<bool> = Some(false); // 裏
const FRONT: Option<bool> = Some(true); // 表
const WALL: Option<bool> = None; // 壁

// フィールドの型
type Field = Vec<Vec<Option<bool>>>;

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
        for i in 1..check.len() - 1 {
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

    for y in 1..field.len() - 1 {
        for x in 1..field.len() - 1 {
            if pos.y == y && pos.x == x {
                print!("🟥")
            } else if field[y][x] == FRONT {
                print!("⬜️")
            } else if field[y][x] == BACK {
                print!("⬛️")
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

    // 時間測定開始
    let start = Instant::now();

    draw(&game);

    // メインループ
    loop {
        match key.getch() {
            // 左移動
            Ok(Key::Char('a')) | Ok(Key::Left) => {
                if game.pos.x != 1 {
                    game.pos.x -= 1
                }
            }
            // 右移動
            Ok(Key::Char('d')) | Ok(Key::Right) => {
                if game.pos.x != game.wall() {
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
                if game.pos.y != game.wall() {
                    game.pos.y += 1
                };
            }
            // 裏返す
            Ok(Key::Char(' ')) => turn_over(&mut game),
            // 終了
            Ok(Key::Char('q')) => break,
            _ => (),
        }

        draw(&game);

        // 全て表ならクリア！
        if all_front_check(&game) {
            // 時間測定終了
            let end = Instant::now();
            let elapsed = end.duration_since(start);
            let minutes = elapsed.as_secs() / 60;
            let seconds = elapsed.as_secs() % 60;

            println!("クリア！");
            println!("経過時間は{}分{}秒です", minutes, seconds);

            break;
        }
    }

    quit()
}

fn quit() {
    // カーソルを表示する
    println!("\x1b[?25h");
}
