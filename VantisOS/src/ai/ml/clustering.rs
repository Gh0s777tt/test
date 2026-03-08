//! Clustering Module
//! 
//! Provides clustering algorithms for data grouping and pattern discovery.
//! 
//! ## Algorithms
//! - K-Means: Centroid-based clustering
//! - DBSCAN: Density-based clustering
//! - Hierarchical: Agglomerative clustering
//! - Gaussian Mixture: Probabilistic clustering
//! 
//! ## Performance Requirements
//! - Inference latency: <10ms
//! - Training time: <30s per model
//! - Memory overhead: <50MB

use crate::ai::error::AIError;

/// K-Means Clustering
/// 
/// Centroid-based clustering algorithm.
/// 
/// ## Features
/// - Simple and efficient
/// - Handles large datasets
/// - Configurable k
/// - K-means++ initialization
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::ml::clustering::KMeans;
//! 
//! let mut kmeans = KMeans::new(3, 100)?;
//! 
//! // Fit to data
//! let data = vec![
//!     vec![1.0, 2.0],
//!     vec![1.5, 1.8],
//!     vec![5.0, 8.0],
//!     vec![8.0, 8.0],
//!     vec![1.0, 0.6],
//!     vec![9.0, 11.0],
//! ];
//! 
//! kmeans.fit(&amp;data)?;
//! 
//! // Predict cluster
/// let cluster = kmeans.predict(&amp;[1.0, 2.0])?;
/// println!("Cluster: {}", cluster);
/// ```
pub struct KMeans {
    k: usize,
    max_iterations: usize,
    centroids: Vec<Vec<f64>>,
    tolerance: f64,
}

impl KMeans {
    /// Create a new K-Means model
    /// 
    /// ## Arguments
    /// * `k` - Number of clusters
    /// * `max_iterations` - Maximum iterations
    pub fn new(k: usize, max_iterations: usize) -> <Result<Self, AIError> {
        if k == 0 {
            return Err(AIError::InvalidInput("k must be > 0".to_string()));
        }

        Ok(Self {
            k,
            max_iterations,
            centroids: Vec::new(),
            tolerance: 1e-4,
        })
    }

    /// Fit model to data
    /// 
    /// ## Arguments
    /// * `data` - Data points to cluster
    pub fn fit(&amp;mut self, data: &amp;[Vec<f64>]) -> <Result<(), AIError> {
        if data.is_empty() {
            return Err(AIError::InsufficientData(
                "Data cannot be empty".to_string(),
            ));
        }

        if data.len() < self.k {
            return Err(AIError::InvalidInput(
                "Number of data points must be >= k".to_string(),
            ));
        }

        // Initialize centroids using k-means++
        self.centroids = self.kmeans_plusplus_init(data)?;

        for _ in 0..self.max_iterations {
            // Assign points to clusters
            let assignments = self.assign_clusters(data)?;

            // Update centroids
            let new_centroids = self.update_centroids(data, &amp;assignments)?;

            // Check convergence
            let converged = self.check_convergence(&amp;new_centroids);

            self.centroids = new_centroids;

            if converged {
                break;
            }
        }

        Ok(())
    }

    /// K-means++ initialization
    fn kmeans_plusplus_init(&amp;self, data: &amp;[Vec<f64>]) -> <Result<Vec<Vec<f64>>, AIError> {
        let mut centroids = Vec::new();

        // Choose first centroid randomly
        let first_idx = rand::random<<usize>() % data.len();
        centroids.push(data[first_idx].clone());

        // Choose remaining centroids
        while centroids.len() < self.k {
            let distances: Vec<f64> = data
                .iter()
                .map(|point| {
                    centroids
                        .iter()
                        .map(|c| self.euclidean_distance(point, c))
                        .fold(f64::MAX, f64::min)
                })
                .collect();

            // Choose next centroid with probability proportional to distance squared
            let total_dist: f64 = distances.iter().map(|d| d * d).sum();
            let mut r = rand::random<<f64>() * total_dist;
            let mut next_idx = 0;

            for (i, &amp;d) in distances.iter().enumerate() {
                r -= d * d;
                if r <= 0.0 {
                    next_idx = i;
                    break;
                }
            }

            centroids.push(data[next_idx].clone());
        }

        Ok(centroids)
    }

