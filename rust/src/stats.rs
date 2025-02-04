pub fn mean(values: &[f32]) -> f32 {
    values.iter().sum::<f32>() / values.len() as f32
}

fn median_even(values: &[f32]) -> f32 {
    let values_clone: &mut Vec<f32> = &mut vec![];
    (*values).clone_into(values_clone);
    values_clone.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid_index = ((values_clone.len() / 2) as f32).floor() as usize;
    (values_clone[mid_index - 1] + values_clone[mid_index]) / 2.0
}

fn median_odd(values: &[f32]) -> f32 {
    let values_clone: &mut Vec<f32> = &mut vec![];
    (*values).clone_into(values_clone);
    values_clone.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid_index = ((values_clone.len() / 2) as f32).floor() as usize;
    values_clone[mid_index]
}

pub fn median(values: &[f32]) -> f32 {
    if values.len() % 2 == 0 {
        median_even(values)
    } else {
        median_odd(values)
    }
}

pub fn quantile(values: &[f32], p: f32) -> f32 {
    let values_clone: &mut Vec<f32> = &mut vec![];
    (*values).clone_into(values_clone);
    let p_index = (p * values.len() as f32).floor() as i32;
    values_clone.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    values_clone[p_index as usize]
}

use std::collections::HashMap;

pub fn mode(values: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();
    for item in values {
        *frequency_map.entry(item).or_insert(0) += 1;
    }
    *frequency_map
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(val, _)| val)
        .expect("Provided vector is empty.")
}

pub fn data_range(values: &[f32]) -> f32 {
    let value_iter = values.iter();
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

pub fn de_mean(values: &[f32]) -> Vec<f32> {
    let x_bar = mean(values);
    values.iter().map(|x| x - x_bar).collect()
}

pub fn variance(values: &[f32]) -> f32 {
    let n = values.len();
    let deviations = de_mean(values);
    deviations.into_iter().map(|x| x.powf(2.0)).sum::<f32>() / (n - 1) as f32
}

pub fn standard_deviation(values: &[f32]) -> f32 {
    variance(values).sqrt()
}

pub fn interquartile_range(values: &[f32]) -> f32 {
    quantile(values, 0.75) - quantile(values, 0.25)
}

use crate::vector_operations;

pub fn covariance(x: &[f32], y: &[f32]) -> f32 {
    let n = x.len();
    vector_operations::dot(&de_mean(x), &de_mean(y)) / (n - 1) as f32
}

pub fn correlation(x: &[f32], y: &[f32]) -> f32 {
    let stdev_x = standard_deviation(x);
    let stdev_y = standard_deviation(y);
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
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&v), 3.0);
    }

    #[test]
    fn test_median() {
        let v1 = vec![1.0, 2.0, 3.0];
        assert_eq!(median(&v1), 2.0);

        let v1 = vec![1.0, 2.0];
        assert_eq!(median(&v1), 1.5);
    }

    #[test]
    fn test_quantile() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(quantile(&v, 0.5), 2.0);
        assert_eq!(quantile(&v, 0.25), 1.0);
    }

    #[test]
    fn test_mode() {
        let v = vec![1, 2, 2, 3];
        assert_eq!(mode(&v), 2);
    }

    #[test]
    fn test_data_range() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(data_range(&v), 2.0);
    }

    #[test]
    fn test_de_mean() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(de_mean(&v), vec![-1.0, 0.0, 1.0]);
    }

    #[test]
    fn test_variance() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(variance(&v), 1.0);
    }

    #[test]
    fn test_standard_deviation() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(standard_deviation(&v), 1.0);
    }

    #[test]
    fn test_interquartile_range() {
        let v = vec![1.0, 2.0, 3.0];
        assert_eq!(interquartile_range(&v), 2.0);
    }

    #[test]
    fn test_covariance() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        assert_eq!(covariance(&v1, &v2), 1.0);
        assert_eq!(covariance(&v2, &v1), 1.0);
    }

    #[test]
    fn test_correlation() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![4.0, 5.0, 6.0];
        assert_eq!(correlation(&v1, &v2), 1.0);
        assert_eq!(correlation(&v2, &v1), 1.0);
    }
}
