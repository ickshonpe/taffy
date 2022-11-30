//! Geometric primitives useful for layout

use crate::style::{Constraint, Constraints, Dimension, FlexDirection};
use core::ops::Add;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Width<T>(pub T);


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Height<T>(pub T);


pub enum Axis<T> {
    Height(T),
    Width(T),
}

impl<T> Clone for Axis<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Height(arg0) => Self::Height(arg0.clone()),
            Self::Width(arg0) => Self::Width(arg0.clone()),
        }
    }
}

pub trait TwoDimensional<T> {
    fn width(&self) -> Axis<T>;
    fn height(&self) -> Axis<T>;
}

impl<T> Copy for Axis<T> where T: Copy {}

impl<T> Axis<T> {
    pub fn has_dir(&self, direction: FlexDirection) -> bool {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => matches!(self, Self::Width(_)),
            FlexDirection::Column | FlexDirection::ColumnReverse => matches!(self, Self::Height(_)),
        }
    }
    pub fn from_dir(direction: FlexDirection, value: T) -> Axis<T> {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => Self::Width(value),
            FlexDirection::Column | FlexDirection::ColumnReverse => Self::Height(value),
        }
    }

    pub fn value(self) -> T {
        match self {
            Axis::Width(inner) => inner,
            Axis::Height(inner) => inner,
        }
    }

    pub fn with_inner<U>(self, f: impl Fn(T) -> U) -> Axis<U> {
        match self {
            Axis::Width(width) => Axis::Width(f(width)),
            Axis::Height(height) => Axis::Height(f(height)),
        }
    }

    pub fn with_size<U, V>(self, size: Size<U>, f: impl Fn(T, U) -> V) -> Axis<V> {
        match self {
            Axis::Width(width) => Axis::Width(f(width, size.width)),
            Axis::Height(height) => Axis::Height(f(height, size.height)),
        }
    }

    pub fn pair_size<U>(self, size: Size<U>) -> Axis<(T, U)> {
        match self {
            Axis::Width(width) => Axis::Width((width, size.width)),
            Axis::Height(height) => Axis::Height((height, size.height)),
        }
    }

    pub fn pair<U>(self, other: impl TwoDimensional<U>) -> Axis<(T, U)> {
        match self {
            Axis::Width(width) => Axis::Width((width, other.width().value())),
            Axis::Height(height) => Axis::Height((height, other.height().value())),
        }
    }
}

impl<T> Axis<Option<T>> {
    #[inline]
    pub fn unwrap_or(self, or: Axis<T>) -> Axis<T> {
        match self {
            Axis::Height(Some(t)) => Axis::Height(t),
            Axis::Width(Some(t)) => Axis::Width(t),
            _ => or,
        }
    }

    #[inline]
    pub fn unwrap_or_else(self, or: impl FnOnce() -> Axis<T>) -> Axis<T> {
        match self {
            Axis::Height(Some(t)) => Axis::Height(t),
            Axis::Width(Some(t)) => Axis::Width(t),
            _ => or(),
        }
    }

    #[inline]
    pub fn or(self, or: T) -> Axis<Option<T>> {
        match self {
            Axis::Height(Some(_)) | Axis::Width(Some(_)) => self,
            Axis::Height(None) => Axis::Height(Some(or)),
            Axis::Width(None) => Axis::Width(Some(or)),
        }
    }

    #[inline]
    pub fn or_else(self, or_else: impl FnOnce() -> T) -> Axis<Option<T>> {
        match self {
            Axis::Height(Some(_)) | Axis::Width(Some(_)) => self,
            Axis::Height(None) => Axis::Height(Some(or_else())),
            Axis::Width(None) => Axis::Width(Some(or_else())),
        }
    }
}

/// An axis-aligned UI rectangle
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Rect<T> {
    /// This can represent either the x-coordinate of the starting edge,
    /// or the amount of padding on the starting side.
    ///
    /// The starting edge is the left edge when working with LTR text,
    /// and the right edge when working with RTL text.
    pub left: T,
    /// This can represent either the x-coordinate of the ending edge,
    /// or the amount of padding on the ending side.
    ///
    /// The ending edge is the right edge when working with LTR text,
    /// and the left edge when working with RTL text.
    pub right: T,
    /// This can represent either the y-coordinate of the top edge,
    /// or the amount of padding on the top side.
    pub top: T,
    /// This can represent either the y-coordinate of the bottom edge,
    /// or the amount of padding on the bottom side.
    pub bottom: T,
}