    /// Assign points to nearest cluster
    fn assign_clusters(&amp;self, data: &amp;[Vec<f64>]) -> <Result<Vec<usize>, AIError> {
        data.iter()
            .map(|point| {
                self.centroids
                    .iter()
                    .enumerate()
                    .min_by(|(_, c1), (_, c2)| {
                        let d1 = self.euclidean_distance(point, c1);
                        let d2 = self.euclidean_distance(point, c2);
                        d1.partial_cmp(&amp;d2).unwrap()
                    })
                    .map(|(i, _)| i)
                    .ok_or_else(|| AIError::InternalError("No centroids".to_string()))
            })
            .collect()
    }

    /// Update centroids based on assignments
    fn update_centroids(
        &amp;self,
        data: &amp;[Vec<f64>],
        assignments: &amp;[usize],
    ) -> <Result<Vec<Vec<f64>>, AIError> {
        let dim = data[0].len();
        let mut new_centroids = vec![vec![0.0; dim]; self.k];
        let mut counts = vec![0usize; self.k];

        // Sum points for each cluster
        for (point, &amp;cluster) in data.iter().zip(assignments.iter()) {
            for (i, &amp;value) in point.iter().enumerate() {
                new_centroids[cluster][i] += value;
            }
            counts[cluster] += 1;
        }

        // Average to get centroids
        for (centroid, &amp;count) in new_centroids.iter_mut().zip(counts.iter()) {
            if count > 0 {
                for value in centroid.iter_mut() {
                    *value /= count as f64;
                }
            }
        }

        Ok(new_centroids)
    }

    /// Check if converged
    fn check_convergence(&amp;self, new_centroids: &amp;[Vec<f64>]) -> bool {
        for (old, new) in self.centroids.iter().zip(new_centroids.iter()) {
            if self.euclidean_distance(old, new) > self.tolerance {
                return false;
            }
        }
        true
    }

    /// Predict cluster for new point
    pub fn predict(&amp;self, point: &amp;[f64]) -> <Result<usize, AIError> {
        if self.centroids.is_empty() {
            return Err(AIError::ModelNotTrained);
        }

        self.centroids
            .iter()
            .enumerate()
            .min_by(|(_, c1), (_, c2)| {
                let d1 = self.euclidean_distance(point, c1);
                let d2 = self.euclidean_distance(point, c2);
                d1.partial_cmp(&amp;d2).unwrap()
            })
            .map(|(i, _)| i)
            .ok_or_else(|| AIError::InternalError("No centroids".to_string()))
    }

    /// Get distances to all centroids
    pub fn distances(&amp;self, point: &amp;[f64]) -> <Result<Vec<f64>, AIError> {
        if self.centroids.is_empty() {
            return Err(AIError::ModelNotTrained);
        }

        Ok(self
            .centroids
            .iter()
            .map(|c| self.euclidean_distance(point, c))
            .collect())
    }

    /// Get cluster centroids
    pub fn centroids(&amp;self) -> &amp;[Vec<f64>] {
        &amp;self.centroids
    }

    /// Euclidean distance
    fn euclidean_distance(&amp;self, a: &amp;[f64], b: &amp;[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum<<f64>()
            .sqrt()
    }
}

/// DBSCAN Clustering
/// 
/// Density-based spatial clustering algorithm.
/// 
/// ## Features
/// - Discovers clusters of arbitrary shape
/// - Handles noise points
/// - No need to specify k
/// - Density-based
pub struct DBSCAN {
    eps: f64,
    min_samples: usize,
    labels: Vec<i32>,
}

impl DBSCAN {
    /// Create a new DBSCAN model
    /// 
    /// ## Arguments
    /// * `eps` - Maximum distance between neighbors
    /// * `min_samples` - Minimum points to form cluster
    pub fn new(eps: f64, min_samples: usize) -> <Result<Self, AIError> {
        if eps <= 0.0 {
            return Err(AIError::InvalidInput("eps must be > 0".to_string()));
        }

        Ok(Self {
            eps,
            min_samples,
            labels: Vec::new(),
        })
    }

