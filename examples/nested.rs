use taffy::prelude::*;

fn main() -> Result<(), taffy::error::TaffyError> {
    let mut taffy = Taffy::new();

    // left
    let child_t1 = taffy.new_leaf(Style {
        size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) },
        ..Default::default()
    })?;

    let div1 = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t1],
    )?;

    // right
    let child_t2 = taffy.new_leaf(Style {
        size: Size { width: Dimension::Points(5.0), height: Dimension::Points(5.0) },
        ..Default::default()
    })?;

    let div2 = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::Percent(0.5), height: Dimension::Percent(1.0) },
            // justify_content: JustifyContent::Center,
            ..Default::default()
        },
        &[child_t2],
    )?;

    let container = taffy.new_with_children(
        Style { size: Size { width: Dimension::Percent(1.0), height: Dimension::Percent(1.0) }, ..Default::default() },
        &[div1, div2],
    )?;

    taffy.compute_layout(
        container,
        Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    )?;

    println!("node: {:#?}", taffy::node::TaffyWorld::layout(&taffy, container)?);

    println!("div1: {:#?}", taffy::node::TaffyWorld::layout(&taffy, div1)?);
    println!("div2: {:#?}", taffy::node::TaffyWorld::layout(&taffy, div2)?);

    println!("child1: {:#?}", taffy::node::TaffyWorld::layout(&taffy, child_t1)?);
    println!("child2: {:#?}", taffy::node::TaffyWorld::layout(&taffy, child_t2)?);

    Ok(())
}