impl<T> Rect<T> {
    /// Applies the function `f` to all four sides of the rect
    ///
    /// When applied to the left and right sides, the width is used
    /// as the second parameter of `f`.
    /// When applied to the top or bottom sides, the height is used instead.
    pub(crate) fn zip_size<R, F, U>(self, size: Size<U>, f: F) -> Rect<R>
    where
        F: Fn(T, U) -> R,
        U: Copy,
    {
        Rect {
            left: f(self.left, size.width),
            right: f(self.right, size.width),
            top: f(self.top, size.height),
            bottom: f(self.bottom, size.height),
        }
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    /// The sum of [`Rect.start`](Rect) and [`Rect.end`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the width of the rectangle.
    pub(crate) fn horizontal_axis_sum(&self) -> T {
        self.left + self.right
    }

    /// The sum of [`Rect.top`](Rect) and [`Rect.bottom`](Rect)
    ///
    /// This is typically used when computing total padding.
    ///
    /// **NOTE:** this is *not* the height of the rectangle.
    pub(crate) fn vertical_axis_sum(&self) -> T {
        self.top + self.bottom
    }

    /// The sum of the two fields of the [`Rect`] representing the main axis.
    ///
    /// This is typically used when computing total padding.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::horizontal`].
    /// Otherwise, this is [`Rect::vertical`].
    pub(crate) fn main_axis_sum(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.horizontal_axis_sum()
        } else {
            self.vertical_axis_sum()
        }
    }

    /// The sum of the two fields of the [`Rect`] representing the cross axis.
    ///
    /// If the [`FlexDirection`] is [`FlexDirection::Row`] or [`FlexDirection::RowReverse`], this is [`Rect::vertical`].
    /// Otherwise, this is [`Rect::horizontal`].
    pub(crate) fn cross_axis_sum(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.vertical_axis_sum()
        } else {
            self.horizontal_axis_sum()
        }
    }
}

impl<T> Rect<T>
where
    T: Copy + Clone,
{
    /// The `start` or `top` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn main_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.left
        } else {
            self.top
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn main_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.right
        } else {
            self.bottom
        }
    }

    /// The `start` or `top` value of the [`Rect`], from the perspective of the cross layout axis
    pub(crate) fn cross_start(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.top
        } else {
            self.left
        }
    }

    /// The `end` or `bottom` value of the [`Rect`], from the perspective of the main layout axis
    pub(crate) fn cross_end(&self, direction: FlexDirection) -> T {
        if direction.is_row() {
            self.bottom
        } else {
            self.right
        }
    }
}

impl Rect<f32> {
    /// Creates a new Rect with `0.0` as all parameters
    pub const ZERO: Rect<f32> = Self { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 };

    /// Creates a new Rect
    #[must_use]
    pub const fn new(start: f32, end: f32, top: f32, bottom: f32) -> Self {
        Self { left: start, right: end, top, bottom }
    }
}

pub struct AxisSummer<'a, T>(pub &'a Rect<T>)
where
    T: Add<Output = T> + Copy + Clone;

impl<'a, T> TwoDimensional<T> for AxisSummer<'a, T>
where
    T: Add<Output = T> + Copy + Clone,
{
    fn width(&self) -> Axis<T> {
        Axis::Width(self.0.horizontal_axis_sum())
    }

    fn height(&self) -> Axis<T> {
        Axis::Height(self.0.vertical_axis_sum())
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Copy + Clone,
{
    pub fn axis_sum(&self) -> AxisSummer<T> {
        AxisSummer(self)
    }
}

/// The width and height of a [`Rect`]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Size<T> {
    /// The x extent of the rectangle
    pub width: T,
    /// The y extent of the rectangle
    pub height: T,
}

impl<T> Size<T> {
    /// Applies the function `f` to both the width and height
    ///
    /// This is used to transform a `Size<T>` into a `Size<R>`.
    pub fn map<R, F>(self, f: F) -> Size<R>
    where
        F: Fn(T) -> R,
    {
        Size { width: f(self.width), height: f(self.height) }
    }

    /// Applies the function `f` to the width
    pub fn map_width<F>(self, f: F) -> Size<T>
    where
        F: Fn(T) -> T,
    {
        Size { width: f(self.width), height: self.height }
    }

    /// Applies the function `f` to the height
    pub fn map_height<F>(self, f: F) -> Size<T>
    where
        F: Fn(T) -> T,
    {
        Size { width: self.width, height: f(self.height) }
    }

    /// Applies the function `f` to both the width and height
    /// of this value and another passed value
    pub fn zip_map<Other, Ret, Func>(self, other: Size<Other>, f: Func) -> Size<Ret>
    where
        Func: Fn(T, Other) -> Ret,
    {
        Size { width: f(self.width, other.width), height: f(self.height, other.height) }
    }

    pub(crate) fn set(&mut self, value: Axis<T>) {
        match value {
            Axis::Width(width) => self.width = width,
            Axis::Height(height) => self.height = height,
        }
    }

    /// Sets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_main(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.width = value
        } else {
            self.height = value
        }
    }

    /// Sets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn set_cross(&mut self, direction: FlexDirection, value: T) {
        if direction.is_row() {
            self.height = value
        } else {
            self.width = value
        }
    }

    /// Gets the extent of the main layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn main(self, direction: FlexDirection) -> Axis<T> {
        if direction.is_row() {
            self.width()
        } else {
            self.height()
        }
    }

    /// Gets the extent of the cross layout axis
    ///
    /// Whether this is the width or height depends on the `direction` provided
    pub(crate) fn cross(self, direction: FlexDirection) -> Axis<T> {
        if direction.is_row() {
            self.height()
        } else {
            self.width()
        }
    }

    pub(crate) fn height(self) -> Axis<T> {
        Axis::Height(self.height)
    }

    pub(crate) fn width(self) -> Axis<T> {
        Axis::Width(self.width)
    }
}

