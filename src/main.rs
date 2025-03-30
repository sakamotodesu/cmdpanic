use crossterm::{
    cursor::{Hide, Show, MoveTo},
    event::{self, Event, KeyCode},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

struct Game {
    score: u32,
    current_hole: u8,
    round_start_time: Instant,
    round_time_limit: Duration,
    game_start_time: Instant,
    game_time_limit: Duration,
    is_running: bool,
}

impl Game {
    fn new() -> Self {
        Self {
            score: 0,
            current_hole: rand::thread_rng().gen_range(1..=5),
            round_start_time: Instant::now(),
            round_time_limit: Duration::from_secs(2),
            game_start_time: Instant::now(),
            game_time_limit: Duration::from_secs(10),
            is_running: true,
        }
    }

    fn draw(&self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;

        // 穴の表示
        for i in 1..=5 {
            if i == self.current_hole {
                write!(stdout, "[🐊]")?;
            } else {
                write!(stdout, "[ ]")?;
            }
            if i < 5 {
                write!(stdout, " ")?;
            }
        }
        write!(stdout, "\n")?;

        // スコアと残り時間の表示
        stdout.queue(MoveTo(0, 1))?;
        let round_elapsed = self.round_start_time.elapsed();
        let round_remaining = self.round_time_limit.as_secs_f32() - round_elapsed.as_secs_f32();
        let game_elapsed = self.game_start_time.elapsed();
        let game_remaining = self.game_time_limit.as_secs_f32() - game_elapsed.as_secs_f32();
        write!(
            stdout,
            "Score: {}  Round Time: {:.1}s  Total Time: {:.1}s\n",
            self.score, round_remaining, game_remaining
        )?;

        stdout.flush()?;
        Ok(())
    }

    fn update(&mut self) -> bool {
        // ラウンドの時間切れチェック
        if self.round_start_time.elapsed() >= self.round_time_limit {
            println!("Time's up!");
            self.next_round();
            return self.is_running;
        }

        // ゲーム全体の時間切れチェック
        if self.game_start_time.elapsed() >= self.game_time_limit {
            println!("Game Over! Time limit reached.");
            self.is_running = false;
            return false;
        }

        self.is_running
    }

    fn check_input(&mut self, input: u8) -> bool {
        if input == self.current_hole {
            self.score += 1;
            true
        } else {
            false
        }
    }

    fn next_round(&mut self) {
        self.current_hole = rand::thread_rng().gen_range(1..=5);
        self.round_start_time = Instant::now();
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}

fn main() -> std::io::Result<()> {
    // スタート画面の表示（標準出力を使用）
    println!("Command Line Whack-a-Mole!");
    println!("Enter the hole number (1-5) to catch the mole!");
    println!("Each round is 2 seconds, total time limit is 10 seconds.");
    println!("Press ESC to exit.");
    println!("Press Enter to start...");
    
    // Enterキーを待つ
    std::io::stdin().read_line(&mut String::new()).unwrap();

    // crosstermの制御を開始
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.queue(Hide)?;

    let mut game = Game::new();
    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(50);

    while game.is_running {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        game.quit();
                        break;
                    }
                    KeyCode::Char(c) => {
                        if let Some(num) = c.to_digit(10) {
                            if num >= 1 && num <= 5 {
                                if game.check_input(num as u8) {
                                    println!("Correct!");
                                    game.next_round();
                                } else {
                                    println!("Wrong!");
                                    game.next_round();
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_update.elapsed() >= update_interval {
            game.draw(&mut stdout)?;
            last_update = Instant::now();
        }

        if !game.update() {
            break;
        }
    }

    terminal::disable_raw_mode()?;
    stdout.queue(Show)?;
    stdout.flush()?;
    println!("\nGame Over! Final Score: {}", game.score);
    Ok(())
}
