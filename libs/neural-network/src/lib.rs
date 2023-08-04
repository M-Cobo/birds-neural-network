use rand::Rng;

pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons : usize,
}

impl Network {
    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(rng, layers[0].neurons, layers[1].neurons)
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
    pub fn random(rng: &mut dyn rand::RngCore, input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
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
    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs.iter().zip(&self.weights).map(|(input, weight)| input * weight).sum::<f32>();

        (self.bias + output).max(0.0)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    mod random {
        use super::*;
        
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;
        
        #[test]
        fn test_neuron() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(neuron.weights.as_slice(), &[0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref());
        }

        #[test]
        fn test_layer() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 4, 2);

            assert_eq!(layer.neurons.len(), 2);
        }

        #[test]
        fn test_network() {
            let layers = vec![LayerTopology {neurons: 4}, LayerTopology {neurons: 2}, LayerTopology {neurons: 1}];
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(&mut rng, &layers);

            assert_eq!(network.layers[0].neurons.len(), 2);
            assert_eq!(network.layers[1].neurons.len(), 1);
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test_neuron() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

            approx::assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

            approx::assert_relative_eq!(neuron.propagate(&[0.5, 1.0]), (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,);
        }

        #[test]
        fn test_layer() {

            let neuron_1 = Neuron {
                bias: 0.5,
                weights: vec![0.5, 1.0],
            };

            let neuron_2 = Neuron {
                bias: 0.7,
                weights: vec![-10.0, -10.0],
            };

            let layer = Layer { 
                neurons: vec![neuron_1, neuron_2],
            };

            assert_eq!(layer.propagate(vec![-0.3, 0.8]), [(-0.3 * 0.5) + (0.8 * 1.0) + 0.5, 0.0],);
        }

        #[test]
        fn test_network() {

            let neuron_1 = Neuron {
                bias: -0.5,
                weights: vec![0.5, 1.0],
            };

            let neuron_2 = Neuron {
                bias: 0.7,
                weights: vec![-10.0, -10.0],
            };

            let neuron_3 = Neuron {
                bias: 0.3,
                weights: vec![0.2, 0.8],
            };

            let layer_1 = Layer { 
                neurons: vec![neuron_1, neuron_2],
            };

            let layer_2 = Layer { 
                neurons: vec![neuron_3],
            };

            let network = Network {
                layers: vec![layer_1, layer_2],
            };

            assert_eq!(network.propagate(vec![-1.0, 1.0]).as_slice(), [0.86]);
        }
    }
}
