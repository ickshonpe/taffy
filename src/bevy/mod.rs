use bevy::prelude::*;
use crate::prelude::LayoutTree;


/// A tree of UI [`Nodes`](`Node`), suitable for UI layout
pub struct BevyTree {

}


// impl LayoutTree for BevyTree {
//     type ChildIter<'a> = core::slice::Iter<'a, Entity>;

//     fn children(&self, node: Node) -> Self::ChildIter<'_> {
//         self.children[node].iter()
//     }

//     fn child_count(&self, node: Node) -> usize {
//         self.children[node].len()
//     }

//     fn is_childless(&self, node: Node) -> bool {
//         self.children[node].is_empty()
//     }

//     fn parent(&self, node: Node) -> Option<Node> {
//         self.parents.get(node).copied().flatten()
//     }

//     fn style(&self, node: Node) -> &Style {
//         &self.nodes[node].style
//     }

//     fn layout(&self, node: Node) -> &Layout {
//         &self.nodes[node].layout
//     }

//     fn layout_mut(&mut self, node: Node) -> &mut Layout {
//         &mut self.nodes[node].layout
//     }

//     #[inline(always)]
//     fn mark_dirty(&mut self, node: Node) -> TaffyResult<()> {
//         self.mark_dirty_internal(node)
//     }

//     fn measure_node(
//         &self,
//         node: Node,
//         known_dimensions: Size<Option<f32>>,
//         available_space: Size<AvailableSpace>,
//     ) -> Size<f32> {
//         match &self.measure_funcs[node] {
//             MeasureFunc::Raw(measure) => measure(known_dimensions, available_space),

//             #[cfg(any(feature = "std", feature = "alloc"))]
//             MeasureFunc::Boxed(measure) => (measure as &dyn Fn(_, _) -> _)(known_dimensions, available_space),
//         }
//     }

//     fn needs_measure(&self, node: Node) -> bool {
//         self.nodes[node].needs_measure && self.measure_funcs.get(node).is_some()
//     }

//     fn cache_mut(&mut self, node: Node, index: usize) -> &mut Option<Cache> {
//         &mut self.nodes[node].size_cache[index]
//     }

//     fn child(&self, node: Node, id: usize) -> Node {
//         self.children[node][id]
//     }
// }

// #[allow(clippy::iter_cloned_collect)] // due to no-std support, we need to use `iter_cloned` instead of `collect`
// impl BevyTree {
//     /// Creates a new [`Taffy`]
//     ///
//     /// The default capacity of a [`Taffy`] is 16 nodes.
//     #[must_use]
//     pub fn new() -> Self {
//         Self::with_capacity(16)
//     }

//     /// Creates a new [`Taffy`] that can store `capacity` nodes before reallocation
//     #[must_use]
//     pub fn with_capacity(capacity: usize) -> Self {
//         Self {
//             // TODO: make this method const upstream,
//             // so constructors here can be const
//             nodes: SlotMap::with_capacity(capacity),
//             children: SlotMap::with_capacity(capacity),
//             parents: SlotMap::with_capacity(capacity),
//             measure_funcs: SparseSecondaryMap::with_capacity(capacity),
//             config: TaffyConfig::default(),
//         }
//     }

//     /// Enable rounding of layout values. Rounding is enabled by default.
//     pub fn enable_rounding(&mut self) {
//         self.config.use_rounding = true;
//     }

//     /// Disable rounding of layout values. Rounding is enabled by default.
//     pub fn disable_rounding(&mut self) {
//         self.config.use_rounding = false;
//     }

//     /// Creates and adds a new unattached leaf node to the tree, and returns the [`Node`] of the new node
//     pub fn new_leaf(&mut self, layout: Style) -> TaffyResult<Node> {
//         let id = self.nodes.insert(NodeData::new(layout));
//         let _ = self.children.insert(new_vec_with_capacity(0));
//         let _ = self.parents.insert(None);

//         Ok(id)
//     }

