// VantisOS BVH Acceleration Structure
// Bounding Volume Hierarchy for efficient ray tracing

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};
use crate::ray_tracing::{Vec3, Ray, AABB, Triangle, RayTracingError};

/// BVH build configuration
#[derive(Debug, Clone, Copy)]
pub struct BVHBuildConfig {
    pub max_depth: u32,
    pub max_triangles_per_leaf: u32,
    pub split_method: SplitMethod,
    pub enable_spatial_splits: bool,
}

impl Default for BVHBuildConfig {
    fn default() -> Self {
        Self {
            max_depth: 20,
            max_triangles_per_leaf: 4,
            split_method: SplitMethod::SAH,
            enable_spatial_splits: false,
        }
    }
}

/// Split method for BVH construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitMethod {
    /// Split along the longest axis
    Middle,
    /// Split at the median
    Median,
    /// Surface Area Heuristic
    SAH,
    /// Equal counts
    EqualCounts,
}

/// BVH node
#[derive(Debug, Clone)]
pub enum BVHNode {
    /// Internal node with children
    Internal {
        bounds: AABB,
        left: Box<BVHNode>,
        right: Box<BVHNode>,
        split_axis: u32,
    },
    /// Leaf node with triangles
    Leaf {
        bounds: AABB,
        triangles: Vec<Triangle>,
    },
}

impl BVHNode {
    /// Get bounding box
    pub fn bounds(&self) -> AABB {
        match self {
            BVHNode::Internal { bounds, .. } => *bounds,
            BVHNode::Leaf { bounds, .. } => *bounds,
        }
    }

    /// Check if node is leaf
    pub fn is_leaf(&self) -> bool {
        matches!(self, BVHNode::Leaf { .. })
    }

    /// Get triangle count
    pub fn triangle_count(&self) -> usize {
        match self {
            BVHNode::Internal { left, right, .. } => {
                left.triangle_count() + right.triangle_count()
            }
            BVHNode::Leaf { triangles, .. } => triangles.len(),
        }
    }

    /// Intersect ray with BVH node
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        if !self.bounds().intersects(ray) {
            return None;
        }