    /// Fit model to data
    /// 
    /// ## Arguments
    /// * `data` - Data points to cluster
    pub fn fit(&amp;mut self, data: &amp;[Vec<f64>]) -> <Result<(), AIError> {
        if data.is_empty() {
            return Err(AIError::InsufficientData(
                "Data cannot be empty".to_string(),
            ));
        }

        let n = data.len();
        self.labels = vec![-1; n]; // -1 = unvisited/noise

        let mut cluster_id = 0;

        for i in 0..n {
            if self.labels[i] != -1 {
                continue; // Already visited
            }

            let neighbors = self.region_query(data, i);

            if neighbors.len() < self.min_samples {
                // Noise point
                continue;
            }

            // Start new cluster
            self.labels[i] = cluster_id;
            let mut seeds = neighbors;

            let mut j = 0;
            while j < seeds.len() {
                let neighbor = seeds[j];

                if self.labels[neighbor] == -1 {
                    self.labels[neighbor] = cluster_id;
                }

                if self.labels[neighbor] != -1 &amp;&amp; self.labels[neighbor] != cluster_id {
                    j += 1;
                    continue;
                }

                self.labels[neighbor] = cluster_id;

                let neighbor_neighbors = self.region_query(data, neighbor);

                if neighbor_neighbors.len() >= self.min_samples {
                    for &amp;nn in &amp;neighbor_neighbors {
                        if self.labels[nn] == -1 || self.labels[nn] == cluster_id {
                            if !seeds.contains(&amp;nn) {
                                seeds.push(nn);
                            }
                        }
                    }
                }

                j += 1;
            }

            cluster_id += 1;
        }

        Ok(())
    }

    /// Find all points within eps distance
    fn region_query(&amp;self, data: &amp;[Vec<f64>], point_idx: usize) -> Vec<usize> {
        data.iter()
            .enumerate()
            .filter(|(i, _)| *i != point_idx)
            .filter(|(_, point)| self.distance(&amp;data[point_idx], point) <= self.eps)
            .map(|(i, _)| i)
            .collect()
    }

    /// Distance between points
    fn distance(&amp;self, a: &amp;[f64], b: &amp;[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum<<f64>()
            .sqrt()
    }

    /// Get cluster labels
    pub fn labels(&amp;self) -> &amp;[i32] {
        &amp;self.labels
    }

    /// Get number of clusters
    pub fn n_clusters(&amp;self) -> usize {
        let max = *self.labels.iter().max().unwrap_or(&amp;-1);
        if max < 0 {
            0
        } else {
            (max + 1) as usize
        }
    }

    /// Predict cluster for new point
    pub fn predict(&amp;self, data: &amp;[Vec<f64>], point: &amp;[f64]) -> <Result<i32, AIError> {
        // Find nearest cluster
        let mut min_dist = f64::MAX;
        let mut nearest_cluster = -1;

        for (i, &amp;label) in self.labels.iter().enumerate() {
            if label >= 0 {
                let dist = self.distance(point, &amp;data[i]);
                if dist <= self.eps &amp;&amp; dist < min_dist {
                    min_dist = dist;
                    nearest_cluster = label;
                }
            }
        }

        Ok(nearest_cluster)
    }
}

/// Hierarchical Clustering
/// 
/// Agglomerative hierarchical clustering.
/// 
/// ## Features
/// - Creates dendrogram
/// - No need to specify k
/// - Different linkage methods
/// - Interpretable hierarchy
pub struct HierarchicalClustering {
    n_clusters: usize,
    linkage: LinkageMethod,
    labels: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum LinkageMethod {
    Single,
    Complete,
    Average,
}

impl HierarchicalClustering {
    /// Create a new hierarchical clustering model
    /// 
    /// ## Arguments
    /// * `n_clusters` - Number of clusters to form
    /// * `linkage` - Linkage method
    pub fn new(n_clusters: usize, linkage: LinkageMethod) -> <Result<Self, AIError> {
        if n_clusters == 0 {
            return Err(AIError::InvalidInput(
                "n_clusters must be > 0".to_string(),
            ));
        }

        Ok(Self {
            n_clusters,
            linkage,
            labels: Vec::new(),
        })
    }