//     /// Creates and adds a new unattached leaf node to the tree, and returns the [`Node`] of the new node
//     ///
//     /// Creates and adds a new leaf node with a supplied [`MeasureFunc`]
//     pub fn new_leaf_with_measure(&mut self, layout: Style, measure: MeasureFunc) -> TaffyResult<Node> {
//         let mut data = NodeData::new(layout);
//         data.needs_measure = true;

//         let id = self.nodes.insert(data);
//         self.measure_funcs.insert(id, measure);

//         let _ = self.children.insert(new_vec_with_capacity(0));
//         let _ = self.parents.insert(None);

//         Ok(id)
//     }

//     /// Creates and adds a new node, which may have any number of `children`
//     pub fn new_with_children(&mut self, layout: Style, children: &[Node]) -> TaffyResult<Node> {
//         let id = self.nodes.insert(NodeData::new(layout));

//         for child in children {
//             self.parents[*child] = Some(id);
//         }

//         let _ = self.children.insert(children.iter().copied().collect::<_>());
//         let _ = self.parents.insert(None);

//         Ok(id)
//     }

//     /// Drops all nodes in the tree
//     pub fn clear(&mut self) {
//         self.nodes.clear();
//         self.children.clear();
//         self.parents.clear();
//     }

//     /// Remove a specific [`Node`] from the tree and drops it
//     ///
//     /// Returns the id of the node removed.
//     pub fn remove(&mut self, node: Node) -> TaffyResult<Node> {
//         if let Some(parent) = self.parents[node] {
//             if let Some(children) = self.children.get_mut(parent) {
//                 children.retain(|f| *f != node);
//             }
//         }

//         let _ = self.children.remove(node);
//         let _ = self.parents.remove(node);
//         let _ = self.nodes.remove(node);

//         Ok(node)
//     }

//     /// Sets the [`MeasureFunc`] of the associated node
//     pub fn set_measure(&mut self, node: Node, measure: Option<MeasureFunc>) -> TaffyResult<()> {
//         if let Some(measure) = measure {
//             self.nodes[node].needs_measure = true;
//             self.measure_funcs.insert(node, measure);
//         } else {
//             self.nodes[node].needs_measure = false;
//             self.measure_funcs.remove(node);
//         }

//         self.mark_dirty_internal(node)?;

//         Ok(())
//     }

//     /// Adds a `child` [`Node`] under the supplied `parent`
//     pub fn add_child(&mut self, parent: Node, child: Node) -> TaffyResult<()> {
//         self.parents[child] = Some(parent);
//         self.children[parent].push(child);
//         self.mark_dirty_internal(parent)?;

//         Ok(())
//     }

//     /// Directly sets the `children` of the supplied `parent`
//     pub fn set_children(&mut self, parent: Node, children: &[Node]) -> TaffyResult<()> {
//         // Remove node as parent from all its current children.
//         for child in &self.children[parent] {
//             self.parents[*child] = None;
//         }

//         // Build up relation node <-> child
//         for child in children {
//             self.parents[*child] = Some(parent);
//         }

//         self.children[parent] = children.iter().copied().collect::<_>();

//         self.mark_dirty_internal(parent)?;

//         Ok(())
//     }

//     /// Removes the `child` of the parent `node`
//     ///
//     /// The child is not removed from the tree entirely, it is simply no longer attached to its previous parent.
//     pub fn remove_child(&mut self, parent: Node, child: Node) -> TaffyResult<Node> {
//         let index = self.children[parent].iter().position(|n| *n == child).unwrap();
//         self.remove_child_at_index(parent, index)
//     }

//     /// Removes the child at the given `index` from the `parent`
//     ///
//     /// The child is not removed from the tree entirely, it is simply no longer attached to its previous parent.
//     pub fn remove_child_at_index(&mut self, parent: Node, child_index: usize) -> TaffyResult<Node> {
//         let child_count = self.children[parent].len();
//         if child_index >= child_count {
//             return Err(error::TaffyError::ChildIndexOutOfBounds { parent, child_index, child_count });
//         }

