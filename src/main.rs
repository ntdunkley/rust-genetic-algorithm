use genetic_algorithm::GA;

fn main() {
    let target = "1000110100110101111010111111000010111111000100110100110111010101";

    let mut thread_handles = Vec::new();
    for _ in 0..10 {
        let thread_handle = std::thread::spawn(|| {
            let mut ga = GA::new(target);
            let mut solution_found = false;
            let mut evolutions = 0;

            while evolutions < 10000 {
                ga.evolve();
                if ga.get_fittest().get_fitness() == target.len() as u16 {
                    solution_found = true;
                    break;
                }
                evolutions += 1;
            }
            println!("Fittest was {:?}", ga.get_fittest());
            if solution_found {
                println!("Thread {:?}: Solution found in {} evolutions", std::thread::current().id(), evolutions);
            } else {
                println!("Thread {:?}: Solution not found", std::thread::current().id());
            }
        });
        thread_handles.push(thread_handle);
    }
    for thread in thread_handles {
        thread.join().expect("Could not join thread");
    }
}