        match self {
            BVHNode::Leaf { triangles, .. } => {
                triangles
                    .iter()
                    .filter_map(|tri| tri.intersects(ray))
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
            }
            BVHNode::Internal { left, right, .. } => {
                let left_hit = left.intersect(ray);
                let right_hit = right.intersect(ray);

                match (left_hit, right_hit) {
                    (Some(l), Some(r)) => Some(l.min(r)),
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                }
            }
        }
    }

    /// Build BVH from triangles
    pub fn build(triangles: Vec<Triangle>, config: &BVHBuildConfig) -> Option<Box<BVHNode>> {
        if triangles.is_empty() {
            return None;
        }

        Self::build_recursive(triangles, config, 0)
    }

    /// Recursive BVH building
    fn build_recursive(triangles: Vec<Triangle>, config: &BVHBuildConfig, depth: u32) 
        -> Option<Box<BVHNode>> {
        if triangles.is_empty() {
            return None;
        }

        // Check if we should create a leaf
        if triangles.len() <= config.max_triangles_per_leaf as usize || depth >= config.max_depth {
            let bounds = triangles.iter().fold(AABB::empty(), |acc, tri| {
                acc.union(&tri.bounding_box())
            });

            return Some(Box::new(BVHNode::Leaf {
                bounds,
                triangles,
            }));
        }

        // Split triangles
        let (left_triangles, right_triangles, split_axis) = match config.split_method {
            SplitMethod::SAH => Self::split_sah(&triangles),
            SplitMethod::Middle => Self::split_middle(&triangles),
            SplitMethod::Median => Self::split_median(&triangles),
            SplitMethod::EqualCounts => Self::split_equal_counts(&triangles),
        };

        // Build children
        let left = Self::build_recursive(left_triangles, config, depth + 1)?;
        let right = Self::build_recursive(right_triangles, config, depth + 1)?;

        // Calculate combined bounds
        let bounds = left.bounds().union(&right.bounds());

        Some(Box::new(BVHNode::Internal {
            bounds,
            left,
            right,
            split_axis,
        }))
    }

    /// Split using Surface Area Heuristic
    fn split_sah(triangles: &[Triangle]) -> (Vec<Triangle>, Vec<Triangle>, u32) {
        if triangles.len() <= 1 {
            return (triangles.to_vec(), Vec::new(), 0);
        }

        let bounds = triangles.iter().fold(AABB::empty(), |acc, tri| {
            acc.union(&tri.bounding_box())
        });

        let extent = bounds.max.sub(&bounds.min);
        let split_axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        // Sort triangles by centroid
        let mut sorted = triangles.to_vec();
        sorted.sort_by(|a, b| {
            let (a_center, b_center) = if split_axis == 0 {
                ((a.v0.x + a.v1.x + a.v2.x) / 3.0, (b.v0.x + b.v1.x + b.v2.x) / 3.0)
            } else if split_axis == 1 {
                ((a.v0.y + a.v1.y + a.v2.y) / 3.0, (b.v0.y + b.v1.y + b.v2.y) / 3.0)
            } else {
                ((a.v0.z + a.v1.z + a.v2.z) / 3.0, (b.v0.z + b.v1.z + b.v2.z) / 3.0)
            };

            a_center.partial_cmp(&b_center).unwrap_or(core::cmp::Ordering::Equal)
        });

        // Find best split using SAH
        let mut best_cost = f32::MAX;
        let mut best_split = 1;

        let surface_area = bounds.surface_area();

        for split in 1..sorted.len() {
            let left_bounds = sorted[..split].iter().fold(AABB::empty(), |acc, tri| {
                acc.union(&tri.bounding_box())
            });
            let right_bounds = sorted[split..].iter().fold(AABB::empty(), |acc, tri| {
                acc.union(&tri.bounding_box())
            });

            let left_cost = (split as f32) * left_bounds.surface_area();
            let right_cost = ((sorted.len() - split) as f32) * right_bounds.surface_area();
            let total_cost = left_cost + right_cost;

            if total_cost < best_cost {
                best_cost = total_cost;
                best_split = split;
            }
        }

        (sorted[..best_split].to_vec(), sorted[best_split..].to_vec(), split_axis)
    }

    /// Split at middle of bounds
    fn split_middle(triangles: &[Triangle]) -> (Vec<Triangle>, Vec<Triangle>, u32) {
        let bounds = triangles.iter().fold(AABB::empty(), |acc, tri| {
            acc.union(&tri.bounding_box())
        });

        let extent = bounds.max.sub(&bounds.min);
        let split_axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        let split_pos = if split_axis == 0 {
            (bounds.min.x + bounds.max.x) / 2.0
        } else if split_axis == 1 {
            (bounds.min.y + bounds.max.y) / 2.0
        } else {
            (bounds.min.z + bounds.max.z) / 2.0
        };

        let mut left = Vec::new();
        let mut right = Vec::new();

        for tri in triangles {
            let center = if split_axis == 0 {
                (tri.v0.x + tri.v1.x + tri.v2.x) / 3.0
            } else if split_axis == 1 {
                (tri.v0.y + tri.v1.y + tri.v2.y) / 3.0
            } else {
                (tri.v0.z + tri.v1.z + tri.v2.z) / 3.0
            };

            if center < split_pos {
                left.push(*tri);
            } else {
                right.push(*tri);
            }
        }

        (left, right, split_axis)
    }

    /// Split at median
    fn split_median(triangles: &[Triangle]) -> (Vec<Triangle>, Vec<Triangle>, u32) {
        let bounds = triangles.iter().fold(AABB::empty(), |acc, tri| {
            acc.union(&tri.bounding_box())
        });

        let extent = bounds.max.sub(&bounds.min);
        let split_axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        let mut sorted = triangles.to_vec();
        sorted.sort_by(|a, b| {
            let (a_center, b_center) = if split_axis == 0 {
                ((a.v0.x + a.v1.x + a.v2.x) / 3.0, (b.v0.x + b.v1.x + b.v2.x) / 3.0)
            } else if split_axis == 1 {
                ((a.v0.y + a.v1.y + a.v2.y) / 3.0, (b.v0.y + b.v1.y + b.v2.y) / 3.0)
            } else {
                ((a.v0.z + a.v1.z + a.v2.z) / 3.0, (b.v0.z + b.v1.z + b.v2.z) / 3.0)
            };

            a_center.partial_cmp(&b_center).unwrap_or(core::cmp::Ordering::Equal)
        });

        let mid = sorted.len() / 2;
        (sorted[..mid].to_vec(), sorted[mid..].to_vec(), split_axis)
    }

    /// Split into equal counts
    fn split_equal_counts(triangles: &[Triangle]) -> (Vec<Triangle>, Vec<Triangle>, u32) {
        let bounds = triangles.iter().fold(AABB::empty(), |acc, tri| {
            acc.union(&tri.bounding_box())
        });

        let extent = bounds.max.sub(&bounds.min);
        let split_axis = if extent.x > extent.y && extent.x > extent.z {
            0
        } else if extent.y > extent.z {
            1
        } else {
            2
        };

        let mut sorted = triangles.to_vec();
        sorted.sort_by(|a, b| {
            let (a_center, b_center) = if split_axis == 0 {
                ((a.v0.x + a.v1.x + a.v2.x) / 3.0, (b.v0.x + b.v1.x + b.v2.x) / 3.0)
            } else if split_axis == 1 {
                ((a.v0.y + a.v1.y + a.v2.y) / 3.0, (b.v0.y + b.v1.y + b.v2.y) / 3.0)
            } else {
                ((a.v0.z + a.v1.z + a.v2.z) / 3.0, (b.v0.z + b.v1.z + b.v2.z) / 3.0)
            };

            a_center.partial_cmp(&b_center).unwrap_or(core::cmp::Ordering::Equal)
        });

        let mid = sorted.len() / 2;
        (sorted[..mid].to_vec(), sorted[mid..].to_vec(), split_axis)
    }

    /// Get BVH statistics
    pub fn get_stats(&self) -> BVHStats {
        match self {
            BVHNode::Internal { left, right, .. } => {
                let left_stats = left.get_stats();
                let right_stats = right.get_stats();
                BVHStats {
                    node_count: 1 + left_stats.node_count + right_stats.node_count,
                    leaf_count: left_stats.leaf_count + right_stats.leaf_count,
                    triangle_count: left_stats.triangle_count + right_stats.triangle_count,
                    max_depth: 1 + left_stats.max_depth.max(right_stats.max_depth),
                }
            }
            BVHNode::Leaf { triangles, .. } => BVHStats {
                node_count: 1,
                leaf_count: 1,
                triangle_count: triangles.len(),
                max_depth: 1,
            },
        }
    }
}

