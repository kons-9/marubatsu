pub mod agent;
pub mod environment;

pub use agent::AgentTrait;

pub struct Runner {
    env: environment::Environment,
}

impl Runner {
    pub fn new(
        agent1: Box<dyn AgentTrait>,
        agent2: Box<dyn AgentTrait>,
    ) ->Self {
        Self {
            env: environment::Environment::new(agent1, agent2),
        }
    }

    pub fn run(&mut self) {
        while !self.env.is_done() {
            println!("現在の盤面");
            println!("{}", self.env);
            println!("次は{:?}の番です", self.env.current_agent_type());

            let res = self.env.next();
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
