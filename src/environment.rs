use crate::agent::Agent;
use anyhow::Result;

#[derive(Clone, PartialEq, Debug)]
pub struct Environment {
    board: Board,
    curent_agent: Agent,
}

// 3 * 3 の盤面を表す
#[derive(Clone, PartialEq, Debug)]
struct Board {
    cells: [[Cell; 3]; 3],
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Cell {
    agent: Option<Agent>,
    counter: u8,
}

impl Environment {
    pub fn new() -> Self {
        let board = Board::new();
        let curent_agent = Agent::MARU;
        Self {
            board,
            curent_agent,
        }
    }

    pub fn next(&mut self, x: usize, y: usize) -> Result<()> {
        if self.board.cells[y][x].agent.is_some() {
            return Err(anyhow::anyhow!("すでに置かれています"));
        }
        self.board.put(x, y, self.curent_agent);
        self.curent_agent = self.curent_agent.next();
        self.board.update();
        Ok(())
    }

    pub fn is_done(&self) -> bool {
        self.board.winner().is_some()
    }

    pub fn winner(&self) -> Agent {
        self.board.winner().unwrap()
    }
    pub fn current_agent(&self) -> Agent {
        self.curent_agent
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "+---+")?;
        for row in self.board.cells.iter() {
            write!(f, "|")?;
            for cell in row.iter() {
                match cell.agent {
                    Some(Agent::MARU) => write!(f, "○")?,
                    Some(Agent::BATSU) => write!(f, "×")?,
                    None => write!(f, " ")?,
                }
            }
            write!(f, "|")?;
            writeln!(f)?;
        }
        writeln!(f, "+---+")?;
        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[Cell::new(); 3]; 3],
        }
    }

    fn put(&mut self, x: usize, y: usize, agent: Agent) {
        self.cells[y][x].agent = Some(agent);
    }

    fn update(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                cell.update();
            }
        }
    }

    fn winner(&self) -> Option<Agent> {
        // check maru batsu
        // check horizontal
        for row in self.cells.iter() {
            if row.iter().all(|cell| cell.agent == Some(Agent::MARU)) {
                return Some(Agent::MARU);
            }
            if row.iter().all(|cell| cell.agent == Some(Agent::BATSU)) {
                return Some(Agent::BATSU);
            }
        }

        // check vertical
        for x in 0..3 {
            if (0..3).all(|y| self.cells[y][x].agent == Some(Agent::MARU)) {
                return Some(Agent::MARU);
            }
            if (0..3).all(|y| self.cells[y][x].agent == Some(Agent::BATSU)) {
                return Some(Agent::BATSU);
            }
        }

        // check diagonal
        let diagonal = [[(0, 0), (1, 1), (2, 2)], [(0, 2), (1, 1), (2, 0)]];
        for diagonal in diagonal.iter() {
            if diagonal.iter().all(|(x, y)| self.cells[*y][*x].agent == Some(Agent::MARU)) {
                return Some(Agent::MARU);
            }
            if diagonal.iter().all(|(x, y)| self.cells[*y][*x].agent == Some(Agent::BATSU)) {
                return Some(Agent::BATSU);
            }
        }
        None
    }
}

impl Cell {
    const MAX_COUNTER: u8 = 7;

    fn new() -> Self {
        Self {
            agent: None,
            counter: 0,
        }
    }
    fn update(&mut self) {
        match self.agent {
            Some(_) => self.counter += 1,
            None => self.counter = 0,
        }
        if self.counter == Self::MAX_COUNTER {
            self.agent = None;
            self.counter = 0;
        }
    }
}
