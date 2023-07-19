pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons : usize,
}

impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|neuron| neuron.propagate(&inputs)).collect()
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    pub fn random(output_size: usize) -> Self {
        let bias = todo!();

        let weights = (0..output_size)
            .map(|_| todo!())
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs.iter().zip(&self.weights).map(|(input, weight)| input * weight).sum::<f32>();

        (self.bias + output).max(0.0)
    }
}