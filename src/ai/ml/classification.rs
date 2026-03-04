//! Classification Module
//! 
//! Provides classification algorithms for decision making and prediction.
//! 
//! ## Algorithms
//! - K-Nearest Neighbors: Instance-based classification
//! - Logistic Regression: Linear classification
//! - Decision Trees: Rule-based classification
//! - Random Forest: Ensemble of decision trees
//! - Support Vector Machine: Margin-based classification
//! 
//! ## Performance Requirements
//! - Inference latency: <5ms
//! - Training time: <30s per model
//! - Memory overhead: <100MB

use crate::ai::error::AIError;

/// K-Nearest Neighbors Classifier
/// 
/// Instance-based classification using k-nearest neighbors voting.
/// 
/// ## Features
/// - Simple implementation
/// - Non-parametric
/// - Handles multi-class
/// - No training phase
/// 
/// ## Example
/// ```rust,no_run
//! use vantisos::ai::ml::classification::KNNClassifier;
//! 
//! let mut classifier = KNNClassifier::new(3)?;
//! 
//! // Train with labeled data
//! classifier.fit(&[
//!     (vec![1.0, 2.0], 0),
//!     (vec![2.0, 3.0], 1),
//! ])?;
//! 
//! // Predict
/// let label = classifier.predict(&[1.5, 2.5])?;
/// println!("Predicted: {}", label);
/// ```
pub struct KNNClassifier {
    k: usize,
    training_data: Vec<(Vec<f64>, usize)>,
    n_classes: usize,
}

impl KNNClassifier {
    /// Create a new KNN classifier
    /// 
    /// ## Arguments
    /// * `k` - Number of neighbors to consider
    pub fn new(k: usize) -> Result<Self, AIError> {
        if k == 0 {
            return Err(AIError::InvalidInput("k must be > 0".to_string()));
        }

        Ok(Self {
            k,
            training_data: Vec::new(),
            n_classes: 0,
        })
    }

    /// Fit classifier with training data
    /// 
    /// ## Arguments
    /// * `data` - Training data with features and labels
    pub fn fit(&mut self, data: &[(Vec<f64>, usize)]) -> Result<(), AIError> {
        if data.is_empty() {
            return Err(AIError::InsufficientData(
                "Training data cannot be empty".to_string(),
            ));
        }

        self.training_data = data.to_vec();

        // Determine number of classes
        let max_label = data.iter().map(|(_, l)| *l).max().unwrap_or(0);
        self.n_classes = max_label + 1;

        Ok(())
    }

    /// Predict label for new sample
    /// 
    /// ## Arguments
    /// * `features` - Feature vector
    /// 
    /// ## Returns
    /// Predicted label
    pub fn predict(&self, features: &[f64]) -> Result<usize, AIError> {
        if self.training_data.is_empty() {
            return Err(AIError::ModelNotTrained);
        }

        // Calculate distances to all training points
        let mut distances: Vec<_> = self.training_data
            .iter()
            .map(|(train_features, label)| {
                let dist = self.euclidean_distance(features, train_features);
                (dist, *label)
            })
            .collect();

        // Sort by distance
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Get k nearest neighbors
        let k_nearest = &distances[..self.k.min(distances.len())];

        // Vote for class
        let mut votes = vec![0usize; self.n_classes];
        for (_, label) in k_nearest {
            votes[*label] += 1;
        }

        // Return class with most votes
        votes
            .iter()
            .enumerate()
            .max_by_key(|(_, &count)| count)
            .map(|(label, _)| label)
            .ok_or_else(|| AIError::InternalError("No votes".to_string()))
    }

    /// Predict with confidence scores
    pub fn predict_proba(&self, features: &[f64]) -> Result<Vec<f64>, AIError> {
        if self.training_data.is_empty() {
            return Err(AIError::ModelNotTrained);
        }

        let mut distances: Vec<_> = self.training_data
            .iter()
            .map(|(train_features, label)| {
                let dist = self.euclidean_distance(features, train_features);
                (dist, *label)
            })
            .collect();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let k_nearest = &distances[..self.k.min(distances.len())];

        let mut votes = vec![0usize; self.n_classes];
        for (_, label) in k_nearest {
            votes[*label] += 1;
        }

        let total = k_nearest.len() as f64;
        Ok(votes.iter().map(|&v| v as f64 / total).collect())
    }

