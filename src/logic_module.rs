#[derive(Debug, Clone)]
pub struct LogicModule {
    pub variables: Vec<f32>,
    pub layers: Vec<Vec<f32>>,
}

impl LogicModule {
    pub fn evaluate(&self, args: [f32; 3]) -> f32 {
        let mut index = 0;
        let mut results : Vec<f32> = args.iter().map(|item| {
            let result = item * self.variables[index];
            index += 1;

            result
        }).collect();

        self.layers.iter().for_each(|layer| {
            let mut new_result : Vec<f32> = std::iter::repeat(0.).take(layer.len()).collect();

            let mut index = 0;
            layer.iter().for_each(|item| {
                results.iter().for_each(|weight| {
                    new_result[index] = item * weight;
                });
                index += 1;
            });

            results = if new_result.is_empty() {
                results.clone()
            } else {
                new_result
            };
        });

        results.iter().fold(0., |acc, item| {
            acc + item
        })
    }
}
