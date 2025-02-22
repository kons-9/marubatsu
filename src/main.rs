#[allow(unused_imports)]
use marubatsu::agent::{ai::AIAgent, human::HumanAgent, random::RandomAgent, AgentType};

use marubatsu::Runner;

fn main() {
    let agent1 = HumanAgent::new(AgentType::MARU);
    let agent1 = Box::new(agent1);
    let agent2 = HumanAgent::new(AgentType::BATSU);
    let agent2 = Box::new(agent2);

    let mut runner = Runner::new(agent1, agent2);
    runner.run();
}
