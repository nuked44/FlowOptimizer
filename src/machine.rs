pub struct MachineManager {
    machines: Vec<Option<Machine>>,
    count: usize,
}

impl MachineManager {
    pub fn new() -> MachineManager {
        MachineManager {
            machines: Vec::new(),
            count: 0,
        }
    }

    pub fn new_machine(
        &mut self,
        id: usize,
        name: String,
        throughput_per_tu: f64,
    ) -> usize {
        let machine = Machine::new(id, name, throughput_per_tu);
        self.machines[id] = Some(machine);
        self.count += 1;
        
        id
    }

    pub fn find_by_id(self, id: usize) -> Option<&'static Machine> {
        if id > self.count {
            return None;
        }
        match self.machines[id] {
            Some(machine) => Some(&machine),
            None => None,
        }
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