impl Size<f32> {
    /// A [`Size`] with zero width and height
    pub const ZERO: Size<f32> = Self { width: 0.0, height: 0.0 };
}

impl Size<Option<f32>> {
    /// A [`Size`] with `None` width and height
    pub const NONE: Size<Option<f32>> = Self { width: None, height: None };

    /// A [`Size<Option<f32>>`] with `Some(width)` and `Some(height)` as parameters
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self {
        Size { width: Some(width), height: Some(height) }
    }

    /// Performs Option::unwrap_or on each component separately
    pub fn unwrap_or(self, alt: Size<f32>) -> Size<f32> {
        Size { width: self.width.unwrap_or(alt.width), height: self.height.unwrap_or(alt.height) }
    }

    /// Performs Option::or on each component separately
    pub fn or(self, alt: Size<Option<f32>>) -> Size<Option<f32>> {
        Size { width: self.width.or(alt.width), height: self.height.or(alt.height) }
    }
}

impl Size<Dimension> {
    /// Generates a [`Size<Dimension>`] using [`Dimension::Points`] values
    #[must_use]
    pub const fn from_points(width: f32, height: f32) -> Self {
        Size { width: Dimension::Points(width), height: Dimension::Points(height) }
    }

    /// Generates a [`Size<Dimension>`] using [`Dimension::Percent`] values
    #[must_use]
    pub const fn from_percent(width: f32, height: f32) -> Self {
        Size { width: Dimension::Percent(width), height: Dimension::Percent(height) }
    }

    /// Generates a [`Size<Dimension>`] using [`Dimension::Auto`] in both width and height
    pub const AUTO: Size<Dimension> = Self { width: Dimension::Auto, height: Dimension::Auto };

    /// Generates a [`Size<Dimension>`] using [`Dimension::Undefined`] in both width and height
    pub const UNDEFINED: Size<Dimension> = Self { width: Dimension::Undefined, height: Dimension::Undefined };
}

/// A 2-dimensional coordinate.
///
/// When used in association with a [`Rect`], represents the bottom-left corner.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point<T> {
    /// The x-coordinate
    pub x: T,
    /// The y-coordinate
    pub y: T,
}

impl Point<f32> {
    /// A [`Point`] with values (0,0), representing the origin
    pub const ZERO: Point<f32> = Self { x: 0.0, y: 0.0 };
}

impl Size<Constraints<Option<f32>>> {
    #[inline]
    pub fn get(&self, constraint: Constraint) -> Size<Option<f32>> {
        Size { width: self.width.get(constraint), height: self.height.get(constraint) }
    }

    #[inline]
    pub fn min(&self) -> Size<Option<f32>> {
        self.get(Constraint::Min)
    }

    #[inline]
    pub fn suggested(&self) -> Size<Option<f32>> {
        self.get(Constraint::Suggested)
    }

    #[inline]
    pub fn max(&self) -> Size<Option<f32>> {
        self.get(Constraint::Max)
    }
}

impl Size<Constraints<Dimension>> {
    pub const AUTO_CONSTRAINTS: Size<Constraints<Dimension>> =
        Self { width: Constraints::AUTO, height: Constraints::AUTO };
    pub const UNDEFINED_CONSTRAINTS: Size<Constraints<Dimension>> =
        Self { width: Constraints::UNDEFINED, height: Constraints::UNDEFINED };