//         let child = self.children[parent].remove(child_index);
//         self.parents[child] = None;

//         self.mark_dirty_internal(parent)?;

//         Ok(child)
//     }

//     /// Replaces the child at the given `child_index` from the `parent` node with the new `child` node
//     ///
//     /// The child is not removed from the tree entirely, it is simply no longer attached to its previous parent.
//     pub fn replace_child_at_index(&mut self, parent: Node, child_index: usize, new_child: Node) -> TaffyResult<Node> {
//         let child_count = self.children[parent].len();
//         if child_index >= child_count {
//             return Err(error::TaffyError::ChildIndexOutOfBounds { parent, child_index, child_count });
//         }

//         self.parents[new_child] = Some(parent);
//         let old_child = core::mem::replace(&mut self.children[parent][child_index], new_child);
//         self.parents[old_child] = None;

//         self.mark_dirty_internal(parent)?;

//         Ok(old_child)
//     }

//     /// Returns the child [`Node`] of the parent `node` at the provided `child_index`
//     pub fn child_at_index(&self, parent: Node, child_index: usize) -> TaffyResult<Node> {
//         let child_count = self.children[parent].len();
//         if child_index >= child_count {
//             return Err(error::TaffyError::ChildIndexOutOfBounds { parent, child_index, child_count });
//         }

//         Ok(self.children[parent][child_index])
//     }

//     /// Returns the number of children of the `parent` [`Node`]
//     pub fn child_count(&self, parent: Node) -> TaffyResult<usize> {
//         Ok(self.children[parent].len())
//     }

//     /// Returns a list of children that belong to the parent [`Node`]
//     pub fn children(&self, parent: Node) -> TaffyResult<Vec<Node>> {
//         Ok(self.children[parent].iter().copied().collect::<_>())
//     }

//     /// Sets the [`Style`] of the provided `node`
//     pub fn set_style(&mut self, node: Node, style: Style) -> TaffyResult<()> {
//         self.nodes[node].style = style;
//         self.mark_dirty_internal(node)?;
//         Ok(())
//     }

//     /// Gets the [`Style`] of the provided `node`
//     pub fn style(&self, node: Node) -> TaffyResult<&Style> {
//         Ok(&self.nodes[node].style)
//     }

//     /// Return this node layout relative to its parent
//     pub fn layout(&self, node: Node) -> TaffyResult<&Layout> {
//         Ok(&self.nodes[node].layout)
//     }

//     /// Marks the layout computation of this node and its children as outdated
//     ///
//     /// Performs a recursive depth-first search up the tree until the root node is reached
//     ///
//     /// WARNING: this will stack-overflow if the tree contains a cycle
//     fn mark_dirty_internal(&mut self, node: Node) -> TaffyResult<()> {
//         /// WARNING: this will stack-overflow if the tree contains a cycle
//         fn mark_dirty_recursive(
//             nodes: &mut SlotMap<Node, NodeData>,
//             parents: &SlotMap<Node, Option<Node>>,
//             node_id: Node,
//         ) {
//             nodes[node_id].mark_dirty();

//             if let Some(Some(node)) = parents.get(node_id) {
//                 mark_dirty_recursive(nodes, parents, *node);
//             }
//         }

//         mark_dirty_recursive(&mut self.nodes, &self.parents, node);

//         Ok(())
//     }

//     /// Indicates whether the layout of this node (and its children) need to be recomputed
//     pub fn dirty(&self, node: Node) -> TaffyResult<bool> {
//         Ok(self.nodes[node].size_cache.iter().all(|entry| entry.is_none()))
//     }

//     /// Updates the stored layout of the provided `node` and its children
//     pub fn compute_layout(&mut self, node: Node, available_space: Size<AvailableSpace>) -> Result<(), TaffyError> {
//         crate::compute::compute_layout(self, node, available_space)
//     }
// }