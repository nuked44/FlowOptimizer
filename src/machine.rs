use core::fmt;

use crate::machine;

pub struct Manager {
    machines: Vec<Machine>,
    count: usize,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            machines: vec![Machine {
                id: 0,
                name: "Empty Machine".to_string(),
                throughput_per_tu: 0f64,
            }],
            count: 0,
        }
    }

    pub fn new_machine(&mut self, id: usize, name: String, throughput_per_tu: f64) -> usize {
        let machine = Machine::new(id, name, throughput_per_tu);
        self.machines.push(machine);
        self.count += 1;

        id
    }

    pub fn find_by_id(&self, id: usize) -> &Machine {
        match self.machines.iter().find(|&m| m.id == id) {
            Some(machine) => machine,
            None => &self.machines[0],
        }
    }
}

impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "There are {} machines", self.count)?;
        for machine in &self.machines {
            writeln!(f, "{machine}")?;
        }
        write!(f, "")
    }
}


pub struct Machine {
    pub id: usize,
    pub name: String,
    pub throughput_per_tu: f64,
}

impl Machine {
    fn new(id: usize, name: String, throughput_per_tu: f64) -> Machine {
        Machine {
            id,
            name,
            throughput_per_tu,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: ({})", self.name, self.id)?;
        writeln!(f, "Throughput/tu: {}", self.throughput_per_tu)
    }
}
