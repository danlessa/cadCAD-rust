fn simulation_run(
    initial_state: Variables,
parameters: Parameters,
timestep_block: TimestepBlock,
timesteps: i32,
samples: i32) -> Results {
    
    let mut results: Results = Vec::new();
    for sample in 0..samples{
        let mut history: History = Vec::new();
        let mut state: Variables = initial_state;
        for timestep in 0..timesteps{
            let mut substep_history: SubstepHistory = Vec::new();
            for (substep, substep_block) in timestep_block.iter().enumerate(){
                for variable_fn in substep_block {
                    state = variable_fn(&parameters, substep, &history, state);
                }
                substep_history.push(state);
            }
            history.push(substep_history)
        }
        results.push(history)
    }
    results
}

#[derive(Copy, Clone, Debug)]
struct Variables {
    prey_population: f32,
    predator_population: f32
}

#[derive(Debug)]
struct Parameters {
    prey_growth: f32,
    predator_growth: f32
}

type VariableUpdate = fn(&Parameters, usize, &History, Variables) -> Variables;
type SubstepBlock = Vec<VariableUpdate>;
type TimestepBlock = Vec<SubstepBlock>;

type SubstepHistory = Vec<Variables>;
type History = Vec<SubstepHistory>;

type Results = Vec<History>;

fn s_update_prey(params: &Parameters,
                 substep: usize,
                 history: &History,
                 mut state: Variables) -> Variables {
    state.prey_population = state.prey_population + params.prey_growth;
    state
}

fn s_update_predator(params: &Parameters,
    substep: usize,
    history: &History,
    mut state: Variables) -> Variables {
state.predator_population = state.predator_population + params.predator_growth;
state
}


fn main() {

    let initial_state = Variables{
        prey_population: 10.0,
        predator_population: 5.0
    };

    let parameters = Parameters{
        prey_growth: 1.0,
        predator_growth: 0.5
    };


    let mut substep_block_1: Vec<VariableUpdate> = Vec::new();
    substep_block_1.push(s_update_prey);
    substep_block_1.push(s_update_predator);

    let mut substep_block_2: Vec<VariableUpdate> = Vec::new();
    substep_block_2.push(s_update_prey);
    substep_block_2.push(s_update_prey);

    let mut timestep_block: TimestepBlock = Vec::new();
    timestep_block.push(substep_block_1);
    timestep_block.push(substep_block_2);

    let timesteps: i32 = 10;
    let samples: i32 = 2;

    let results: Results = simulation_run(initial_state, parameters, timestep_block, timesteps, samples);

    println!("------------");
    for (n, sample_data) in results.iter().enumerate() {
        for (t, timestep_data) in sample_data.iter().enumerate()  {
            for (s, substep_data) in timestep_data.iter().enumerate()  {
                println!("(S, T, s) = ({}, {}, {}) {:?}", n, t, s, substep_data);
            }
        }
    }
    println!("------------");
}