#!/bin/bash
# Script to add #[allow(dead_code)] to structs with unused fields

cd "$(dirname "$0")/../src/verified"

# Add allow(dead_code) to specific structs in direct_metal_vulkan.rs
sed -i '/^struct VulkanInstance {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanPhysicalDevice {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanDeviceProperties {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanQueueFamily {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanDevice {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanMemory {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanCommandBuffer {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanFence {$/i #[allow(dead_code)]' direct_metal_vulkan.rs
sed -i '/^struct VulkanPipeline {$/i #[allow(dead_code)]' direct_metal_vulkan.rs

# Add allow(dead_code) to specific structs in direct_metal_metal.rs
sed -i '/^struct MetalDeviceInfo {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^struct MetalDevice {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^struct MetalBuffer {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^enum MetalStorageMode {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^struct MetalCommandBuffer {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^enum MetalEncoder {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^struct MetalFence {$/i #[allow(dead_code)]' direct_metal_metal.rs
sed -i '/^struct MetalPipeline {$/i #[allow(dead_code)]' direct_metal_metal.rs

# Add allow(dead_code) to specific structs in direct_metal.rs
sed -i '/^pub struct CommandBuffer {$/i #[allow(dead_code)]' direct_metal.rs
sed -i '/^pub struct GpuFence {$/i #[allow(dead_code)]' direct_metal.rs
sed -i '/^pub struct GpuPipeline {$/i #[allow(dead_code)]' direct_metal.rs
sed -i '/^pub struct GpuScheduler {$/i #[allow(dead_code)]' direct_metal.rs

echo "✅ Added #[allow(dead_code)] attributes to GPU backend structs"