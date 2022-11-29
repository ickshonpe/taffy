use taffy::prelude::*;
use taffy::style::Constraints;

fn main() {
    // println!("{:#?}", taffy::style::Style::DEFAULT);
    // println!("{:#?}", taffy::style::Style::default());

    let s: Size<Dimension> = Size::default();
    println!("{:?}", s);

    let s: Size<Constraints<Dimension>> = Size::default();
    println!("{:?}", s);

    let s: Size<Constraints<Dimension>> = Size::default();
    println!("{:?}", s);
}