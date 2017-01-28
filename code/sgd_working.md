# Program Explanations 

## Explanation of Gradient Descent Code (POC)

The derivation is based on the research from the [dvigneshwer SGD blog](http://dvigneshwer.github.io/blog/2016/12/04/SGD_Optimizer)

POC is based on the GD for linear regression cost function. 

### Variable - details

Below are the details of the variable in the user defined data structure params (which is a rust structure)

* sgd_alpha - This is the learning rate of the model set by the user for accelerating the reach to the local minimum.This will be a float 64 bit value.
* sample_size - Total number of .This will be an interger 64 bit value.
* theta - (f64 theta 0, f64 theta 1) 
* data - (f64,f64) training features
* label - (f64,f64) classes of the features
* update_int - (f64,f64) intermediate updates

### Code Workflow

* Defined an user defined dataset **params** which contain all the parameters
* Used traits **parameter_optimization** to define functionality for the params datatype
* parameter_optimization contains two function **theta_one_update** and **theta_zero_update**