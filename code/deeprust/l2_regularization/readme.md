# L2 regularization

### Theory

Here are links to the explanation of l2 regularization  
1. [earthlyabhijay blog](https://earthlyabhijay.wordpress.com/2016/12/06/regularization/)  
2. [Holehous; Regularization from Andrew Ng's lecture ](http://www.holehouse.org/mlclass/07_Regularization.html)  

### Variable - details

Below are the details of the variable in the user defined data structure params (which is a rust structure)

* lambda - regularization parameter; f64
* m - No. of training samples; f64
* theta - Vec<f64>
* loss - mut f64
* theta_square - f64

### Code Workflow

* Declared dummy values for variables lambda, m, theta and loss
* Calculated lambda*theta_square in a separate function (more generalized implementation required)