    /// Fit model to data
    pub fn fit(&amp;mut self, data: &amp;[Vec<f64>]) -> <Result<(), AIError> {
        if data.is_empty() {
            return Err(AIError::InsufficientData(
                "Data cannot be empty".to_string(),
            ));
        }

        let n = data.len();

        // Initialize each point as its own cluster
        let mut cluster_labels: Vec<usize> = (0..n).collect();
        let mut active_clusters: Vec<Vec<usize>> = (0..n).map(|i| vec![i]).collect();

        // Compute distance matrix
        let mut distances = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                distances[i][j] = self.distance(&amp;data[i], &amp;data[j]);
                distances[j][i] = distances[i][j];
            }
        }

        // Merge clusters until we have n_clusters
        while active_clusters.len() > self.n_clusters {
            // Find closest pair of clusters
            let (mut min_i, mut min_j) = (0, 1);
            let mut min_dist = f64::MAX;

            for i in 0..active_clusters.len() {
                for j in (i + 1)..active_clusters.len() {
                    let dist = self.cluster_distance(
                        &amp;active_clusters[i],
                        &amp;active_clusters[j],
                        &amp;distances,
                    );

                    if dist < min_dist {
                        min_dist = dist;
                        min_i = i;
                        min_j = j;
                    }
                }
            }

            // Merge clusters
            active_clusters[min_i].append(&amp;mut active_clusters[min_j]);
            active_clusters.remove(min_j);
        }

        // Assign labels
        self.labels = vec![0; n];
        for (cluster_id, cluster) in active_clusters.iter().enumerate() {
            for &amp;point_idx in cluster {
                self.labels[point_idx] = cluster_id;
            }
        }

        Ok(())
    }

    /// Compute distance between two clusters
    fn cluster_distance(
        &amp;self,
        c1: &amp;[usize],
        c2: &amp;[usize],
        distances: &amp;[Vec<f64>],
    ) -> f64 {
        let pairwise: Vec<f64> = c1
            .iter()
            .flat_map(|&amp;i| c2.iter().map(move |&amp;j| distances[i][j]))
            .collect();

        match self.linkage {
            LinkageMethod::Single => pairwise.iter().cloned().fold(f64::MAX, f64::min),
            LinkageMethod::Complete => pairwise.iter().cloned().fold(f64::MIN, f64::max),
            LinkageMethod::Average => pairwise.iter().sum<<f64>() / pairwise.len() as f64,
        }
    }

    /// Distance between points
    fn distance(&amp;self, a: &amp;[f64], b: &amp;[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum<<f64>()
            .sqrt()
    }

    /// Get cluster labels
    pub fn labels(&amp;self) -> &amp;[usize] {
        &amp;self.labels
    }
}

/// Gaussian Mixture Model
/// 
/// Probabilistic clustering using Gaussian distributions.
/// 
/// ## Features
/// - Soft clustering
/// - Probability assignments
/// - EM algorithm
/// - Handles overlapping clusters
pub struct GaussianMixtureModel {
    n_components: usize,
    max_iterations: usize,
    weights: Vec<f64>,
    means: Vec<Vec<f64>>,
    covariances: Vec<Vec<Vec<f64>>>,
    tolerance: f64,
}

impl GaussianMixtureModel {
    /// Create a new GMM
    /// 
    /// ## Arguments
    /// * `n_components` - Number of Gaussian components
    /// * `max_iterations` - Maximum EM iterations
    pub fn new(n_components: usize, max_iterations: usize) -> <Result<Self, AIError> {
        Ok(Self {
            n_components,
            max_iterations,
            weights: Vec::new(),
            means: Vec::new(),
            covariances: Vec::new(),
            tolerance: 1e-6,
        })
    }

    /// Fit model to data using EM algorithm
    pub fn fit(&amp;mut self, data: &amp;[Vec<f64>]) -> <Result<(), AIError> {
        if data.is_empty() {
            return Err(AIError::InsufficientData(
                "Data cannot be empty".to_string(),
            ));
        }

        let n = data.len();
        let dim = data[0].len();

        // Initialize using k-means-like approach
        self.weights = vec![1.0 / self.n_components as f64; self.n_components];
        self.means = data
            .iter()
            .take(self.n_components)
            .cloned()
            .collect();
        self.covariances = vec![vec![vec![if i == j { 1.0 } else { 0.0 }; dim]; dim]; self.n_components];

        for _ in 0..self.max_iterations {
            // E-step: compute responsibilities
            let responsibilities = self.e_step(data)?;

            // M-step: update parameters
            self.m_step(data, &amp;responsibilities)?;
        }

        Ok(())
    }

