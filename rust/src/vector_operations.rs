pub fn add_vectors(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

pub fn subtract_vectors(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
}

pub fn vector_sum(vectors: Vec<Vec<f32>>) -> Vec<f32> {
    let mut result: Vec<f32> = vectors[0].clone();
    for vector in &vectors[1..] {
        result = add_vectors(result, vector.clone());
    }
    result
}

pub fn scalar_multiply(c: f32, vector: Vec<f32>) -> Vec<f32> {
    vector.iter().map(|v| v * c).collect()
}

pub fn vector_mean(vectors: Vec<Vec<f32>>) -> Vec<f32> {
    scalar_multiply(1.0 / (vectors.len()) as f32, vector_sum(vectors))
}

pub fn dot(a: Vec<f32>, b: Vec<f32>) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(a, b)| a * b)
        .collect::<Vec<f32>>()
        .iter()
        .sum()
}

pub fn sum_of_squares(v: Vec<f32>) -> f32 {
    dot(v.clone(), v.clone())
}

pub fn magnitude(v: Vec<f32>) -> f32 {
    sum_of_squares(v).sqrt()
}

pub fn squared_distance(a: Vec<f32>, b: Vec<f32>) -> f32 {
    sum_of_squares(subtract_vectors(a, b))
}

pub fn distance(a: Vec<f32>, b: Vec<f32>) -> f32 {
    squared_distance(a, b).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vectors() {
        assert_eq!(
            add_vectors(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]),
            vec![2.0, 4.0, 6.0]
        );
    }

    #[test]
    fn test_subtract_vectors() {
        assert_eq!(
            subtract_vectors(vec![2.0, 3.0, 4.0], vec![1.0, 2.0, 3.0]),
            vec![1.0, 1.0, 1.0]
        );
    }

    #[test]
    fn test_scalar_multiply() {
        assert_eq!(
            scalar_multiply(3.0, vec![2.0, 3.0, 4.0]),
            vec![6.0, 9.0, 12.0]
        );
    }

    #[test]
    fn test_vector_mean() {
        assert_eq!(
            vector_mean(vec![vec![1.0, 2.0, 3.0], vec![3.0, 4.0, 5.0]]),
            vec![2.0, 3.0, 4.0]
        );
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(vec![2.0, 3.0, 4.0], vec![6.0, 9.0, 12.0]), 87.0);
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(vec![2.0, 3.0, 4.0]), 29.0);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(magnitude(vec![3.0, 4.0]), 5.0);
        assert_eq!(magnitude(vec![1.0, 2.0, 2.0]), 3.0);
    }

    #[test]
    fn test_squared_distance() {
        assert_eq!(
            squared_distance(vec![3.0, 4.0, 5.0], vec![1.0, 2.0, 3.0]),
            12.0
        );
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(vec![5.0, 6.0], vec![2.0, 2.0]), 5.0);
        assert_eq!(distance(vec![5.0, 6.0, 7.0], vec![4.0, 4.0, 5.0]), 3.0);
    }
}
