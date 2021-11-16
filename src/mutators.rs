use rand;

use crate::logic_module::LogicModule;
use crate::lander::Lander;

use crate::{INITIAL_HEIGHT, GRAVITY};

pub fn seed() -> Lander {
    let mut layers = Vec::new();
    layers.push(Vec::new());

    let variables = vec![rand::random(), rand::random(), rand::random()];

    let logic_module = LogicModule {
        layers, variables,
    };

    Lander::new(logic_module, GRAVITY, INITIAL_HEIGHT)
}

fn mutate_attributes(attribute: f32) -> f32 {
    let add = if rand::random() { 5.  } else { -5. };
    attribute + (rand::random::<f32>() * add)
}

pub fn mutate(lander: &Lander) -> Lander {
    let mut variables = lander.logic_module.variables.clone();
    let mut layers = lander.logic_module.layers.clone();

    variables.iter_mut().for_each(|item| {
        *item = mutate_attributes(*item) * lander.mutagen;
    });
    layers.iter_mut().for_each(|layer| {
        layer.iter_mut().for_each(|item| {
            *item = mutate_attributes(*item) * lander.mutagen;
        });
    });

    if rand::random::<f32>() > 0.8 {
        if let Some(last) = layers.iter_mut().last() {
            if last.len() > 5 {
                layers.push(Vec::new());
            } else {
                last.push(rand::random::<f32>());
            }
        }
    }

    let logic_module = LogicModule {
        variables, layers,
    };

    Lander::new(logic_module, GRAVITY, INITIAL_HEIGHT)
}

