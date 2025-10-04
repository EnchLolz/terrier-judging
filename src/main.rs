use ndarray::{array, Array1, Array2};
use scirs2_stats::distributions;

fn big_phi(x: f64, normal: &distributions::Normal<f64>) -> f64 {
    normal.cdf(x)
}

fn cost_function(data: &Array2<i32>, 
                 means: &Array1<f64>, 
                 normal: &distributions::Normal<f64>) -> f64
{
    let (n , _) = data.dim();

    let mut cost = 0.0;
    // more complex stuff

    for i in 0..n {
        for j in 0..n {
            if i == j {continue};
            cost += (data[[i,j]] as f64) * (big_phi(means[i] - means[j], normal)).log2();
        }
    }

    // subtraction
    for i in 0..n {
        cost -= means[i]*means[i]/2.0;
    }

    cost
}

fn calc_expected_means(data: &Array2<i32>) -> Array1<f64> {
    if data.is_empty() {
        return array![];
    }
    let (n, cols) = data.dim();
    assert!(n == cols); // Ensure it's a square matrix
    let mut means: Array1<f64> = Array1::zeros(n);

    means
}



fn main() {
    println!("Hello, world!");
    let data = array![
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    let means = calc_expected_means(&data);
    println!("Means: {:?}", means);
}