/// BVH statistics
#[derive(Debug, Clone, Copy)]
pub struct BVHStats {
    pub node_count: usize,
    pub leaf_count: usize,
    pub triangle_count: usize,
    pub max_depth: usize,
}

/// BVH builder
#[derive(Debug)]
pub struct BVHBuilder {
    config: BVHBuildConfig,
}

impl BVHBuilder {
    pub fn new(config: BVHBuildConfig) -> Self {
        Self { config }
    }

    pub fn build(&self, triangles: Vec<Triangle>) -> Result<Option<Box<BVHNode>>, RayTracingError> {
        Ok(BVHNode::build(triangles, &self.config))
    }
}

impl Default for BVHBuilder {
    fn default() -> Self {
        Self::new(BVHBuildConfig::default())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_build_config_default() {
        let config = BVHBuildConfig::default();
        assert_eq!(config.max_depth, 20);
        assert_eq!(config.max_triangles_per_leaf, 4);
        assert_eq!(config.split_method, SplitMethod::SAH);
    }

    #[test]
    fn test_bvh_build_simple() {
        let triangles = vec![
            Triangle::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3::new(2.0, 0.0, 0.0),
                Vec3::new(3.0, 0.0, 0.0),
                Vec3::new(2.0, 1.0, 0.0),
            ),
        ];

        let config = BVHBuildConfig::default();
        let bvh = BVHNode::build(triangles, &config);
        assert!(bvh.is_some());
    }

    #[test]
    fn test_bvh_intersection() {
        let triangles = vec![Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )];

        let config = BVHBuildConfig::default();
        let bvh = BVHNode::build(triangles, &config).unwrap();

        let ray = Ray::new(Vec3::new(0.3, 0.3, -1.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(bvh.intersect(&ray).is_some());

        let ray_miss = Ray::new(Vec3::new(2.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        assert!(bvh.intersect(&ray_miss).is_none());
    }

    #[test]
    fn test_bvh_stats() {
        let triangles = vec![
            Triangle::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3::new(2.0, 0.0, 0.0),
                Vec3::new(3.0, 0.0, 0.0),
                Vec3::new(2.0, 1.0, 0.0),
            ),
        ];

        let config = BVHBuildConfig::default();
        let bvh = BVHNode::build(triangles, &config).unwrap();
        let stats = bvh.get_stats();

        assert_eq!(stats.triangle_count, 2);
        assert!(stats.node_count > 0);
    }

    #[test]
    fn test_bvh_builder() {
        let builder = BVHBuilder::default();
        let triangles = vec![Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )];

        let result = builder.build(triangles);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}