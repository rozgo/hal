pub use self::sgd::SGD;
mod sgd;

pub use self::adam::Adam;
mod adam;

use af;
use af::{Array, Dim4, NormType};
use std::collections::HashMap;

use utils;
use error::HALError;
use params::ParamManager;

pub trait Optimizer {
  fn new(params: &HashMap<&str, &str>) -> Self where Self: Sized;
  //fn setup(&mut self, w_dim: Vec<Dim4>, b_dim: Vec<Dim4>);
  fn setup(&mut self, dims: Vec<Dim4>);
  fn update(&mut self, parameter_manager: &mut ParamManager, batch_size: u64);
  fn info(&self);
}

pub fn get_optimizer(name: &str, params: &HashMap<&str, &str>) -> Result<Box<Optimizer>, HALError>{
  match name.to_lowercase().as_str() {
    "sgd"  => Ok(Box::new(SGD::new(params))),
    "adam" => Ok(Box::new(Adam::new(params))),
    _     => Err(HALError::UNKNOWN),
  }
}

pub fn get_optimizer_with_defaults(name: &str) -> Result<Box<Optimizer>, HALError>{
  match name.to_lowercase().as_str() {
    "sgd" =>  Ok(Box::new(SGD::default())),
    "adam" => Ok(Box::new(Adam::default())),
    _     => Err(HALError::UNKNOWN),
  }
}

pub fn clip_grads(input: &Array, rescale: f32) -> Array {
  let norm = af::norm(input, NormType::VECTOR_2, 0f64, 0f64) as f32;
  let scale = rescale / norm.max(rescale);
  utils::cast(&af::mul(input, &scale, false), input.get_type())
  //utils::clip_by_value(input, -5.0f32, 5.0f32)
}
