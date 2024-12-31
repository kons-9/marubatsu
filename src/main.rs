use marubatsu::Runner;
use marubatsu::agent;

fn main() {
    let agent1 = marubatsu::agent::HumanAgent::new(agent::AgentType::MARU);
    let agent1 = Box::new(agent1);
    let agent2 = marubatsu::agent::RandomAgent::new(agent::AgentType::BATSU);
    let agent2 = Box::new(agent2);
    let mut runner = Runner::new(agent1, agent2);
    runner.run();
}