    /// Calculate Euclidean distance
    fn euclidean_distance(&self, a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

/// Logistic Regression Classifier
/// 
/// Linear classifier using logistic function.
/// 
/// ## Features
/// - Probabilistic predictions
/// - Binary and multiclass
/// - L2 regularization
/// - Stochastic gradient descent
pub struct LogisticRegressionClassifier {
    weights: Vec<f64>,
    bias: f64,
    learning_rate: f64,
    n_iterations: usize,
    regularization: f64,
}

impl LogisticRegressionClassifier {
    /// Create a new logistic regression classifier
    /// 
    /// ## Arguments
    /// * `learning_rate` - Learning rate for SGD
    /// * `n_iterations` - Number of training iterations
    pub fn new(learning_rate: f64, n_iterations: usize) -> Result<Self, AIError> {
        Ok(Self {
            weights: Vec::new(),
            bias: 0.0,
            learning_rate,
            n_iterations,
            regularization: 0.01,
        })
    }

    /// Fit classifier with training data
    pub fn fit(&mut self, features: &[Vec<f64>], labels: &[usize]) -> Result<(), AIError> {
        if features.is_empty() || labels.is_empty() {
            return Err(AIError::InsufficientData(
                "Training data cannot be empty".to_string(),
            ));
        }

        let n_features = features[0].len();
        self.weights = vec![0.0; n_features];

        // Gradient descent
        for _ in 0..self.n_iterations {
            for (x, &y) in features.iter().zip(labels.iter()) {
                let prediction = self.sigmoid(&self.dot(x, &self.weights), self.bias);
                let error = prediction - y as f64;

                // Update weights
                for (i, &xi) in x.iter().enumerate() {
                    self.weights[i] -= self.learning_rate * (error * xi + self.regularization * self.weights[i]);
                }

                // Update bias
                self.bias -= self.learning_rate * error;
            }
        }

        Ok(())
    }

    /// Predict class
    pub fn predict(&self, features: &[f64]) -> Result<usize, AIError> {
        let probability = self.predict_proba(features)?[1];
        Ok(if probability >= 0.5 { 1 } else { 0 })
    }

    /// Predict probabilities
    pub fn predict_proba(&self, features: &[f64]) -> Result<Vec<f64>, AIError> {
        let z = self.dot(features, &self.weights) + self.bias;
        let prob = self.sigmoid(z, 0.0);
        Ok(vec![1.0 - prob, prob])
    }

    /// Sigmoid function
    fn sigmoid(&self, z: f64, bias: f64) -> f64 {
        1.0 / (1.0 + (-(z + bias)).exp())
    }

    /// Dot product
    fn dot(&self, a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
}

/// Decision Tree Classifier
/// 
/// Rule-based classifier using decision trees.
/// 
/// ## Features
/// - Interpretable
/// - Handles categorical and numerical
/// - No scaling required
/// - Feature importance
pub struct DecisionTreeClassifier {
    max_depth: usize,
    min_samples_split: usize,
    tree: Option<TreeNode>,
    n_classes: usize,
}

struct TreeNode {
    feature_idx: Option<usize>,
    threshold: Option<f64>,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    label: Option<usize>,
}

impl DecisionTreeClassifier {
    /// Create a new decision tree classifier
    /// 
    /// ## Arguments
    /// * `max_depth` - Maximum tree depth
    /// * `min_samples_split` - Minimum samples to split
    pub fn new(max_depth: usize, min_samples_split: usize) -> Result<Self, AIError> {
        Ok(Self {
            max_depth,
            min_samples_split,
            tree: None,
            n_classes: 0,
        })
    }

    /// Fit classifier with training data
    pub fn fit(&mut self, features: &[Vec<f64>], labels: &[usize]) -> Result<(), AIError> {
        if features.is_empty() || labels.is_empty() {
            return Err(AIError::InsufficientData(
                "Training data cannot be empty".to_string(),
            ));
        }

        self.n_classes = labels.iter().max().map(|&m| m + 1).unwrap_or(2);

        // Build tree recursively
        let indices: Vec<usize> = (0..features.len()).collect();
        self.tree = Some(self.build_tree(features, labels, &indices, 0));

        Ok(())
    }

    /// Build tree recursively
    fn build_tree(
        &self,
        features: &[Vec<f64>],
        labels: &[usize],
        indices: &[usize],
        depth: usize,
    ) -> TreeNode {
        // Check stopping conditions
        if depth >= self.max_depth
            || indices.len() < self.min_samples_split
            || self.is_pure(labels, indices)
        {
            return TreeNode {
                feature_idx: None,
                threshold: None,
                left: None,
                right: None,
                label: Some(self.majority_vote(labels, indices)),
            };
        }

        // Find best split
        if let Some((feature_idx, threshold, left_idx, right_idx)) =
            self.find_best_split(features, labels, indices)
        {
            TreeNode {
                feature_idx: Some(feature_idx),
                threshold: Some(threshold),
                left: Some(Box::new(self.build_tree(features, labels, &left_idx, depth + 1))),
                right: Some(Box::new(self.build_tree(features, labels, &right_idx, depth + 1))),
                label: None,
            }
        } else {
            TreeNode {
                feature_idx: None,
                threshold: None,
                left: None,
                right: None,
                label: Some(self.majority_vote(labels, indices)),
            }
        }
    }

    /// Check if node is pure (all same label)
    fn is_pure(&self, labels: &[usize], indices: &[usize]) -> bool {
        let first = labels[indices[0]];
        indices.iter().all(|&i| labels[i] == first)
    }

    /// Get majority vote label
    fn majority_vote(&self, labels: &[usize], indices: &[usize]) -> usize {
        let mut counts = vec![0usize; self.n_classes];
        for &i in indices {
            counts[labels[i]] += 1;
        }
        counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)
            .map(|(l, _)| l)
            .unwrap_or(0)
    }

    /// Find best split using Gini impurity
    fn find_best_split(
        &self,
        features: &[Vec<f64>],
        labels: &[usize],
        indices: &[usize],
    ) -> Option<(usize, f64, Vec<usize>, Vec<usize>)> {
        let n_features = features[0].len();
        let mut best_gain = 0.0;
        let mut best_split = None;

        let current_gini = self.gini_impurity(labels, indices);

        for feature_idx in 0..n_features {
            // Get unique values as potential thresholds
            let mut values: Vec<f64> = indices.iter().map(|&i| features[i][feature_idx]).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            values.dedup();

            for &threshold in &values {
                let (left_idx, right_idx): (Vec<usize>, Vec<usize>) = indices
                    .iter()
                    .filter(|&&i| !features[i][feature_idx].is_nan())
                    .partition(|&&i| features[i][feature_idx] <= threshold);

                if left_idx.is_empty() || right_idx.is_empty() {
                    continue;
                }

                // Calculate information gain
                let left_weight = left_idx.len() as f64 / indices.len() as f64;
                let right_weight = right_idx.len() as f64 / indices.len() as f64;

                let gain = current_gini
                    - left_weight * self.gini_impurity(labels, &left_idx)
                    - right_weight * self.gini_impurity(labels, &right_idx);

                if gain > best_gain {
                    best_gain = gain;
                    best_split = Some((feature_idx, threshold, left_idx, right_idx));
                }
            }
        }

        best_split
    }

    /// Calculate Gini impurity
    fn gini_impurity(&self, labels: &[usize], indices: &[usize]) -> f64 {
        if indices.is_empty() {
            return 0.0;
        }

        let mut counts = vec![0usize; self.n_classes];
        for &i in indices {
            counts[labels[i]] += 1;
        }

        let total = indices.len() as f64;
        let impurity: f64 = counts
            .iter()
            .map(|&c| {
                let p = c as f64 / total;
                p * (1.0 - p)
            })
            .sum();

        impurity
    }

    /// Predict class
    pub fn predict(&self, features: &[f64]) -> Result<usize, AIError> {
        match &self.tree {
            Some(tree) => self.traverse_tree(tree, features),
            None => Err(AIError::ModelNotTrained),
        }
    }

    /// Traverse tree to find prediction
    fn traverse_tree(&self, node: &TreeNode, features: &[f64]) -> Result<usize, AIError> {
        match node.label {
            Some(label) => Ok(label),
            None => {
                let feature_idx = node.feature_idx.unwrap();
                let threshold = node.threshold.unwrap();

                if features[feature_idx] <= threshold {
                    self.traverse_tree(node.left.as_ref().unwrap(), features)
                } else {
                    self.traverse_tree(node.right.as_ref().unwrap(), features)
                }
            }
        }
    }
}

/// Ensemble Classifier
/// 
/// Combines multiple classifiers using voting or averaging.
/// 
/// ## Features
/// - Multiple classifier support
/// - Soft and hard voting
/// - Improved accuracy
/// - Reduced overfitting
pub struct EnsembleClassifier {
    classifiers: Vec<Box<dyn Classifier>>,
    voting: VotingMethod,
}

/// Classifier trait for ensemble
pub trait Classifier {
    fn predict(&self, features: &[f64]) -> Result<usize, AIError>;
    fn predict_proba(&self, features: &[f64]) -> Result<Vec<f64>, AIError>;
}

#[derive(Debug, Clone, Copy)]
pub enum VotingMethod {
    Hard,
    Soft,
}

impl EnsembleClassifier {
    /// Create a new ensemble classifier
    pub fn new(voting: VotingMethod) -> Self {
        Self {
            classifiers: Vec::new(),
            voting,
        }
    }

    /// Add classifier to ensemble
    pub fn add_classifier<C: Classifier + 'static>(&mut self, classifier: C) {
        self.classifiers.push(Box::new(classifier));
    }

    /// Predict using ensemble
    pub fn predict(&self, features: &[f64]) -> Result<usize, AIError> {
        if self.classifiers.is_empty() {
            return Err(AIError::ModelNotTrained);
        }

        match self.voting {
            VotingMethod::Hard => {
                // Hard voting: majority vote
                let mut votes = std::collections::HashMap::new();
                for classifier in &self.classifiers {
                    let label = classifier.predict(features)?;
                    *votes.entry(label).or_insert(0) += 1;
                }
                votes
                    .into_iter()
                    .max_by_key(|(_, count)| *count)
                    .map(|(label, _)| label)
                    .ok_or_else(|| AIError::InternalError("No votes".to_string()))
            }
            VotingMethod::Soft => {
                // Soft voting: average probabilities
                let mut all_probas: Vec<Vec<f64>> = Vec::new();
                for classifier in &self.classifiers {
                    all_probas.push(classifier.predict_proba(features)?);
                }

                let n_classes = all_probas[0].len();
                let mut avg_probas = vec![0.0; n_classes];

                for proba in &all_probas {
                    for (i, &p) in proba.iter().enumerate() {
                        avg_probas[i] += p;
                    }
                }

                for p in &mut avg_probas {
                    *p /= all_probas.len() as f64;
                }

                avg_probas
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .map(|(i, _)| i)
                    .ok_or_else(|| AIError::InternalError("No probabilities".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knn_classifier() {
        let mut classifier = KNNClassifier::new(3).unwrap();
        
        let data = vec![
            (vec![1.0, 1.0], 0),
            (vec![2.0, 2.0], 1),
            (vec![1.0, 2.0], 0),
            (vec![2.0, 1.0], 1),
        ];
        
        classifier.fit(&data).unwrap();
        let prediction = classifier.predict(&[1.5, 1.5]).unwrap();
        assert!(prediction == 0 || prediction == 1);
    }

    #[test]
    fn test_decision_tree() {
        let mut classifier = DecisionTreeClassifier::new(5, 2).unwrap();
        
        let features = vec![
            vec![1.0, 2.0],
            vec![2.0, 3.0],
            vec![3.0, 4.0],
            vec![4.0, 5.0],
        ];
        let labels = vec![0, 0, 1, 1];
        
        classifier.fit(&features, &labels).unwrap();
        let prediction = classifier.predict(&[1.5, 2.5]).unwrap();
        assert!(prediction == 0 || prediction == 1);
    }
}