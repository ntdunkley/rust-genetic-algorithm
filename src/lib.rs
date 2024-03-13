use rand::distributions::{Distribution, Uniform};
use rand::Rng;

const POPULATION_SIZE: usize = 100;
const TOURNAMENT_SIZE: usize = 5;
const MUTATION_RATE: f32 = 0.01;

#[derive(Debug)]
pub struct Individual {
    data: String,
    fitness: u16
}

impl Individual {
    fn new(len: usize, target: &str) -> Self {
        let mut data = String::with_capacity(len);

        let mut rng = rand::thread_rng();
        let zero_or_one = Uniform::from(0..=1);
        for _ in 0..len {
            let throw = zero_or_one.sample(&mut rng);
            data.push_str(&throw.to_string());
        }
        let fitness = Self::calculate_fitness(&data, target);
        Self {
            data,
            fitness
        }
    }

    fn with_data(data: String, target: &str) -> Self {
        let fitness = Self::calculate_fitness(data.as_str(), target);
        Self {
            data,
            fitness
        }
    }

    fn calculate_fitness(data: &str, target: &str) -> u16 {
        let mut fitness = 0u16;
        for i in 0..data.len() {
            if data.as_bytes()[i] == target.as_bytes()[i] {
                fitness += 1;
            }
        }
        fitness
    }

    pub fn get_fitness(&self) -> u16 {
        self.fitness
    }
}

pub struct GA {
    pub target: String,
    pub population: Vec<Individual>
}

impl GA {
    pub fn new(target: &str) -> Self {
        let mut population = Vec::with_capacity(POPULATION_SIZE);
        for _ in 0..POPULATION_SIZE {
            population.push(Individual::new(target.len(), target))
        }
        Self {
            target: target.to_string(),
            population
        }
    }

    pub fn evolve(&mut self) {
        let mut new_population = Vec::with_capacity(POPULATION_SIZE);

        for _ in 0..POPULATION_SIZE {
            let indv_1 = self.selection();
            //println!("Winner is : {:?}", indv_1);
            let indv_2 = self.selection();
            //println!("Winner is : {:?}", indv_2);
            let mut new_indv = self.crossover(indv_1, indv_2);
            //println!("Offspring is : {:?}", new_indv);
            Self::mutate(&mut new_indv, self.target.as_str());
            new_population.push(new_indv);
        }

        self.population = new_population;
    }

    pub fn get_fittest(&self) -> &Individual {
        self.population.iter().max_by_key(|indv| indv.fitness).unwrap()
    }

    fn selection(&self) -> &Individual {
        //println!("\nTOURNAMENT:");
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..POPULATION_SIZE);

        let mut best_indv = &self.population[die.sample(&mut rng)];
        //println!("Entry: {:?}", best_indv);
        for _ in 1..TOURNAMENT_SIZE {
            let curr_indv = &self.population[die.sample(&mut rng)];
            //println!("Entry: {:?}", curr_indv);
            if curr_indv.fitness > best_indv.fitness {
                best_indv = curr_indv;
            }
        }
        best_indv
    }

    fn crossover(&self, indv_1: &Individual, indv_2: &Individual) -> Individual {
        let mut rng = rand::thread_rng();
        let coin_flip = Uniform::from(0..=1);
        let mut new_indv_data = String::new();
        for i in 0..indv_1.data.len() {
            let choice = coin_flip.sample(&mut rng);
            if choice == 0 {
                new_indv_data.push(indv_1.data.as_bytes()[i] as char);
            } else {
                new_indv_data.push(indv_2.data.as_bytes()[i] as char);
            }
        }
        Individual::with_data(new_indv_data, self.target.as_str())
    }

    fn mutate(indv: &mut Individual, target: &str) {
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(0.0..=1.0);
        //println!("{}", rand);
        if rand <= MUTATION_RATE {
            let rand_index = rng.gen_range(0..indv.data.len());
            let gene: char = indv.data.as_bytes()[rand_index] as char;
            if gene == '0' {
                indv.data.replace_range(rand_index..rand_index+1, "1");
            } else {
                indv.data.replace_range(rand_index..rand_index+1, "0");
            }
            if target.as_bytes()[rand_index] as char == indv.data.as_bytes()[rand_index] as char {
                indv.fitness += 1;
            }
        }
    }
}