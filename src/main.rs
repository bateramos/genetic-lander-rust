mod logic_module;
mod lander;
mod mutators;

use std::thread;
use std::sync::{mpsc, Mutex, Arc};
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

use lander::Lander;
use mutators::{seed, mutate};

const GRAVITY : f32 = 10.;
const INITIAL_HEIGHT : f32 = 10000.;

fn fitness(lander: &Lander) -> f32 {
    if lander.height > lander.initial_height {
        return -10000.
    }
    if !lander.landed && !lander.crashed_landed {
        return -1000.
    }

    let time_fitness = 1./lander.descent_time;
    let speed_fitness = 1./lander.descent_speed;
    let landing_fitness = if lander.height < 0. { lander.height * 10. } else { 1000. };

    if lander.height > lander.initial_height {
        -1000.
    } else {
        time_fitness + landing_fitness + speed_fitness
    }
}

fn run_generation(mut lander: Lander) -> Lander {
    let mut time = 0.;
    while !lander.landed && !lander.crashed_landed && time < 60. {
        lander.tick(GRAVITY, time);
        time += 0.1;
    }

    lander.fitness = fitness(&lander);

    lander
}

const THREAD_COUNT : usize = 4;

fn main() {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let mut lander_biggest_fitness = seed();
    let landers = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..THREAD_COUNT {
        let receiver = Arc::clone(&receiver);
        let landers = Arc::clone(&landers);

        thread::spawn(move || loop {
            if let Ok(lander) = receiver.lock().unwrap().recv() {
                let mut biggest_fit : Lander = lander;
                for _ in 0..100 {
                    let l = biggest_fit.clone();
                    for _ in 0..25 {
                        let lander = run_generation(mutate(&l));

                        if lander.fitness > biggest_fit.fitness {
                            biggest_fit = lander;
                        }
                    }
                }

                landers.lock().unwrap().push(biggest_fit);
            } else {
                break;
            }
        });
    }

    for i in 0..100 {
        for _ in 0..THREAD_COUNT {
            sender.send(lander_biggest_fitness.clone()).unwrap();
        }

        loop {
            thread::sleep(Duration::new(0, 10));

            let should_break = {
                let counter = landers.lock().unwrap();
                counter.len() == THREAD_COUNT
            };

            if should_break {
                let max_mutation = landers.lock().unwrap().iter().max_by_key(|l| l.fitness as i32).unwrap().clone();
                if lander_biggest_fitness.fitness < max_mutation.fitness {
                    lander_biggest_fitness = max_mutation;
                } else {
                    lander_biggest_fitness.mutagen += 0.01;
                }
                landers.lock().unwrap().clear();
                break;
            }
        }

        if i % 10 == 0 {
            println!("[{}] -> {} {:?}", i, lander_biggest_fitness.fitness, lander_biggest_fitness);
        }
    }

    println!("[final] -> {} {:?}", lander_biggest_fitness.fitness, lander_biggest_fitness);

    let mut file = File::create("report.json").unwrap();
    file.write_all(lander_biggest_fitness.descent_to_json().dump()[..].as_bytes()).unwrap();
}
