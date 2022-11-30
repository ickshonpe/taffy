use taffy::node::MeasureFunc;

fn main() {
    let mut taffy = taffy::node::Taffy::new();
    let child = taffy
        .new_leaf_with_measure(
            taffy::style::Style {
                //size: taffy::geometry::Size { height: taffy::style::Dimension::Points(50.0), ..Default::default() },
                size_constraints: taffy::geometry::Size::suggested_from_height(taffy::style::Dimension::Points(50.0)),
                ..Default::default()
            },
            MeasureFunc::Raw(|known_dimensions, _available_space| taffy::geometry::Size {
                width: known_dimensions.width.unwrap_or(100.0),
                height: known_dimensions.height.unwrap_or(100.0),
            }),
        )
        .unwrap();

    let node = taffy.new_with_children(taffy::style::Style { ..Default::default() }, &[child]).unwrap();
    taffy.compute_layout(node, taffy::geometry::Size::MAX_CONTENT).unwrap();

    assert_eq!(taffy.layout(child).unwrap().size.width, 100.0);
    assert_eq!(taffy.layout(child).unwrap().size.height, 50.0);
}
