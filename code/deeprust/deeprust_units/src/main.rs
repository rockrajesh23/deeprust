// Task :- Performing gradient descent operations
// Author :- @digneshwer
// Version :- 0.0.1
// Date : - 28 Jan 2017

// Modules used
use std::{f64,i32};

// Main function point of execution
fn main() {
    println!("Gradient Descent Experiment");

    let mut parameter_test = Params {
    	sgd_alpha : 0.1,sample_size : 2,theta : (1.0, 1.0),data : (1.0,1.0),label: (1.0,1.0), update_int :(0.0,0.0)
    };

    println!("Updated theta_zero : {:?}",parameter_test.cal_update().theta_zero_update());
    println!("Updated theta_one : {:?}",parameter_test.cal_update().theta_one_update());
}

// Defining an user defined data type params
// check the ../../sgd_working.md file variable documentation
#[derive(Debug)]
struct Params {
	sgd_alpha : f64,
	sample_size : i32,
	theta : (f64, f64),
	data : (f64,f64),
	label : (f64,f64),
	update_int : (f64,f64),
}

// Parameter optimization functionality for the param datatype
trait ParameterOptimization {
	// fn theta_zero_update(&self) -> (f64,f64,i32,f64);
	// fn theta_one_update(&self) -> (f64,f64,i32,f64);
	fn theta_zero_update(&mut self) -> &Params;
	fn theta_one_update(&mut self) -> &Params;
	fn cal_update(&mut self) -> &Params;
}

// implement area for circle
impl ParameterOptimization for Params {
	fn theta_zero_update(&mut self) -> &Params {
		self.theta.0 = self.theta.0 - self.update_int.0;
		// (self.theta.0, self.sgd_alpha, self.sample_size, self.data.0)
		self
	}

	fn theta_one_update(&mut self) -> &Params {
		self.theta.1 = self.theta.1 - self.update_int.1;
		// (self.theta.1, self.sgd_alpha, self.sample_size, self.data.1)
		self
	}

	fn cal_update(&mut self) -> &Params{
		let (int_0 :f64 , int_1:f64, int : f64) = (0.0,0.0,0.0);
		for indval in self.data.len(){
			int = self.theta.0 + (self.theta.1 * self.data.indval) - self.label.indval;
			int_0 += int;
			int_1 += (int*self.data.indval);
		}
		self.update_int.0 = self.sgd_alpha*(1/self.sample_size)*int_0;
		self.update_int.1 = self.sgd_alpha*(1/self.sample_size)*int_1;
		self
	}
}
