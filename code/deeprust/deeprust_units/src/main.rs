// Task : Main file for testing the different deeprust units
// Author : Vigneshwer 
// Date : 18th FEB 2016
// Version : 1.0

extern crate deeprust_units;

fn main() {
    println!("Performing metrics");
    deeprust_units::metrics::confusion_matrix::exec();

    println!("Performing L2 regularization");
    deeprust_units::L2_reg::l2_reg::exec();

    println!("Performing gradient");
    deeprust_units::sgd::gradient::exec();
}