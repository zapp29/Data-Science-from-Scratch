pub fn mean(values: Vec<f32>) -> f32 {
    values.iter().sum::<f32>() / values.len() as f32
}

fn median_even(values: Vec<f32>) -> f32 {
    let mut values_clone = values.clone();
    values_clone.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid_index = ((values_clone.len() / 2) as f32).floor() as usize;
    (values_clone[mid_index - 1] + values_clone[mid_index]) / 2.0
}

fn median_odd(values: Vec<f32>) -> f32 {
    let mut values_clone = values.clone();
    values_clone.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid_index = ((values_clone.len() / 2) as f32).floor() as usize;
    values_clone[mid_index]
}

pub fn median(values: Vec<f32>) -> f32 {
    if values.len() % 2 == 0 {
        median_even(values)
    } else {
        median_odd(values)
    }
}

pub fn quantile(values: Vec<f32>, p: f32) -> f32 {
    let mut values_clone = values.clone();
    let p_index = (p * values.len() as f32).floor() as i32;
    values_clone.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    values_clone[p_index as usize]
}

use std::collections::HashMap;

pub fn mode(values: Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();
    for item in values {
        *frequency_map.entry(item).or_insert(0) += 1;
    }
    frequency_map
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| val)
        .expect("Provided vector is empty.")
}

pub fn data_range(values: Vec<f32>) -> f32 {
    let value_iter = values.into_iter();
    let max_value = value_iter
        .clone()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .expect("Provided vector is empty.");
    let min_value = value_iter
        .clone()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .expect("Provided vector is empty.");

    max_value - min_value
}

pub fn de_mean(values: Vec<f32>) -> Vec<f32> {
    let x_bar = mean(values.clone());
    values.into_iter().map(|x| x - x_bar).collect()
}

pub fn variance(values: Vec<f32>) -> f32 {
    let n = values.len();
    let deviations = de_mean(values);
    deviations.into_iter().map(|x| x.powf(2.0)).sum::<f32>() / (n - 1) as f32
}

pub fn standard_deviation(values: Vec<f32>) -> f32 {
    variance(values).sqrt()
}

pub fn interquartile_range(values: Vec<f32>) -> f32 {
    quantile(values.clone(), 0.75) - quantile(values.clone(), 0.25)
}

use crate::vector_operations;

pub fn covariance(x: Vec<f32>, y: Vec<f32>) -> f32 {
    let n = x.len();
    vector_operations::dot(de_mean(x), de_mean(y)) / (n - 1) as f32
}

pub fn correlation(x: Vec<f32>, y: Vec<f32>) -> f32 {
    let stdev_x = standard_deviation(x.clone());
    let stdev_y = standard_deviation(y.clone());
    if (stdev_x > 0.0) & (stdev_y > 0.0) {
        covariance(x, y) / stdev_x / stdev_y
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        assert_eq!(mean(vec![1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(vec![1.0, 2.0, 3.0]), 2.0);
        assert_eq!(median(vec![1.0, 2.0]), 1.5);
    }

    #[test]
    fn test_quantile() {
        assert_eq!(quantile(vec![1.0, 2.0, 3.0], 0.5), 2.0);
        assert_eq!(quantile(vec![1.0, 2.0, 3.0], 0.25), 1.0);
    }

    #[test]
    fn test_mode() {
        assert_eq!(mode(vec![1, 2, 2, 3]), 2);
    }

    #[test]
    fn test_data_range() {
        assert_eq!(data_range(vec![1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn test_de_mean() {
        assert_eq!(de_mean(vec![1.0, 2.0, 3.0]), vec![-1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_variance() {
        assert_eq!(variance(vec![1.0, 2.0, 3.0]), 1.0);
    }

    #[test]
    fn test_standard_deviation() {
        assert_eq!(standard_deviation(vec![1.0, 2.0, 3.0]), 1.0);
    }

    #[test]
    fn test_interquartile_range() {
        assert_eq!(interquartile_range(vec![1.0, 2.0, 3.0]), 2.0);
    }

    #[test]
    fn test_covariance() {
        assert_eq!(covariance(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]), 1.0);
    }

    #[test]
    fn test_correlation() {
        assert_eq!(correlation(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]), 1.0);
    }
}
