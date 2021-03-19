fn simulation_run(
    initial_state: Variables,
parameters: Parameters,
timestep_block: TimestepBlock,
timesteps: i32,
samples: i32) -> Results{

    
    
    let mut results: Results = Vec::new();
    for sample in 1..samples{
        let mut history: History = Vec::new();
        let mut state: Variables = initial_state;
        for timestep in 1..timesteps{
            let mut substep_history: SubstepHistory = Vec::new();
            for (substep, substep_block) in timestep_block.iter().enumerate(){
                let mut new_state: Variables;
                new_state = state;
                for variable_fn in substep_block {
                    new_state = variable_fn(parameters, substep, history, state);
                }
                state = new_state;
                substep_history.push(state);
            }
            
        }
    }
    results
}

#[derive(Copy, Clone)]
struct Variables {
    prey_population: f32,
    predator_population: f32
}

struct Parameters {
    prey_growth: f32,
    predator_growth: f32
}

type VariableUpdate = fn(Parameters, usize, History, Variables);
type SubstepBlock = Vec<VariableUpdate>;
type TimestepBlock = Vec<SubstepBlock>;

type SubstepHistory = Vec<Variables>;
type History = Vec<SubstepHistory>;
type Results = Vec<History>;

fn s_update_prey(params: Parameters,
                 substep: usize,
                 history: History,
                 state: Variables){
    state.prey_population = state.prey_population + params.prey_growth;
}



fn main() {

    let initial_state = Variables{
        prey_population: 10.0,
        predator_population: 5.0
    };

    let parameters = Parameters{
        prey_growth: 1.0,
        predator_growth: 1.0
    };


    let mut substep_block_1: Vec<VariableUpdate> = Vec::new();
    substep_block_1.push(s_update_prey);

    let mut timestep_block: TimestepBlock = Vec::new();
    timestep_block.push(substep_block_1);

    let timesteps: i32 = 10;
    let samples: i32 = 2;

    simulation_run(initial_state, parameters, timestep_block, timesteps, samples);

    println!("Hello, world!");
}