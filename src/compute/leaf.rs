//! Computes size using styles and measure functions

use crate::geometry::Size;
use crate::layout::{AvailableSpace, RunMode, SizingMode};
use crate::math::ApplyConstraints;
use crate::node::Node;
use crate::resolve::{MaybeResolve, ResolveOrDefault};
use crate::style::Constraints;
use crate::tree::LayoutTree;

#[cfg(feature = "debug")]
use crate::debug::NODE_LOGGER;

/// Compute the size of a leaf node (node with no children)
pub(crate) fn compute(
    tree: &mut impl LayoutTree,
    node: Node,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<AvailableSpace>,
    _run_mode: RunMode,
    sizing_mode: SizingMode,
) -> Size<f32> {
    let style = tree.style(node);

    let node_constraints: Size<Constraints<Option<f32>>> = match sizing_mode {
        SizingMode::ContentSize => {
            Size {
                width: Constraints::suggested(known_dimensions.width),
                height: Constraints::suggested(known_dimensions.height)
            }
        }
        SizingMode::InherentSize => {
            let mut size = style.size_constraints.maybe_resolve(available_space.as_options());
            size.width.suggested = known_dimensions.width.or(size.width.suggested);
            size.height.suggested = known_dimensions.height.or(size.height.suggested);
            size
        }
    };

    #[cfg(feature = "debug")]
    NODE_LOGGER.log("LEAF");
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("node_size", node_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("min_size ", node_min_size);
    #[cfg(feature = "debug")]
    NODE_LOGGER.labelled_debug_log("max_size ", node_max_size);

    // Return early if both width and height are known
    if let Size { width: Some(width), height: Some(height) } = node_constraints.suggested() {
        return Size { width, height }.apply_clamp(node_constraints);
    };

    if tree.needs_measure(node) {
        // Compute available space
        let available_space = Size {
                width: available_space.width.maybe_set(node_constraints.suggested().width),
                height: available_space.height.maybe_set(node_constraints.suggested().height),
            };

        // Measure node
        let measured_size = tree.measure_node(node, known_dimensions, available_space);
        return node_constraints.suggested().unwrap_or(measured_size).apply_clamp(node_constraints);
    }

    let padding = style.padding.resolve_or_default(available_space.width.into_option());
    let border = style.border.resolve_or_default(available_space.width.into_option());

    Size {
        width: node_constraints.suggested()
            .width
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .apply_clamp(node_constraints.width),
        height: node_constraints.suggested()
            .height            
            .unwrap_or(0.0 + padding.horizontal_axis_sum() + border.horizontal_axis_sum()) // border-box
            .apply_clamp(node_constraints.height),
    }
}
