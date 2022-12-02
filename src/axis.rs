use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum Axis {
    Row,
    Column,
}

impl Axis {
    pub fn cross(self) -> Self {
        match self {
            Self::Row => Self::Column,
            Self::Column => Self::Row,
        }
    }
}

impl From<FlexDirection> for Axis {
    fn from(direction: FlexDirection) -> Self {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => Self::Row,
            FlexDirection::Column | FlexDirection::ColumnReverse => Self::Column,
        }
    }
}

pub struct Extent<'a> {
    style: &'a Style,
    axis: Axis,
}


//  pub trait Extent {
//     pub fn display(&self) -> Display;
//     pub fn position_type(&self) -> PositionType;
//     pub fn flex_direction(&self) -> FlexDirection;
//     pub fn flex_wrap(&self) -> FlexWrap;
//     pub fn align_items(&self) -> AlignItems;
//     pub fn align_self(&self) -> AlignSelf;
//     pub fn align_content(&self) -> AlignContent;
//     pub fn justify_content(&self) -> JustifyContent;
//     pub fn position(&self) -> Rect<LengthPercentageAuto>;
//     pub fn margin_min(&self) -> LengthPercentageAuto;
//     pub fn margin_max(&self) -> LengthPercentageAuto;
//     pub fn padding_min(&self) -> LengthPercentage;
//     pub fn padding_max(&self) -> LengthPercentage;
//     pub fn border_min(&self) -> LengthPercentage;
//     pub fn border_max(&self) -> LengthPercentage;
//     pub fn gap(&self) -> LengthPercentage;
//     pub fn flex_grow(&self) -> f32;
//     pub fn flex_shrink(&self) -> f32;
//     pub fn flex_basis(&self) -> Dimension;
//     pub fn size(&self) -> Dimension;
//     pub fn min_size(&self) -> Dimension;
//     pub fn max_size(&self) -> Dimension;
//     pub fn aspect_ratio(&self) -> Option<f32>;
// }

impl Style {
    pub fn axis(&self, axis: impl Into<Axis>) -> Extent {
        Extent {
            style: self,
            axis: axis.into(),
        }
    }

    pub fn cross(&self, axis: impl Into<Axis>) -> Extent {
        Extent {
            style: self,
            axis: axis.into().cross(),
        }
    }
}

impl <T> Rect<T> {
    pub fn start(self, axis: impl Into<Axis>) -> T {
        match axis.into() {
            Axis::Row => self.left,
            Axis::Column => self.top,
        }
    }

    pub fn end(self, axis: impl Into<Axis>) -> T {
        match axis.into() {
            Axis::Row => self.bottom,
            Axis::Column => self.right,
        }
    }
}

impl <T> Size<T> where T: Copy {
    pub fn axis(&self, axis: impl Into<Axis>) -> T {
        match axis.into() {
            Axis::Row => self.width,
            Axis::Column => self.height,
        }
    }
}

impl <'a> Extent<'a> {
    pub fn display(&self) -> Display {
        self.style.display
    }

    pub fn position_type(&self) -> PositionType {
        self.style.position_type
    }

    pub fn flex_direction(&self) -> FlexDirection {
        self.style.flex_direction
    }

    pub fn flex_wrap(&self) -> FlexWrap {
        self.style.flex_wrap
    }

    pub fn align_items(&self) -> AlignItems {
        self.style.align_items
    }

    pub fn align_self(&self) -> AlignSelf {
        self.style.align_self
    }

    pub fn align_content(&self) -> AlignContent {
        self.style.align_content
    }

    pub fn justify_content(&self) -> JustifyContent {
        self.style.justify_content
    }

    pub fn position(&self) -> Rect<LengthPercentageAuto> {
        self.style.position
    }

    pub fn margin_start(&self) -> LengthPercentageAuto {
        self.style.margin.start(self.axis)
    }

    pub fn margin_end(&self) -> LengthPercentageAuto {
        self.style.margin.end(self.axis)
    }

    pub fn padding_start(&self) -> LengthPercentage {
        self.style.padding.start(self.axis)
    }

    pub fn padding_end(&self) -> LengthPercentage {
        self.style.padding.end(self.axis)
    }

    pub fn border_start(&self) -> LengthPercentage {
        self.style.border.start(self.axis)
    }

    pub fn border_max(&self) -> LengthPercentage {
        self.style.border.end(self.axis)
    }

    pub fn gap(&self) -> LengthPercentage {
        self.style.gap.axis(self.axis)
    }

    pub fn flex_grow(&self) -> f32 {
        self.style.flex_grow
    }

    pub fn flex_shrink(&self) -> f32 {
        self.style.flex_shrink
    }

    pub fn flex_basis(&self) -> Dimension {
        self.style.flex_basis
    }

    pub fn size(&self) -> Dimension {
        self.style.size.axis(self.axis)
    }

    pub fn min_size(&self) -> Dimension {
        self.style.min_size.axis(self.axis)
    }

    pub fn max_size(&self) -> Dimension {
        self.style.max_size.axis(self.axis)
    }

    pub fn aspect_ratio(&self) -> Option<f32> {
        self.style.aspect_ratio
    }
}