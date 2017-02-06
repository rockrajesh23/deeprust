// Task : Evaluation metrics for deeplearning algorithms results
// Date :- 6th Feb 2017
// Authors :- @dvigneshwer  @nifey
// Version :- 0.0.2

use std::{i32, f32};

// Sample function for assigning values to confusion matrix
fn main() {
    // assigning random values to the confusion matrix
    let sample = Confusionmatrix {
        true_positive: 100,
        true_negative: 50,
        false_positive: 10,
        false_negative: 5,
    };

    println!("The total predictions {}", sample.total());
    // Calculating the accuracy of the model
    println!("Accuracy of the model {:.2}", sample.accuracy());
    // Calculating the precision of the model
    println!("Precision of the model {:.2}", sample.precision());
    // Calculating the true positive rate of the model
    println!("True positive rate of the model {:.2}",
             sample.true_poitive_rate());
    // Calculating the false positive rate of the model
    println!("False positive rate of the model {:.2}",
             sample.false_positive_rate());
    // Calculating the misclassification rate of the model
    println!("Misclassification rate of the model {:.2}",
             sample.misclassification_rate());
    // Calculating the specificity of the model
    println!("Specificity of the model {:.2}", sample.specificity());
    // Calculating the prevalance of the model
    println!("Prevalance of the model {:.2}", sample.prevalance());


}

//defining a struct to represent a confusion matrix for a binary classifier
struct Confusionmatrix {
    true_positive: i32,
    true_negative: i32,
    false_positive: i32,
    false_negative: i32,
}

impl Confusionmatrix {
    //to find total number of predictions
    fn total(&self) -> i32 {
        self.true_positive + self.true_negative + self.false_positive + self.false_negative
    }
    //to find the accuracy of the model
    fn accuracy(&self) -> f32 {
        percentage((self.true_positive as f32 + self.true_negative as f32) / (self.total() as f32))
    }
    //to find the precision of the model
    fn precision(&self) -> f32 {
        percentage((self.true_positive as f32) /
                   (self.true_positive as f32 + self.false_positive as f32))
    }
    //to find the true positive rate of the model
    fn true_poitive_rate(&self) -> f32 {
        percentage((self.true_positive as f32) /
                   (self.true_positive as f32 + self.false_negative as f32))
    }
    //to find the false positive rate of the model
    fn false_positive_rate(&self) -> f32 {
        percentage((self.false_positive as f32) /
                   (self.false_positive as f32 + self.true_negative as f32))
    }
    //to find the misclassification rate of the model
    fn misclassification_rate(&self) -> f32 {
        percentage((self.false_positive as f32 + self.false_negative as f32) /
                   (self.total() as f32))
    }
    //to find the specificity of the model
    fn specificity(&self) -> f32 {
        percentage((self.true_negative as f32) /
                   (self.false_positive as f32 + self.true_negative as f32))
    }
    //to find the prevalance of the model
    fn prevalance(&self) -> f32 {
        percentage((self.true_positive as f32 + self.false_negative as f32) / (self.total() as f32))
    }
}

// Converting to percentage
fn percentage(value: f32) -> f32 {
    value as f32 * 100.0
}
