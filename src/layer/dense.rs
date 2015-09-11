use af;
use af::{Dim4, Array, MatProp};

use utils;
use activations;
use initializations;
use layer::Layer;

pub struct Dense {
  weights: Vec<Array>,
  bias: Vec<Array>,
  delta: (Array, Array),
  inputs: Array,
  activation: &'static str,
}

impl Layer for Dense {
  fn new(input_size: u64, output_size: u64
         , output_activation: &'static str
         , w_init: &'static str, b_init: &str) -> Dense
  {
    Dense {
      weights: vec![initializations::get_initialization(w_init, Dim4::new(&[output_size, input_size, 1, 1])).unwrap()],  // W
      bias: vec![initializations::get_initialization(b_init, Dim4::new(&[output_size, 1, 1, 1])).unwrap()],              // b
      inputs: initializations::get_initialization("zeros", Dim4::new(&[input_size, 1, 1, 1])).unwrap(),                  // a_{l-1}
      delta: (initializations::get_initialization("zeros", Dim4::new(&[output_size, input_size, 1, 1])).unwrap()         // delW
              , initializations::get_initialization("zeros", Dim4::new(&[output_size, 1, 1, 1])).unwrap()),              // delb
      activation: output_activation,
    }
  }

  fn forward(&mut self, activation: &Array) -> Array {
    // append previous_activation
    self.inputs = activation.clone();
    
    //sigma(Wx + b)
    activations::get_activation(self.activation, &af::add(&af::matmul(&self.weights[0]
                                                                      , &activation
                                                                      , MatProp::NONE
                                                                      , MatProp::NONE).unwrap()
                                                          , &self.bias[0]).unwrap()).unwrap()
  }

  fn backward(&self, upper_diffs: &Array, gradients: &Array) -> Array {
    // d_l = (transpose(W) * d_{l+1}) .* dActivation(z) where z = activation w/out non-linearity
    let inner = af::matmul(&self.weights[0]
                           , upper_diffs
                           , MatProp::CTRANS
                           , MatProp::NONE).unwrap();
    af::mul(&inner, gradients).unwrap()
  }

  fn update(&mut self, delta: (Array, Array), train: bool) {
    self.delta.0 = af::add(&self.delta.0, &delta.0).unwrap();
    self.delta.1 = af::add(&self.delta.1, &delta.1).unwrap();
  }

  fn get_delta(&self) -> (Array, Array) {
    (self.delta.0.clone(), self.delta.1.clone())
  }

  fn get_weights(&self) -> Vec<Array> {
    self.weights.clone()
  }

  fn set_weights(&mut self, weights: &Array, index: usize) {
    self.weights[index] = weights.clone();
  }

  fn get_bias(&self) -> Vec<Array> {
    self.bias.clone()
  }

  fn set_bias(&mut self, bias: &Array, index: usize) {
    self.bias[index] = bias.clone();
  }

  fn get_bias_dims(&self) -> Vec<Dim4> {
    let mut dims = Vec::new();
    for b in &self.bias {
      dims.push(b.dims().unwrap().clone())
    }
    dims
  }

  fn get_weight_dims(&self) -> Vec<Dim4> {
    let mut dims = Vec::new();
    for w in &self.weights {
      dims.push(w.dims().unwrap().clone())
    }
    dims
  }

  fn get_input(&self) -> Array {
    self.inputs.clone()
  }

  fn output_size(&self) -> u64 {
    let weight_dims = self.get_weight_dims();
    weight_dims[weight_dims.len() - 1][1]    
  }

  fn input_size(&self) -> u64 {
    let weight_dims = self.get_weight_dims();
    weight_dims[0][0]
  }

  fn get_activation_type(&self) -> &'static str {
    &self.activation
  }
}
