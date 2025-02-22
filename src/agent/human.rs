use super::AgentTrait;
use super::AgentType;
use super::Command;
use crate::environment::Board;
use anyhow::Result;

pub struct HumanAgent {
    agent_type: AgentType,
}

impl HumanAgent {
    pub fn new(agent_type: AgentType) -> Self {
        Self { agent_type }
    }

    fn try_parse_x_y<'a, T>(&self, mut iter: T) -> Result<Command>
    where
        T: Iterator<Item = &'a str>,
    {
        let x = match iter.next() {
            Some(x) => x
                .parse()
                .or_else(|_| Err(anyhow::anyhow!("xは数字で指定してください")))?,
            None => {
                return Err(anyhow::anyhow!("xを指定してください"));
            }
        };
        let y = match iter.next() {
            Some(y) => y.parse()?,
            None => {
                return Err(anyhow::anyhow!("yを指定してください"));
            }
        };
        if iter.next().is_some() {
            return Err(anyhow::anyhow!("引数が多すぎます"));
        }
        if self.validate_x_y(x, y) {
            return Ok(Command::Next(x, y));
        } else {
            return Err(anyhow::anyhow!("x yは0から2の間で指定してください"));
        }
    }
    fn parse_command(&self, command: &str) -> Result<Command> {
        let mut iter = command.split_whitespace();
        match iter.next() {
            Some("next") => self.try_parse_x_y(iter),
            Some("prev") => Ok(Command::Prev),
            Some("exit") => Ok(Command::Exit),
            Some("help") => Ok(Command::Help),
            _ => self.try_parse_x_y(command.split_whitespace()),
        }
    }
    fn validate_x_y(&self, x: usize, y: usize) -> bool {
        x < 3 && y < 3
    }
}

impl AgentTrait for HumanAgent {
    fn agent_type(&self) -> AgentType {
        self.agent_type
    }

    fn next(&self, _: &Board) -> Command {
        loop {
            println!("x yを入力してください");
            // get input
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            let command = buf.trim().to_string();
            match self.parse_command(&command) {
                Ok(command) => return command,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
        }
    }
}
