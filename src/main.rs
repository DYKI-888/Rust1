use getch_rs::{Getch, Key};
use std::time::Instant;

const BACK: Option<bool> = Some(false); // 裏
const FRONT: Option<bool> = Some(true); // 表
const WALL: Option<bool> = None; // 壁

// フィールドの型
type Field = Vec<Vec<Option<bool>>>;

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

        buf
    }

    // 選択している位置を中心にマスを十字に裏返す
    fn turn_over(&mut self) {
        let y = self.pos.y;
        let x = self.pos.x;

        let list: [(usize, usize); 5] = [(y, x), (y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

        for i in list {
            if let Some(value) = self.field[i.0][i.1] {
                self.field[i.0][i.1] = Some(!value);
            }
        }
    }

    // 描画
    fn draw(&self) {
        // 画面クリア
        println!("\x1b[2J\x1b[H");

        for y in 1..self.field.len() - 1 {
            for x in 1..self.field.len() - 1 {
                if self.pos.y == y && self.pos.x == x {
                    print!("🟥")
                } else if self.field[y][x] == FRONT {
                    print!("⬜️")
                } else if self.field[y][x] == BACK {
                    print!("⬛️")
                }
            }
            println!()
        }
    }

    // 全てのマスが表がならtrue、そうでないならfalse
    fn all_front_check(&self) -> bool {
        for check in self.field.iter() {
            for &item in check.iter().skip(1).take(check.len() - 1) {
                if item == Some(false) {
                    return false;
                }
            }
        }
        true
    }

    fn wall(&self) -> usize {
        self.field.len() - 2
    }
}

fn main() {
    let mut game = Game::new();
    let key = Getch::new();

    // カーソルの削除
    print!("\x1b[?25l");

    // 時間測定開始
    let start = Instant::now();

    game.draw();

    // メインループ
    loop {
        match key.getch() {
            // 左移動
            Ok(Key::Char('h')) | Ok(Key::Left) => {
                if game.pos.x != 1 {
                    game.pos.x -= 1
                }
            }
            // 右移動
            Ok(Key::Char('l')) | Ok(Key::Right) => {
                if game.pos.x != game.wall() {
                    game.pos.x += 1
                }
            }
            // 上移動
            Ok(Key::Char('k')) | Ok(Key::Up) => {
                if game.pos.y != 1 {
                    game.pos.y -= 1
                }
            }
            // 下移動
            Ok(Key::Char('j')) | Ok(Key::Down) => {
                if game.pos.y != game.wall() {
                    game.pos.y += 1
                }
            }
            // 裏返す
            Ok(Key::Char(' ')) => game.turn_over(),
            // 終了
            Ok(Key::Char('q')) => break,
            _ => (),
        }

        game.draw();

        // 全て表ならクリア！
        if game.all_front_check() {
            // 時間測定終了
            let end = Instant::now();
            let elapsed = end.duration_since(start);
            let minutes = elapsed.as_secs() / 60;
            let seconds = elapsed.as_secs() % 60;

            println!("クリア！");
            println!("経過時間{}分{}秒", minutes, seconds);

            break;
        }
    }

    quit()
}

fn quit() {
    // カーソルを表示する
    println!("\x1b[?25h");
}
