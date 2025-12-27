//! Split pane management
//!
//! Handles horizontal and vertical terminal splits.

#![allow(dead_code)]

use eframe::egui;

/// Direction of a split
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

/// A pane in the split layout
#[derive(Clone, Debug)]
pub struct Pane {
    /// Unique pane ID
    pub id: usize,
    /// Size ratio (0.0 - 1.0) relative to parent
    pub ratio: f32,
    /// Whether this pane is focused
    pub focused: bool,
}

/// Split layout tree node
#[derive(Clone, Debug)]
pub enum SplitNode {
    /// A leaf node containing a single pane
    Pane(Pane),
    /// A split containing two child nodes
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<SplitNode>,
        second: Box<SplitNode>,
    },
}

/// Split pane manager
pub struct SplitManager {
    /// Root of the split tree
    root: SplitNode,
    /// Next pane ID
    next_id: usize,
    /// Currently focused pane ID
    focused_pane: usize,
    /// Minimum pane size in pixels
    min_pane_size: f32,
    /// Whether broadcast mode is active (input goes to all panes)
    broadcast_mode: bool,
}

impl SplitManager {
    /// Create a new split manager with a single pane
    pub fn new() -> Self {
        Self {
            root: SplitNode::Pane(Pane {
                id: 0,
                ratio: 1.0,
                focused: true,
            }),
            next_id: 1,
            focused_pane: 0,
            min_pane_size: 100.0,
            broadcast_mode: false,
        }
    }

    /// Split the currently focused pane
    pub fn split(&mut self, direction: SplitDirection) -> usize {
        let new_id = self.next_id;
        self.next_id += 1;

        // Find and split the focused pane
        self.root = self.split_node(self.root.clone(), self.focused_pane, direction, new_id);

        new_id
    }

    /// Split a specific node
    fn split_node(&self, node: SplitNode, target_id: usize, direction: SplitDirection, new_id: usize) -> SplitNode {
        match node {
            SplitNode::Pane(pane) if pane.id == target_id => {
                SplitNode::Split {
                    direction,
                    ratio: 0.5,
                    first: Box::new(SplitNode::Pane(Pane {
                        id: pane.id,
                        ratio: 1.0,
                        focused: false,
                    })),
                    second: Box::new(SplitNode::Pane(Pane {
                        id: new_id,
                        ratio: 1.0,
                        focused: true,
                    })),
                }
            }
            SplitNode::Split { direction: dir, ratio, first, second } => {
                SplitNode::Split {
                    direction: dir,
                    ratio,
                    first: Box::new(self.split_node(*first, target_id, direction, new_id)),
                    second: Box::new(self.split_node(*second, target_id, direction, new_id)),
                }
            }
            other => other,
        }
    }

    /// Close a pane by ID
    pub fn close_pane(&mut self, pane_id: usize) -> bool {
        if self.pane_count() <= 1 {
            return false; // Can't close the last pane
        }

        self.root = self.remove_pane(self.root.clone(), pane_id);

        // If we closed the focused pane, focus another
        if self.focused_pane == pane_id {
            if let Some(id) = self.first_pane_id() {
                self.focus_pane(id);
            }
        }

        true
    }

    /// Remove a pane from the tree
    fn remove_pane(&self, node: SplitNode, pane_id: usize) -> SplitNode {
        match node {
            SplitNode::Split { direction, ratio, first, second } => {
                // Check if either child is the pane to remove
                match (first.as_ref(), second.as_ref()) {
                    (SplitNode::Pane(p), other) if p.id == pane_id => other.clone(),
                    (other, SplitNode::Pane(p)) if p.id == pane_id => other.clone(),
                    _ => {
                        SplitNode::Split {
                            direction,
                            ratio,
                            first: Box::new(self.remove_pane(*first, pane_id)),
                            second: Box::new(self.remove_pane(*second, pane_id)),
                        }
                    }
                }
            }
            other => other,
        }
    }

    /// Focus a specific pane
    pub fn focus_pane(&mut self, pane_id: usize) {
        self.focused_pane = pane_id;
        self.root = self.set_focus(self.root.clone(), pane_id);
    }

    /// Set focus in the tree
    fn set_focus(&self, node: SplitNode, target_id: usize) -> SplitNode {
        match node {
            SplitNode::Pane(mut pane) => {
                pane.focused = pane.id == target_id;
                SplitNode::Pane(pane)
            }
            SplitNode::Split { direction, ratio, first, second } => {
                SplitNode::Split {
                    direction,
                    ratio,
                    first: Box::new(self.set_focus(*first, target_id)),
                    second: Box::new(self.set_focus(*second, target_id)),
                }
            }
        }
    }

