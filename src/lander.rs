use json;

use crate::logic_module::LogicModule;
use crate::mutators::seed;
use crate::GRAVITY;

#[derive(Debug, Clone)]
pub struct Lander {
    pub logic_module: LogicModule,
    pub initial_height: f32,
    pub gravity: f32,
    pub height: f32,
    pub descent_speed: f32,
    pub descent_time: f32,
    pub fitness: f32,
    pub thruster_speed: f32,
    pub landed: bool,
    pub crashed_landed: bool,
    pub mutagen: f32,
}

impl Lander {
    pub fn new(logic_module: LogicModule, descent_speed: f32, height: f32) -> Lander {
        Lander {
            logic_module,
            height,
            gravity: descent_speed,
            initial_height: height,
            descent_speed: 0.,
            thruster_speed: 0.,
            landed: false,
            crashed_landed: false,
            descent_time: 0.,
            fitness: f32::MIN,
            mutagen: 1.,
        }
    }

    pub fn tick(&mut self, gravity: f32, descent_time: f32) {
        if self.landed || self.crashed_landed {
            return
        }

        let tick_time = 0.1;

        self.descent_speed += gravity * tick_time;
        let thruster_speed = self.vertical_thruster([self.height/self.initial_height, self.descent_speed/100., descent_time/100.]);

        self.thruster_speed = thruster_speed;
        self.descent_speed -= thruster_speed * tick_time;

        self.height -= self.descent_speed;

        if self.height < 0. {
            self.crashed_landed = true;
        } else if self.height < 2. && self.descent_speed < 2. {
            self.landed = true;
        }

        self.descent_time = descent_time;
    }

    pub fn vertical_thruster(&self, args: [f32;3]) -> f32 {
        let mut thruster = self.logic_module.evaluate(args);
        if thruster > 50. {
            thruster = 50.
        } else if thruster < 0. {
            thruster = 0.
        }

        thruster
    }

    pub fn descent_to_json(&mut self) -> json::JsonValue {
        let mut lander = seed();
        lander.logic_module = self.logic_module.clone();

        let mut report = json::JsonValue::new_array();
        let mut time = 0.;

        while !lander.landed && !lander.crashed_landed && time < 60. {
            lander.tick(GRAVITY, time);
            time += 0.1;
            report.push(json::object!{
                "height": lander.height,
                "speed": lander.descent_speed,
                "time": lander.descent_time,
            }).unwrap();
        }

        report
    }
}


