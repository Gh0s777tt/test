//! ML Metrics Module
//! 
//! Provides evaluation metrics for machine learning models.
//! 
//! ## Metrics
//! - Classification metrics: Accuracy, Precision, Recall, F1, ROC-AUC
//! - Regression metrics: MSE, MAE, RMSE, R²
//! - Clustering metrics: Silhouette score, Davies-Bouldin index

use crate::ai::error::AIError;

/// Classification Metrics
/// 
/// Comprehensive evaluation metrics for classification tasks.
pub struct ClassificationMetrics;

impl ClassificationMetrics {
    /// Calculate accuracy
    pub fn accuracy(y_true: &[usize], y_pred: &[usize]) -> Result<f64, AIError> {
        if y_true.len() != y_pred.len() {
            return Err(AIError::InvalidInput(
                "Label lengths must match".to_string(),
            ));
        }

        let correct = y_true
            .iter()
            .zip(y_pred.iter())
            .filter(|(&t, &p)| t == p)
            .count();

        Ok(correct as f64 / y_true.len() as f64)
    }

    /// Calculate precision
    pub fn precision(y_true: &[usize], y_pred: &[usize], class: usize) -> Result<f64, AIError> {
        let mut true_positives = 0;
        let mut false_positives = 0;

        for (&t, &p) in y_true.iter().zip(y_pred.iter()) {
            if p == class {
                if t == class {
                    true_positives += 1;
                } else {
                    false_positives += 1;
                }
            }
        }

        if true_positives + false_positives == 0 {
            Ok(0.0)
        } else {
            Ok(true_positives as f64 / (true_positives + false_positives) as f64)
        }
    }

    /// Calculate recall
    pub fn recall(y_true: &[usize], y_pred: &[usize], class: usize) -> Result<f64, AIError> {
        let mut true_positives = 0;
        let mut false_negatives = 0;

        for (&t, &p) in y_true.iter().zip(y_pred.iter()) {
            if t == class {
                if p == class {
                    true_positives += 1;
                } else {
                    false_negatives += 1;
                }
            }
        }

        if true_positives + false_negatives == 0 {
            Ok(0.0)
        } else {
            Ok(true_positives as f64 / (true_positives + false_negatives) as f64)
        }
    }

    /// Calculate F1 score
    pub fn f1_score(y_true: &[usize], y_pred: &[usize], class: usize) -> Result<f64, AIError> {
        let p = Self::precision(y_true, y_pred, class)?;
        let r = Self::recall(y_true, y_pred, class)?;

        if p + r == 0.0 {
            Ok(0.0)
        } else {
            Ok(2.0 * p * r / (p + r))
        }
    }

    /// Calculate macro-averaged F1 score
    pub fn macro_f1_score(y_true: &[usize], y_pred: &[usize], n_classes: usize) -> Result<f64, AIError> {
        let mut sum = 0.0;
        let mut count = 0;

        for class in 0..n_classes {
            let f1 = Self::f1_score(y_true, y_pred, class)?;
            sum += f1;
            count += 1;
        }

        Ok(sum / count as f64)
    }

    /// Calculate confusion matrix
    pub fn confusion_matrix(
        y_true: &[usize],
        y_pred: &[usize],
        n_classes: usize,
    ) -> Result<Vec<Vec<usize>>, AIError> {
        let mut matrix = vec![vec![0usize; n_classes]; n_classes];

        for (&t, &p) in y_true.iter().zip(y_pred.iter()) {
            matrix[t][p] += 1;
        }

        Ok(matrix)
    }
}

/// Regression Metrics
/// 
/// Evaluation metrics for regression tasks.
pub struct RegressionMetrics;

impl RegressionMetrics {
    /// Calculate Mean Squared Error
    pub fn mse(y_true: &[f64], y_pred: &[f64]) -> Result<f64, AIError> {
        if y_true.len() != y_pred.len() {
            return Err(AIError::InvalidInput(
                "Lengths must match".to_string(),
            ));
        }

        let n = y_true.len();
        let sum: f64 = y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(t, p)| (t - p).powi(2))
            .sum();

