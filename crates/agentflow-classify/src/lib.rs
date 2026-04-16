pub struct AgentHeuristics;

impl AgentHeuristics {
    pub fn new() -> Self {
        Self
    }

    pub fn classify_by_compute(&self, cu_consumed: u64) -> bool {
        cu_consumed > 50_000
    }

    pub fn calculate_machine_value(&self, cu_consumed: u64) -> u64 {
        cu_consumed * 10
    }
}