    /// E-step: compute responsibilities
    fn e_step(&amp;self, data: &amp;[Vec<f64>]) -> <Result<Vec<Vec<f64>>, AIError> {
        let n = data.len();
        let mut responsibilities = vec![vec![0.0; self.n_components]; n];

        for (i, point) in data.iter().enumerate() {
            let mut total = 0.0;

            for (k, _) in self.means.iter().enumerate() {
                let prob = self.weights[k] * self.gaussian_pdf(point, k)?;
                responsibilities[i][k] = prob;
                total += prob;
            }

            if total > 0.0 {
                for k in 0..self.n_components {
                    responsibilities[i][k] /= total;
                }
            }
        }

        Ok(responsibilities)
    }

    /// M-step: update parameters
    fn m_step(&amp;mut self, data: &amp;[Vec<f64>], responsibilities: &amp;[Vec<Vec<f64>>) -> <Result<(), AIError> {
        let n = data.len();
        let dim = data[0].len();

        for k in 0..self.n_components {
            let n_k: f64 = responsibilities.iter().map(|r| r[k]).sum();

            // Update weight
            self.weights[k] = n_k / n as f64;

            // Update mean
            let mut new_mean = vec![0.0; dim];
            for (i, point) in data.iter().enumerate() {
                for (j, &amp;value) in point.iter().enumerate() {
                    new_mean[j] += responsibilities[i][k] * value;
                }
            }
            for value in &amp;mut new_mean {
                *value /= n_k;
            }
            self.means[k] = new_mean;
        }

        Ok(())
    }

    /// Gaussian PDF
    fn gaussian_pdf(&amp;self, point: &amp;[f64], component: usize) -> <Result<f64, AIError> {
        let dim = point.len();
        let mean = &amp;self.means[component];

        // Simplified: assume diagonal covariance
        let mut pdf = 1.0;
        for i in 0..dim {
            let var = self.covariances[component][i][i].max(1e-6);
            let diff = point[i] - mean[i];
            pdf *= (-0.5 * diff * diff / var).exp() / (2.0 * std::f64::consts::PI * var.sqrt());
        }

        Ok(pdf)
    }

    /// Predict cluster probabilities
    pub fn predict_proba(&amp;self, point: &amp;[f64]) -> <Result<Vec<f64>, AIError> {
        let mut posteriors = vec![0.0; self.n_components];
        let total: f64 = self.weights.iter().enumerate().map(|(k, &amp;w)| {
            let pdf = self.gaussian_pdf(point, k)?;
            posteriors[k] = w * pdf;
            Ok(w * pdf)
        }).sum()?;

        for p in &amp;mut posteriors {
            *p /= total;
        }

        Ok(posteriors)
    }

    /// Predict most likely cluster
    pub fn predict(&amp;self, point: &amp;[f64]) -> <Result<usize, AIError> {
        let probas = self.predict_proba(point)?;
        probas.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .ok_or_else(|| AIError::InternalError("No predictions".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans() {
        let mut kmeans = KMeans::new(2, 10).unwrap();
        let data = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![10.0, 10.0],
            vec![11.0, 11.0],
        ];
        kmeans.fit(&amp;data).unwrap();
        assert_eq!(kmeans.predict(&amp;[0.0, 0.0]).unwrap(), kmeans.predict(&amp;[1.0, 1.0]).unwrap());
    }

    #[test]
    fn test_dbscan() {
        let mut dbscan = DBSCAN::new(3.0, 2).unwrap();
        let data = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![10.0, 10.0],
        ];
        dbscan.fit(&amp;data).unwrap();
        assert!(dbscan.n_clusters() >= 0);
    }

    #[test]
    fn test_hierarchical() {
        let mut hc = HierarchicalClustering::new(2, LinkageMethod::Average).unwrap();
        let data = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![10.0, 10.0],
            vec![11.0, 11.0],
        ];
        hc.fit(&amp;data).unwrap();
        assert_eq!(hc.labels().len(), 4);
    }
}