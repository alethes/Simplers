use crate::point::*;
use ordered_float::OrderedFloat;

/// encapsulate a function and its domain of definition
pub struct TargetFunction
{
   f: fn(&Coordinates) -> f64,
   hypercube: Vec<(f64, f64)>,
   pub dimension: usize
}

impl TargetFunction
{
   /// builds a new function that encapsulate both the function to evaluate and its domain of definition
   pub fn new(f: fn(&Coordinates) -> f64, hypercube: Vec<(f64, f64)>) -> TargetFunction
   {
      let dimension = hypercube.len();
      TargetFunction { f, hypercube, dimension }
   }

   /// converts coordinates from the hypercube to the unit simplex
   /// for the formula used, see: https://math.stackexchange.com/a/385071/495073
   pub fn to_simplex(&self, c: Coordinates) -> Coordinates
   {
      // goes to the unit hypercube
      let c: Coordinates =
         c.into_iter().zip(self.hypercube.iter()).map(|(x, (inf, sup))| (x - inf) / (sup - inf)).collect();
      // goes to the unit simplex
      let sum: f64 = c.iter().sum();
      let max = c.iter()
                 .map(|&c| OrderedFloat(c))
                 .max()
                 .map(|c| *c)
                 .expect("You should have at least one coordinate.");
      let ratio = if sum == 0. { 0. } else { max / sum };
      c.into_iter().map(|x| x * ratio).collect()
   }

   /// converts coordinates from the unit simplex to the hypercube
   /// formula deduced from: https://math.stackexchange.com/a/385071/495073
   pub fn to_hypercube(&self, c: Coordinates) -> Coordinates
   {
      // gets the ratio to go from the unit hypercube to the unit simplex
      let sum: f64 = c.iter().sum();
      let max = c.iter()
                 .map(|&c| OrderedFloat(c))
                 .max()
                 .map(|c| *c)
                 .expect("You should have at least one coordinate.");
      let ratio = if max == 0. { 0. } else { max / sum };
      // goes from the simplex to the target hypercube
      c.into_iter().zip(self.hypercube.iter()).map(|(x, (inf, sup))| inf + x * ratio * (sup - inf)).collect()
   }

   /// takes coordinates in the unit simplex and evaluate them
   pub fn evaluate(&self, c: &Coordinates) -> f64
   {
      let c_hypercube = self.to_hypercube(c.clone());
      let value = (self.f)(&c_hypercube);
      //println!("value:{} in [{}, {}]", value, c_hypercube[0], c_hypercube[1]);
      value
   }
}
