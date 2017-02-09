// Task :- l2 regularization for sgd
// Author :- @abhijayghildyal 
// Version :- 1.10
// Date : - 5th Feb 2017

fn regularization_term(lambda: f64, theta: Vec<f64>) -> f64 {

    let theta_square: f64 = theta[1..theta.len()].iter().zip(theta[1..theta.len()].iter()).map(|(x, y)| x*y).sum();

	return lambda*theta_square;
}
fn main() {
    // Regularization term
    // http://www.holehouse.org/mlclass/07_Regularization.html
    // let mut theta = vec![1,2,3,4]; // Make mutable later

    // test variables
    let lambda = 4.0_f64;
    let m = 5.0; // Number of training samples y.len()
    let theta : Vec<f64> = vec![1.0,2.0,3.0,4.0];
    let mut loss = 3.0;

    // printing input
    println!("\nlambda = {0} \nm = {1} \ntheta = {2:?} \nloss = {3}", lambda, m, theta, loss);

    println!("theta^2 = {0} ", regularization_term(lambda, theta.clone())/lambda);
    println!("lambda*theta_square = {0} ", regularization_term(lambda, theta.clone()));
    println!("(lambda*theta_square)/(2.0*m) = {0} ", regularization_term(lambda, theta.clone())/(2.0*m));

    // function call
    loss = loss + regularization_term(lambda, theta)/(2.0*m);

    println!("loss = {} \n", loss);
    
}