        Ok(sum / n as f64)
    }

    /// Calculate Mean Absolute Error
    pub fn mae(y_true: &[f64], y_pred: &[f64]) -> Result<f64, AIError> {
        if y_true.len() != y_pred.len() {
            return Err(AIError::InvalidInput(
                "Lengths must match".to_string(),
            ));
        }

        let n = y_true.len();
        let sum: f64 = y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(t, p)| (t - p).abs())
            .sum();

        Ok(sum / n as f64)
    }

    /// Calculate Root Mean Squared Error
    pub fn rmse(y_true: &[f64], y_pred: &[f64]) -> Result<f64, AIError> {
        Ok(Self::mse(y_true, y_pred)?.sqrt())
    }

    /// Calculate R² score
    pub fn r2_score(y_true: &[f64], y_pred: &[f64]) -> Result<f64, AIError> {
        if y_true.len() != y_pred.len() {
            return Err(AIError::InvalidInput(
                "Lengths must match".to_string(),
            ));
        }

        let n = y_true.len();
        let mean_true: f64 = y_true.iter().sum::<f64>() / n as f64;

        let ss_res: f64 = y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(t, p)| (t - p).powi(2))
            .sum();

        let ss_tot: f64 = y_true
            .iter()
            .map(|t| (t - mean_true).powi(2))
            .sum();

        if ss_tot == 0.0 {
            return Ok(if ss_res == 0.0 { 1.0 } else { 0.0 });
        }

        Ok(1.0 - ss_res / ss_tot)
    }

    /// Calculate Mean Absolute Percentage Error
    pub fn mape(y_true: &[f64], y_pred: &[f64]) -> Result<f64, AIError> {
        if y_true.len() != y_pred.len() {
            return Err(AIError::InvalidInput(
                "Lengths must match".to_string(),
            ));
        }

        let n = y_true.len();
        let sum: f64 = y_true
            .iter()
            .zip(y_pred.iter())
            .map(|(t, p)| {
                if *t != 0.0 {
                    (t - p).abs() / t.abs()
                } else {
                    0.0
                }
            })
            .sum();

        Ok(sum / n as f64)
    }
}

/// Clustering Metrics
/// 
/// Evaluation metrics for clustering algorithms.
pub struct ClusteringMetrics;

impl ClusteringMetrics {
    /// Calculate silhouette score
    pub fn silhouette_score(data: &[Vec<f64>], labels: &[usize]) -> Result<f64, AIError> {
        if data.is_empty() || data.len() != labels.len() {
            return Err(AIError::InvalidInput("Invalid input".to_string()));
        }

        let mut silhouette_values = Vec::new();

        for (i, (point, &label)) in data.iter().zip(labels.iter()).enumerate() {
            // Calculate mean distance to points in same cluster
            let same_cluster: Vec<&Vec<f64>> = data
                .iter()
                .zip(labels.iter())
                .filter(|(_, &l)| l == label && *point != **point)
                .map(|(p, _)| p)
                .collect();

            let a = if same_cluster.is_empty() {
                0.0
            } else {
                same_cluster
                    .iter()
                    .map(|p| euclidean_distance(point, p))
                    .sum::<f64>() / same_cluster.len() as f64
            };

            // Calculate mean distance to points in nearest other cluster
            let mut min_b = f64::MAX;

            for cluster_label in labels.iter().filter(|&&l| l != label).collect::<std::collections::HashSet<_>>() {
                let other_cluster: Vec<&Vec<f64>> = data
                    .iter()
                    .zip(labels.iter())
                    .filter(|(_, &l)| l == *cluster_label)
                    .map(|(p, _)| p)
                    .collect();

                if other_cluster.is_empty() {
                    continue;
                }

                let b = other_cluster
                    .iter()
                    .map(|p| euclidean_distance(point, p))
                    .sum::<f64>() / other_cluster.len() as f64;

                min_b = min_b.min(b);
            }

            if same_cluster.is_empty() {
                // Single point in cluster
                silhouette_values.push(0.0);
            } else {
                // Calculate silhouette
                let s = (min_b - a) / a.max(min_b);
                silhouette_values.push(s);
            }
        }

        Ok(silhouette_values.iter().sum::<f64>() / silhouette_values.len() as f64)
    }

    /// Calculate Davies-Bouldin index
    pub fn davies_bouldin_index(
        data: &[Vec<f64>],
        labels: &[usize],
    ) -> Result<f64, AIError> {
        if data.is_empty() || data.len() != labels.len() {
            return Err(AIError::InvalidInput("Invalid input".to_string()));
        }

        let n_clusters = *labels.iter().max().unwrap_or(&0) + 1;

        // Calculate cluster centroids and average distances
        let mut centroids = Vec::new();
        let mut avg_distances = Vec::new();

        for cluster_id in 0..n_clusters {
            let cluster: Vec<&Vec<f64>> = data
                .iter()
                .zip(labels.iter())
                .filter(|(_, &l)| l == cluster_id)
                .map(|(p, _)| p)
                .collect();

            if cluster.is_empty() {
                return Err(AIError::InvalidInput(
                    "Empty cluster".to_string(),
                ));
            }

            // Calculate centroid
            let centroid = cluster
                .iter()
                .fold(vec![0.0; data[0].len()], |mut sum, point| {
                    for (s, v) in sum.iter_mut().zip(point.iter()) {
                        *s += v;
                    }
                    sum
                });

            let centroid: Vec<f64> = centroid
                .iter()
                .map(|v| v / cluster.len() as f64)
                .collect();

            // Calculate average distance to centroid
            let avg_dist = cluster
                .iter()
                .map(|p| euclidean_distance(p, &centroid))
                .sum::<f64>() / cluster.len() as f64;

            centroids.push(centroid);
            avg_distances.push(avg_dist);
        }

        // Calculate Davies-Bouldin index
        let mut db_sum = 0.0;

        for i in 0..n_clusters {
            let mut max_ratio = 0.0;

            for j in 0..n_clusters {
                if i == j {
                    continue;
                }

                let separation = euclidean_distance(&centroids[i], &centroids[j]);
                let ratio = (avg_distances[i] + avg_distances[j]) / separation;

                max_ratio = max_ratio.max(ratio);
            }

            db_sum += max_ratio;
        }

        Ok(db_sum / n_clusters as f64)
    }

