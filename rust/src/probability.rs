pub fn uniform_pdf(x: f32) -> f32 {
    if (0.0..1.0).contains(&x) {
        1.0
    } else {
        0.0
    }
}

pub fn uniform_cdf(x: f32) -> f32 {
    if x < 0.0 {
        0.0
    } else if x < 1.0 {
        x
    } else {
        1.0
    }
}

pub fn normal_pdf(x: f32, mu: f32, sigma: f32) -> f32 {
    let two_pi = 2.0 * std::f32::consts::PI;
    let sqrt_two_pi = two_pi.sqrt();
    let sigma_sqrt_two_pi = sigma * sqrt_two_pi;
    let x_mu_sigma = (x - mu) / sigma;
    (1.0 / sigma_sqrt_two_pi) * (-0.5 * x_mu_sigma.powf(2.0)).exp()
}

use statrs::function::erf::erf;

pub fn normal_cdf(x: f32, mu: f32, sigma: f32) -> f32 {
    (1.0 + erf(((x - mu) / (sigma * (2.0_f32).sqrt())) as f64) as f32) / 2.0
}

pub fn inverse_normal_cdf(p: f32, mu: f32, sigma: f32, tolerance: f32) -> f32 {
    if (mu != 0.0) || (sigma != 1.0) {
        mu + sigma * inverse_normal_cdf(p, 0.0, 1.0, tolerance)
    } else {
        let mut low_z = -10.0;
        let mut hi_z = 10.0;
        let mut mid_z: f32 = (low_z + hi_z) / 2.0;
        let mut mid_p: f32;
        while hi_z - low_z > tolerance {
            mid_z = (low_z + hi_z) / 2.0;
            mid_p = normal_cdf(mid_z, 0.0, 1.0);
            if mid_p < p {
                low_z = mid_z;
            } else {
                hi_z = mid_z;
            }
        }
        mid_z
    }
}

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn bernoulli_trial(p: f32, seed: Option<u64>) -> i32 {
    let mut rng: StdRng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_rng(&mut rand::rng()),
    };
    if rng.random::<f32>() < p {
        1
    } else {
        0
    }
}

pub fn binomial(n: i32, p: f32, seed: Option<u64>) -> i32 {
    (0..n).map(|_| bernoulli_trial(p, seed)).sum()
}

#[allow(dead_code)]
fn round(x: f32, decimals: u32) -> f32 {
    let y = 10i32.pow(decimals) as f32;
    (x * y).round() / y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_pdf() {
        assert_eq!(uniform_pdf(0.5), 1.0);
        assert_eq!(uniform_pdf(1.5), 0.0);
    }

    #[test]
    fn test_uniform_cdf() {
        assert_eq!(uniform_cdf(0.5), 0.5);
        assert_eq!(uniform_cdf(1.5), 1.0);
    }

    #[test]
    fn test_normal_pdf() {
        assert_eq!(
            round(normal_pdf(0.0, 0.0, 1.0), 5),
            round(0.3989422804014327, 5)
        );
        assert_eq!(
            round(normal_pdf(1.0, 0.0, 1.0), 5),
            round(0.24197072451914337, 5)
        );
    }

    #[test]
    fn test_normal_cdf() {
        assert_eq!(round(normal_cdf(0.0, 0.0, 1.0), 5), 0.5);
        assert_eq!(
            round(normal_cdf(1.0, 0.0, 1.0), 5),
            round(0.8413447460685429, 5)
        );
    }

    #[test]
    fn test_inverse_normal_cdf() {
        assert_eq!(round(inverse_normal_cdf(0.5, 0.0, 1.0, 1e-5), 2), 0.0);
        assert_eq!(
            round(inverse_normal_cdf(0.8413447460685429, 0.0, 1.0, 1e-5), 2),
            1.0
        );
    }

    #[test]
    fn test_bernoulli_trial() {
        assert_eq!(bernoulli_trial(0.5, Some(222)), 0);
        assert_eq!(bernoulli_trial(0.0, Some(222)), 0);
        assert_eq!(bernoulli_trial(1.0, Some(222)), 1);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(10, 0.5, Some(222)), 0);
        assert_eq!(binomial(10, 0.0, Some(222)), 0);
        assert_eq!(binomial(10, 1.0, Some(222)), 10);
    }
}
