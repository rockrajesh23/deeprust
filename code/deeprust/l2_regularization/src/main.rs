// Task :- l2 regularization for sgd
// Author :- @abhijayghildyal 
// Version :- 1.13.0
// Date : - 5th Feb 2017

fn regularization_term(mut loss : i32, lambda : i32, theta : Vec<i32>) -> i32 {

    let theta_square: i32 = theta[1..theta.len()].iter().zip(theta[1..theta.len()].iter()).map(|(x, y)| x*y).sum();
	loss = lambda*(loss + theta_square)/(2*(theta.len() as i32));
    // code optimization required above

	return loss
}
fn main() {
    // Regularization term
    // let mut theta = vec![1,2,3,4]; // Make mutable later

    // test variables
    let lambda = 4;
    let theta = vec![1,2,3,4];
    let mut loss = 3;

    // printing input
    println!("\nlambda = {0} \ntheta = {1:?} \nloss = {2}", lambda, theta, loss);

    // function call
    loss = regularization_term(loss, lambda, theta);

    // printing result
    println!("(lambda/2m)*theta^2 = {0} \n", loss);
}