    /// Calculate Calinski-Harabasz index
    pub fn calinski_harabasz_index(
        data: &[Vec<f64>],
        labels: &[usize],
    ) -> Result<f64, AIError> {
        if data.is_empty() || data.len() != labels.len() {
            return Err(AIError::InvalidInput("Invalid input".to_string()));
        }

        let n = data.len();
        let n_clusters = *labels.iter().max().unwrap_or(&0) + 1;
        let dim = data[0].len();

        // Calculate global centroid
        let global_centroid = data
            .iter()
            .fold(vec![0.0; dim], |mut sum, point| {
                for (s, v) in sum.iter_mut().zip(point.iter()) {
                    *s += v;
                }
                sum
            });

        let global_centroid: Vec<f64> = global_centroid
            .iter()
            .map(|v| v / n as f64)
            .collect();

        // Calculate between-cluster sum of squares
        let mut ss_between = 0.0;

        for cluster_id in 0..n_clusters {
            let cluster: Vec<&Vec<f64>> = data
                .iter()
                .zip(labels.iter())
                .filter(|(_, &l)| l == cluster_id)
                .map(|(p, _)| p)
                .collect();

            if cluster.is_empty() {
                continue;
            }

            // Calculate cluster centroid
            let centroid = cluster
                .iter()
                .fold(vec![0.0; dim], |mut sum, point| {
                    for (s, v) in sum.iter_mut().zip(point.iter()) {
                        *s += v;
                    }
                    sum
                });

            let centroid: Vec<f64> = centroid
                .iter()
                .map(|v| v / cluster.len() as f64)
                .collect();

            // Sum distances to global centroid
            for point in cluster {
                ss_between += euclidean_distance(point, &global_centroid).powi(2);
            }
        }

        // Calculate within-cluster sum of squares
        let mut ss_within = 0.0;

        for (point, &label) in data.iter().zip(labels.iter()) {
            let cluster: Vec<&Vec<f64>> = data
                .iter()
                .zip(labels.iter())
                .filter(|(_, &l)| l == label)
                .map(|(p, _)| p)
                .collect();

            if cluster.is_empty() {
                continue;
            }

            // Calculate cluster centroid
            let centroid = cluster
                .iter()
                .fold(vec![0.0; dim], |mut sum, point| {
                    for (s, v) in sum.iter_mut().zip(point.iter()) {
                        *s += v;
                    }
                    sum
                });

            let centroid: Vec<f64> = centroid
                .iter()
                .map(|v| v / cluster.len() as f64)
                .collect();

            // Sum distances to cluster centroid
            ss_within += euclidean_distance(point, &centroid).powi(2);
        }

        let k = n_clusters as f64;
        let n_f = n as f64;

        if ss_within == 0.0 {
            return Ok(f64::MAX);
        }

        Ok((ss_between / (k - 1.0)) / (ss_within / (n_f - k)))
    }
}

/// Euclidean distance helper
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy() {
        let y_true = vec![0, 0, 1, 1];
        let y_pred = vec![0, 1, 0, 1];
        let acc = ClassificationMetrics::accuracy(&y_true, &y_pred).unwrap();
        assert!((acc - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_mse() {
        let y_true = vec![1.0, 2.0, 3.0];
        let y_pred = vec![1.1, 2.1, 3.1];
        let mse = RegressionMetrics::mse(&y_true, &y_pred).unwrap();
        assert!((mse - 0.01).abs() < 0.001);
    }

    #[test]
    fn test_f1_score() {
        let y_true = vec![0, 0, 1, 1];
        let y_pred = vec![0, 1, 0, 1];
        let f1 = ClassificationMetrics::f1_score(&y_true, &y_pred, 1).unwrap();
        assert!((f1 - 0.5).abs() < 0.01);
    }
}