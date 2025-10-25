use ndarray::{Array1, Array2, array};
use scirs2_optimize::unconstrained::{Method, minimize};
use scirs2_stats::distributions;
use std::error::Error;

// Put this all in a struct

fn big_phi(x: f64, normal: &distributions::Normal<f64>) -> f64 {
    normal.cdf(x)
}

fn cost_function(
    data: &Array2<i32>,
    means: &Array1<f64>,
    normal: &distributions::Normal<f64>,
) -> f64 {
    let (n, _) = data.dim();

    let mut cost = 0.0;
    // more complex stuff

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            };
            cost += (data[[i, j]] as f64) * (big_phi(means[i] - means[j], normal)).log2();
        }
    }

    // subtraction
    for i in 0..n {
        cost -= means[i] * means[i] / 2.0;
    }

    -cost
}

fn calc_expected_means(
    data: &Array2<i32>,
    prev_means: Option<Vec<f64>>,
) -> Result<Array1<f64>, Box<dyn Error>> {
    if data.is_empty() {
        return Ok(array![]);
    }
    let (n, cols) = data.dim();
    if n != cols {
        return Err("non-square matrix".into());
    }
    let normal = distributions::Normal::new(0.0, 1.0)?;
    let x0 = match prev_means {
        Some(x) => {
            if x.len() == n {
                x
            } else {
                return Err("prev_means not of expected length".into());
            }
        }
        None => vec![0.0; n],
    };

    // Use BFGS method
    let result = minimize(
        |means| cost_function(data, &means.to_owned(), &normal),
        &x0,
        Method::BFGS,
        None,
    )?;

    Ok(result.x)
}

pub fn rank(data: &Array2<i32>) -> Result<Vec<usize>, Box<dyn Error>> {
    let means = calc_expected_means(data, None)?;
    let mut indices: Vec<usize> = (0..means.len()).collect();
    indices.sort_by(|&i, &j| means[j].partial_cmp(&means[i]).unwrap());
    Ok(indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = array![
            [0, 0, 0, 0, 0, 0, 0, 0, 13, 0],
            [20, 0, 0, 0, 0, 14, 0, 1, 20, 5],
            [20, 20, 0, 14, 13, 20, 8, 20, 20, 20],
            [20, 20, 6, 0, 7, 20, 3, 19, 20, 20],
            [20, 20, 7, 13, 0, 20, 4, 19, 20, 20],
            [20, 6, 0, 0, 0, 0, 0, 8, 20, 8],
            [20, 20, 12, 17, 16, 20, 0, 20, 20, 20],
            [20, 19, 0, 1, 1, 12, 0, 0, 20, 13],
            [7, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [20, 15, 0, 0, 0, 12, 0, 7, 20, 0]
        ];

        assert_eq!(rank(&data).unwrap(), vec![6, 2, 4, 3, 7, 9, 5, 1, 0, 8]);
    }
}