    /// Focus the next pane
    pub fn focus_next(&mut self) {
        let panes = self.all_pane_ids();
        if let Some(pos) = panes.iter().position(|&id| id == self.focused_pane) {
            let next = (pos + 1) % panes.len();
            self.focus_pane(panes[next]);
        }
    }

    /// Focus the previous pane
    pub fn focus_prev(&mut self) {
        let panes = self.all_pane_ids();
        if let Some(pos) = panes.iter().position(|&id| id == self.focused_pane) {
            let prev = if pos == 0 { panes.len() - 1 } else { pos - 1 };
            self.focus_pane(panes[prev]);
        }
    }

    /// Resize the split containing the focused pane
    pub fn resize(&mut self, delta: f32) {
        self.root = self.resize_split(self.root.clone(), self.focused_pane, delta);
    }

    /// Resize a split in the tree
    fn resize_split(&self, node: SplitNode, target_id: usize, delta: f32) -> SplitNode {
        match node {
            SplitNode::Split { direction, mut ratio, first, second } => {
                // Check if either child contains the target
                let first_contains = self.contains_pane(&first, target_id);

                if first_contains || self.contains_pane(&second, target_id) {
                    // Adjust ratio
                    if first_contains {
                        ratio = (ratio + delta).clamp(0.1, 0.9);
                    } else {
                        ratio = (ratio - delta).clamp(0.1, 0.9);
                    }
                }

                SplitNode::Split {
                    direction,
                    ratio,
                    first: Box::new(self.resize_split(*first, target_id, delta)),
                    second: Box::new(self.resize_split(*second, target_id, delta)),
                }
            }
            other => other,
        }
    }

    /// Check if a node contains a pane
    fn contains_pane(&self, node: &SplitNode, pane_id: usize) -> bool {
        match node {
            SplitNode::Pane(p) => p.id == pane_id,
            SplitNode::Split { first, second, .. } => {
                self.contains_pane(first, pane_id) || self.contains_pane(second, pane_id)
            }
        }
    }

    /// Get all pane IDs in order
    fn all_pane_ids(&self) -> Vec<usize> {
        let mut ids = Vec::new();
        self.collect_pane_ids(&self.root, &mut ids);
        ids
    }

    fn collect_pane_ids(&self, node: &SplitNode, ids: &mut Vec<usize>) {
        match node {
            SplitNode::Pane(p) => ids.push(p.id),
            SplitNode::Split { first, second, .. } => {
                self.collect_pane_ids(first, ids);
                self.collect_pane_ids(second, ids);
            }
        }
    }

    /// Get the first pane ID
    fn first_pane_id(&self) -> Option<usize> {
        self.all_pane_ids().first().copied()
    }

    /// Get the number of panes
    pub fn pane_count(&self) -> usize {
        self.all_pane_ids().len()
    }

    /// Get the focused pane ID
    pub fn focused_pane_id(&self) -> usize {
        self.focused_pane
    }

    /// Toggle broadcast mode
    pub fn toggle_broadcast(&mut self) {
        self.broadcast_mode = !self.broadcast_mode;
    }

    /// Check if broadcast mode is active
    pub fn is_broadcast_mode(&self) -> bool {
        self.broadcast_mode
    }

    /// Calculate layout rectangles for all panes
    pub fn calculate_layout(&self, available: egui::Rect) -> Vec<(usize, egui::Rect)> {
        let mut layouts = Vec::new();
        self.layout_node(&self.root, available, &mut layouts);
        layouts
    }

    fn layout_node(&self, node: &SplitNode, rect: egui::Rect, layouts: &mut Vec<(usize, egui::Rect)>) {
        match node {
            SplitNode::Pane(pane) => {
                layouts.push((pane.id, rect));
            }
            SplitNode::Split { direction, ratio, first, second } => {
                let (first_rect, second_rect) = match direction {
                    SplitDirection::Horizontal => {
                        let split_y = rect.top() + rect.height() * ratio;
                        (
                            egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), split_y)),
                            egui::Rect::from_min_max(egui::pos2(rect.left(), split_y), rect.max),
                        )
                    }
                    SplitDirection::Vertical => {
                        let split_x = rect.left() + rect.width() * ratio;
                        (
                            egui::Rect::from_min_max(rect.min, egui::pos2(split_x, rect.bottom())),
                            egui::Rect::from_min_max(egui::pos2(split_x, rect.top()), rect.max),
                        )
                    }
                };

                self.layout_node(first, first_rect, layouts);
                self.layout_node(second, second_rect, layouts);
            }
        }
    }
}

impl Default for SplitManager {
    fn default() -> Self {
        Self::new()
    }
}
