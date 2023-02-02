pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product = a.iter().zip(b).map(|(a, b)| a * b).sum::<f32>();
    let a_norm = a.iter().map(|a| a * a).sum::<f32>().sqrt();
    let b_norm = b.iter().map(|b| b * b).sum::<f32>().sqrt();
    dot_product / (a_norm * b_norm)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::assert_delta;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let c = vec![7.0, 8.0, 9.0];

        assert_delta!(cosine_similarity(&a, &b), 0.97463185, 0.0001);
        assert_delta!(cosine_similarity(&a, &c), 0.99227786, 0.0001);
        assert_delta!(cosine_similarity(&b, &c), 0.99227786, 0.0001);
    }
}