    #[inline]
    pub fn min_from(min: Size<Dimension>) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::min(min.width), height: Constraints::min(min.height), ..Size::UNDEFINED_CONSTRAINTS }
    }
    #[inline]
    pub fn suggested_from(suggested: Size<Dimension>) -> Size<Constraints<Dimension>> {
        Size {
            width: Constraints::suggested(suggested.width),
            height: Constraints::suggested(suggested.height),
            ..Size::UNDEFINED_CONSTRAINTS
        }
    }
    #[inline]
    pub fn max_from(max: Size<Dimension>) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::max(max.width), height: Constraints::max(max.height), ..Size::UNDEFINED_CONSTRAINTS }
    }

    #[inline]
    pub fn min_from_points(width: f32, height: f32) -> Size<Constraints<Dimension>> {
        Size::min_from(Size::from_points(width, height))
    }

    #[inline]
    pub fn max_from_points(width: f32, height: f32) -> Size<Constraints<Dimension>> {
        Size::max_from(Size::from_points(width, height))
    }

    #[inline]
    pub fn suggested_from_points(width: f32, height: f32) -> Size<Constraints<Dimension>> {
        Size::suggested_from(Size::from_points(width, height))
    }

    #[inline]
    pub fn min_from_percent(width: f32, height: f32) -> Size<Constraints<Dimension>> {
        Size::min_from(Size::from_percent(width, height))
    }

    #[inline]
    pub fn max_from_percent(width: f32, height: f32) -> Size<Constraints<Dimension>> {
        Size::max_from(Size::from_percent(width, height))
    }

    #[inline]
    pub fn suggested_from_percent(width: f32, height: f32) -> Size<Constraints<Dimension>> {
        Size::suggested_from(Size::from_percent(width, height))
    }

    #[inline]
    pub fn get(&self, constraint: Constraint) -> Size<Dimension> {
        Size { width: self.width.get(constraint), height: self.height.get(constraint) }
    }

    #[inline]
    pub fn min(&self) -> Size<Dimension> {
        self.get(Constraint::Min)
    }

    #[inline]
    pub fn suggested(&self) -> Size<Dimension> {
        self.get(Constraint::Suggested)
    }

    #[inline]
    pub fn max(&self) -> Size<Dimension> {
        self.get(Constraint::Max)
    }

    pub const fn from_width(width: Constraints<Dimension>) -> Size<Constraints<Dimension>> {
        Size { width, ..Size::AUTO_CONSTRAINTS }
    }

    pub const fn from_height(height: Constraints<Dimension>) -> Size<Constraints<Dimension>> {
        Size { height, ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn constraint_from_width(constraint: Constraint, width: Dimension) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::from_constraint(constraint, width), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn constraint_from_height(constraint: Constraint, width: Dimension) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::from_constraint(constraint, width), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn min_from_width(width: Dimension) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::from_constraint(Constraint::Min, width), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn min_from_height(height: Dimension) -> Size<Constraints<Dimension>> {
        Size { height: Constraints::from_constraint(Constraint::Min, height), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn suggested_from_width(width: Dimension) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::from_constraint(Constraint::Suggested, width), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn suggested_from_height(height: Dimension) -> Size<Constraints<Dimension>> {
        Size { height: Constraints::from_constraint(Constraint::Suggested, height), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn max_from_width(width: Dimension) -> Size<Constraints<Dimension>> {
        Size { width: Constraints::from_constraint(Constraint::Max, width), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub fn max_from_height(height: Dimension) -> Size<Constraints<Dimension>> {
        Size { height: Constraints::from_constraint(Constraint::Max, height), ..Size::AUTO_CONSTRAINTS }
    }

    #[inline]
    pub const fn has_min_or_max(&self) -> bool {
        self.width.has_min_or_max() || self.height.has_min_or_max()
    }
}

impl<T> Axis<Constraints<T>>
where
    T: Copy,
{
    #[inline]
    pub fn min(&self) -> Axis<T> {
        self.with_inner(|inner| inner.min)
    }

    #[inline]
    pub fn max(&self) -> Axis<T> {
        self.with_inner(|inner| inner.max)
    }

    #[inline]
    pub fn suggested(&self) -> Axis<T> {
        self.with_inner(|inner| inner.suggested)
    }
}

pub trait MaybeSet<T> {
    fn maybe_set(self, value: T) -> Self;
}
