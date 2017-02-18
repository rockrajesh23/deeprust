// Task :- Performing gradient descent operations
// Author :- @digneshwer
// Version :- 0.0.1
// Date : - 28 Jan 2017

// Modules used
use std::{f64,i32};

// Main function point of execution
pub fn exec() {
    println!("Gradient Descent Experiment");

    let mut parameter_test = Params {
    	sgd_alpha : 0.1,sample_size : 2.0,theta : (1.0, 1.0),data : (1.0,1.0),label: (1.0,1.0), update_int :(0.0,0.0)
    };
    parameter_test.cal_update();
    parameter_test.theta_zero_update();
    parameter_test.theta_one_update();
    println!("Updated theta_zero : {:?}",parameter_test.theta.0);
    println!("Updated theta_one : {:?}",parameter_test.theta.1);
}

// Defining an user defined data type params
// check the ../../sgd_working.md file variable documentation
#[derive(Debug)]
struct Params {
	sgd_alpha : f64,
	sample_size : f64,
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
		let (mut int_0,mut int_1,mut int) = (0.0,0.0,0.0);
		for index in 0..2{
			int = self.theta.0 + self.theta.1 * self.data.0 - self.label.0;
			int_0 += int;
			int_1 += int*self.data.0;
		}
		self.update_int.0 = (self.sgd_alpha*(1/self.sample_size as i32)as f64)*int_0;
		self.update_int.1 = (self.sgd_alpha*(1/self.sample_size as i32) as f64)*int_1;
		self
	}
}
