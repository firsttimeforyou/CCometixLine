use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct DirectorySegment;

impl DirectorySegment {
    pub fn new() -> Self {
        Self
    }

    /// Extract directory name from path, handling both Unix and Windows separators
    fn extract_directory_name(path: &str) -> String {
        // Handle both Unix and Windows separators by trying both
        let unix_name = path.split('/').next_back().unwrap_or("");
        let windows_name = path.split('\\').next_back().unwrap_or("");

        // Choose the name that indicates actual path splitting occurred
        let result = if windows_name.len() < path.len() {
            // Windows path separator was found
            windows_name
        } else if unix_name.len() < path.len() {
            // Unix path separator was found
            unix_name
        } else {
            // No separator found, use the whole path
            path
        };

        if result.is_empty() {
            "root".to_string()
        } else {
            result.to_string()
        }
    }

    /// Extract the second folder name from a path
    /// For Windows: D:\javaCode\zm\d3 -> "zm" (skip drive letter)
    /// For Unix: /home/user/project -> "user" (skip leading empty string)
    fn extract_secondary_folder(path: &str) -> String {
        // Determine path type and split
        let is_windows = path.contains('\\');
        let parts: Vec<&str> = if is_windows {
            path.split('\\').collect()
        } else {
            path.split('/').collect()
        };

        // For Windows paths, skip the drive letter (e.g., "D:")
        // For Unix paths, skip the leading empty string
        let start_index = if is_windows {
            // Check if first part is a drive letter (ends with ':')
            if parts.first().map(|p| p.ends_with(':')).unwrap_or(false) {
                2 // Skip drive letter and first folder
            } else {
                1 // Just skip first folder
            }
        } else {
            // Unix: skip leading empty string and first folder
            if parts.first().map(|p| p.is_empty()).unwrap_or(false) {
                2
            } else {
                1
            }
        };

        // Get the second folder if it exists
        if start_index < parts.len() {
            parts[start_index].to_string()
        } else {
            String::new()
        }
    }
}

impl Segment for DirectorySegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let current_dir = &input.workspace.current_dir;

        // Handle cross-platform path separators manually for better compatibility
        let dir_name = Self::extract_directory_name(current_dir);
        let proj_name = Self::extract_secondary_folder(current_dir);

        // Store the full path in metadata for potential use
        let mut metadata = HashMap::new();
        metadata.insert("full_path".to_string(), current_dir.clone());

        Some(SegmentData {
            primary: dir_name,
            secondary: proj_name,
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Directory
    }
}
