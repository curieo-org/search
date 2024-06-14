pub fn cosine_similarity(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    if v1.len() != v2.len() {
        return 0.0;
    }
    let dot_product = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum::<f64>();

    let magnitude_v1 = v1.iter().map(|a| a * a).sum::<f64>().sqrt();
    let magnitude_v2 = v2.iter().map(|a| a * a).sum::<f64>().sqrt();
    let magnitude_product = magnitude_v1 * magnitude_v2;
    if magnitude_product == 0.0 {
        return 0.0;
    }

    return dot_product / magnitude_product;
}
