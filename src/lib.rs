pub mod agent;
pub mod environment;

use proconio::input;

pub struct Runner {
    env: environment::Environment,
}

impl Runner {
    pub fn new() -> Self {
        Self {
            env: environment::Environment::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.env.is_done() {
            println!("現在の盤面");
            println!("{}", self.env);
            println!("次は{:?}の番です", self.env.current_agent());
            println!("x yを入力してください");
            input! {
                x: usize,
                y: usize,
            }
            if x >= 3 || y >= 3 {
                println!("xとyは0から2の間で指定してください");
                continue;
            }

            let res = self.env.next(x, y);
            if let Err(e) = res {
                println!("{}", e);
                continue;
            }
        }

        println!("終了");
        println!("{}", self.env);
        let winner = self.env.winner();
        println!("{:?}の勝ち", winner);
    }
}
