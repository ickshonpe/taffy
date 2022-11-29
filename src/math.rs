//! Contains numerical helper traits and functions
#![allow(clippy::manual_clamp)]

use crate::geometry::AxisSize;
use crate::geometry::Size;
use crate::layout::AvailableSpace;
use crate::style::Constraints;

/// A trait to conveniently calculate minimums and maximums when some data may not be defined
///
/// If the left-hand value is [`None`], these operations return [`None`].
/// If the right-hand value is [`None`], it is treated as zero.
pub(crate) trait MaybeMath<In, Out> {
    /// Returns the minimum of `self` and `rhs`
    fn maybe_min(self, rhs: In) -> Out;

    /// Returns the maximum of `self` and `rhs`
    fn maybe_max(self, rhs: In) -> Out;

    /// Returns `self` clamped between `min` and `max`
    fn maybe_clamp(self, min: In, max: In) -> Out;

    /// Adds `self` and `rhs`.
    fn maybe_add(self, rhs: In) -> Out;

    /// Subtracts rhs from `self`, treating [`None`] values as default
    fn maybe_sub(self, rhs: In) -> Out;
}

// doesn't make sense to implement this maybe?
// impl MaybeMath<Option<f32>, Constraints<Option<f32>>> for Constraints<Option<f32>> {
//     fn maybe_min(self, rhs: Option<f32>) -> Constraints<Option<f32>> {
//         Constraints { 
//             min: self.min.maybe_min(rhs), 
//             suggested: self.suggested.maybe_min(rhs), 
//             max: self.max.maybe_min(rhs) 
//         }
//     }

//     fn maybe_max(self, rhs: Option<f32>) -> Constraints<Option<f32>> {
//         Constraints { 
//             min: self.min.maybe_max(rhs), 
//             suggested: self.suggested.maybe_max(rhs), 
//             max: self.max.maybe_max(rhs) 
//         }
//     }

//     fn maybe_clamp(self, min: Option<f32>, max: Option<f32>) -> Constraints<Option<f32>> {
//         Constraints { 
//             min: self.min.maybe_clamp(min, max), 
//             suggested: self.suggested.maybe_clamp(min, max), 
//             max: self.max.maybe_clamp(min, max) 
//         }
//     }

//     fn maybe_add(self, rhs: Option<f32>) -> Constraints<Option<f32>> {
//         Constraints { 
//             min: self.min.maybe_add(rhs), 
//             suggested: self.suggested.maybe_add(rhs), 
//             max: self.suggested.maybe_add(rhs) 
//         }
//     }

//     fn maybe_sub(self, rhs: Option<f32>) -> Constraints<Option<f32>> {
//         Constraints { 
//             min: self.min.maybe_sub(rhs), 
//             suggested: self.suggested.maybe_sub(rhs), 
//             max: self.suggested.maybe_sub(rhs) 
//         }
//     }
// }

impl MaybeMath<Option<f32>, Option<f32>> for Option<f32> {
    fn maybe_min(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.min(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_max(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l.max(r)),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_clamp(self, min: Option<f32>, max: Option<f32>) -> Option<f32> {
        match (self, min, max) {
            (Some(base), Some(min), Some(max)) => Some(base.min(max).max(min)),
            (Some(base), None, Some(max)) => Some(base.min(max)),
            (Some(base), Some(min), None) => Some(base.max(min)),
            (Some(_), None, None) => self,
            (None, _, _) => None,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l + r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }

    fn maybe_sub(self, rhs: Option<f32>) -> Option<f32> {
        match (self, rhs) {
            (Some(l), Some(r)) => Some(l - r),
            (Some(_l), None) => self,
            (None, Some(_r)) => None,
            (None, None) => None,
        }
    }
}

impl MaybeMath<f32, Option<f32>> for Option<f32> {
    fn maybe_min(self, rhs: f32) -> Option<f32> {
        self.map(|val| val.min(rhs))
    }

    fn maybe_max(self, rhs: f32) -> Option<f32> {
        self.map(|val| val.max(rhs))
    }

    fn maybe_clamp(self, min: f32, max: f32) -> Option<f32> {
        self.map(|val| val.min(max).max(min))
    }

    fn maybe_add(self, rhs: f32) -> Option<f32> {
        self.map(|val| val + rhs)
    }

    fn maybe_sub(self, rhs: f32) -> Option<f32> {
        self.map(|val| val - rhs)
    }
}

impl MaybeMath<Option<f32>, f32> for f32 {
    fn maybe_min(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self.min(val),
            None => self,
        }
    }

    fn maybe_max(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self.max(val),
            None => self,
        }
    }

    fn maybe_clamp(self, min: Option<f32>, max: Option<f32>) -> f32 {
        match (min, max) {
            (Some(min), Some(max)) => self.min(max).max(min),
            (None, Some(max)) => self.min(max),
            (Some(min), None) => self.max(min),
            (None, None) => self,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self + val,
            None => self,
        }
    }

    fn maybe_sub(self, rhs: Option<f32>) -> f32 {
        match rhs {
            Some(val) => self - val,
            None => self,
        }
    }
}

impl MaybeMath<f32, AxisSize<Option<f32>>> for AxisSize<Option<f32>> {
    fn maybe_min(self, rhs: f32) -> AxisSize<Option<f32>> {
        self.with_inner(|inner| 
            inner.maybe_min(rhs)
        )
    }

    fn maybe_max(self, rhs: f32) -> AxisSize<Option<f32>> {
        self.with_inner(|inner| 
            inner.maybe_max(rhs)
        )
    }

    fn maybe_clamp(self, min: f32, max: f32) -> AxisSize<Option<f32>> {
        self.with_inner(|inner| 
            inner.maybe_clamp(min, max)
        )
    }

    fn maybe_add(self, rhs: f32) -> AxisSize<Option<f32>> {
        self.with_inner(|inner| 
            inner.maybe_add(rhs)
        )
    }

    fn maybe_sub(self, rhs: f32) -> AxisSize<Option<f32>> {
        self.with_inner(|inner| 
            inner.maybe_sub(rhs)
        )
    }
}

impl MaybeMath<Size<Option<f32>>, AxisSize<f32>> for AxisSize<f32> {
    fn maybe_min(self, rhs: Size<Option<f32>>) -> AxisSize<f32> {
        self.with_size(
            rhs, 
            |inner, other| inner.maybe_min(other)
        )
    }

    fn maybe_max(self, rhs: Size<Option<f32>>) -> AxisSize<f32> {
        self.with_size(
            rhs, 
            |inner, other| inner.maybe_max(other)
        )
    }

    fn maybe_clamp(self, min: Size<Option<f32>>, max: Size<Option<f32>>) -> AxisSize<f32> {
        // self.with_size(
        //     min,
        //     |inner, min| 
        //         self.with_size(|inner, max| maybe_clamp(inner, min, max))
        // )
        self.pair(min).pair(max)
        .with_inner(|((size, min), max)| size.maybe_clamp(min, max))
    }

    fn maybe_add(self, rhs: Size<Option<f32>>) -> AxisSize<f32> {
        self.pair(rhs).with_inner(|(size, other)| size.maybe_add(other))
    }

    fn maybe_sub(self, rhs: Size<Option<f32>>) -> AxisSize<f32> {
        self.pair(rhs).with_inner(|(size, other)| size.maybe_sub(other))
    }
}

impl MaybeMath<f32, AvailableSpace> for AvailableSpace {
    fn maybe_min(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(rhs)),
            AvailableSpace::MinContent => AvailableSpace::Definite(rhs),
            AvailableSpace::MaxContent => AvailableSpace::Definite(rhs),
        }
    }
    fn maybe_max(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.max(rhs)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }

    fn maybe_clamp(self, min: f32, max: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val.min(max).max(min)),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }

    fn maybe_add(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val + rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
    fn maybe_sub(self, rhs: f32) -> AvailableSpace {
        match self {
            AvailableSpace::Definite(val) => AvailableSpace::Definite(val - rhs),
            AvailableSpace::MinContent => AvailableSpace::MinContent,
            AvailableSpace::MaxContent => AvailableSpace::MaxContent,
        }
    }
}

impl MaybeMath<Option<f32>, AvailableSpace> for AvailableSpace {
    fn maybe_min(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.min(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MinContent, None) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, Some(rhs)) => AvailableSpace::Definite(rhs),
            (AvailableSpace::MaxContent, None) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_max(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val.max(rhs)),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }

    fn maybe_clamp(self, min: Option<f32>, max: Option<f32>) -> AvailableSpace {
        match (self, min, max) {
            (AvailableSpace::Definite(val), Some(min), Some(max)) => AvailableSpace::Definite(val.min(max).max(min)),
            (AvailableSpace::Definite(val), None, Some(max)) => AvailableSpace::Definite(val.min(max)),
            (AvailableSpace::Definite(val), Some(min), None) => AvailableSpace::Definite(val.max(min)),
            (AvailableSpace::Definite(val), None, None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _, _) => AvailableSpace::MaxContent,
        }
    }

    fn maybe_add(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val + rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
    fn maybe_sub(self, rhs: Option<f32>) -> AvailableSpace {
        match (self, rhs) {
            (AvailableSpace::Definite(val), Some(rhs)) => AvailableSpace::Definite(val - rhs),
            (AvailableSpace::Definite(val), None) => AvailableSpace::Definite(val),
            (AvailableSpace::MinContent, _) => AvailableSpace::MinContent,
            (AvailableSpace::MaxContent, _) => AvailableSpace::MaxContent,
        }
    }
}

impl<In, Out, T: MaybeMath<In, Out>> MaybeMath<Size<In>, Size<Out>> for Size<T> {
    fn maybe_min(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_min(rhs.width), height: self.height.maybe_min(rhs.height) }
    }

    fn maybe_max(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_max(rhs.width), height: self.height.maybe_max(rhs.height) }
    }

    fn maybe_clamp(self, min: Size<In>, max: Size<In>) -> Size<Out> {
        Size {
            width: self.width.maybe_clamp(min.width, max.width),
            height: self.height.maybe_clamp(min.height, max.height),
        }
    }

    fn maybe_add(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_add(rhs.width), height: self.height.maybe_add(rhs.height) }
    }

    fn maybe_sub(self, rhs: Size<In>) -> Size<Out> {
        Size { width: self.width.maybe_sub(rhs.width), height: self.height.maybe_sub(rhs.height) }
    }
}

pub (crate) trait ApplyConstraints<In, Out> {
    fn apply_min(self, rhs: In) -> Out;
    fn apply_max(self, rhs: In) -> Out;
    fn apply_clamp(self, rhs: In) -> Out;
}

impl ApplyConstraints<Constraints<Option<f32>>, f32> for f32 {
    fn apply_min(self, rhs: Constraints<Option<f32>>) -> f32 {
        MaybeMath::maybe_min(self, rhs.min)
    }

    fn apply_max(self, rhs: Constraints<Option<f32>>) -> f32 {
        MaybeMath::maybe_max(self, rhs.max)
    }

    fn apply_clamp(self, rhs: Constraints<Option<f32>>) -> f32 {
        MaybeMath::maybe_clamp(self, rhs.min, rhs.max)
    }
}

impl ApplyConstraints<Constraints<Option<f32>>, Option<f32>> for Option<f32> {
    fn apply_min(self, rhs: Constraints<Option<f32>>) -> Option<f32> {
        self.maybe_min(rhs.min)
    }

    fn apply_max(self, rhs: Constraints<Option<f32>>) -> Option<f32> {
        self.maybe_max(rhs.max)
    }

    fn apply_clamp(self, rhs: Constraints<Option<f32>>) -> Option<f32> {
        self.maybe_clamp(rhs.min, rhs.max)
    }
}

impl ApplyConstraints<Size<Constraints<Option<f32>>>, Size<f32>> for Size<f32> {
    fn apply_min(self, rhs: Size<Constraints<Option<f32>>>) -> Size<f32> {
        Size {
            width: self.width.apply_min(rhs.width),
            height: self.height.apply_min(rhs.height),
        }
    }

    fn apply_max(self, rhs: Size<Constraints<Option<f32>>>) -> Size<f32>{
        Size {
            width: self.width.apply_max(rhs.width),
            height: self.height.apply_max(rhs.height),
        }
    }

    fn apply_clamp(self, rhs: Size<Constraints<Option<f32>>>) -> Size<f32> {
        Size {
            width: self.width.apply_clamp(rhs.width),
            height: self.height.apply_clamp(rhs.height),
        }
    }
}

impl ApplyConstraints<Size<Constraints<Option<f32>>>, AxisSize<f32>> for AxisSize<f32> {
    fn apply_min(self, rhs: Size<Constraints<Option<f32>>>) -> AxisSize<f32> {
        let constraint = match self {
            AxisSize::Height(_) => {
                rhs.height
            },
            AxisSize::Width(_) => {
                rhs.width
            }
        };
        //self.with_inner(self.value().apply_min(constraint))
        self.with_inner(|inner| inner.apply_min(constraint))
        
    }

    fn apply_max(self, rhs: Size<Constraints<Option<f32>>>) -> AxisSize<f32> {
        let constraint = match self {
            AxisSize::Height(_) => {
                rhs.height
            },
            AxisSize::Width(_) => {
                rhs.width
            }
        };
        self.with_inner(|inner| inner.apply_max(constraint))
    }

    fn apply_clamp(self, rhs: Size<Constraints<Option<f32>>>) -> AxisSize<f32> {
        let constraint = match self {
            AxisSize::Height(_) => {
                rhs.height
            },
            AxisSize::Width(_) => {
                rhs.width
            }
        };
        self.with_inner(|inner| inner.apply_clamp(constraint))
    }
}

impl Constraints<Option<f32>> {
    #[inline]
    pub fn clamp_suggested(&self) -> Option<f32> {
         match (self.suggested, self.min, self.max) {
            (Some(base), Some(min), Some(max)) => Some(base.min(max).max(min)),
            (Some(base), None, Some(max)) => Some(base.min(max)),
            (Some(base), Some(min), None) => Some(base.max(min)),
            (Some(base), None, None) => Some(base),
            (None, _, _) => None,
        }
    }
}

impl Size<Constraints<Option<f32>>> {
    #[inline]
    pub fn clamp_suggested(&self) -> Size<Option<f32>> {
        Size {
            width: self.width.clamp_suggested(),
            height: self.height.clamp_suggested()
        }
    }
}

impl AxisSize<Constraints<Option<f32>>> {
    #[inline]
    pub fn clamp_suggested(&self) -> AxisSize<Option<f32>> {
        self.with_inner(|inner| inner.clamp_suggested())
    }
}




#[cfg(test)]
mod tests {
    mod lhs_option_f32_rhs_option_f32 {

        use crate::math::MaybeMath;
        use rstest::rstest;

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(3.0))]
        #[case(Some(5.0), Some(3.0), Some(3.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_min(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_min(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(5.0))]
        #[case(Some(5.0), Some(3.0), Some(5.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_max(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_max(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(8.0))]
        #[case(Some(5.0), Some(3.0), Some(8.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_add(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_add(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), Some(5.0), Some(-2.0))]
        #[case(Some(5.0), Some(3.0), Some(2.0))]
        #[case(Some(3.0), None, Some(3.0))]
        #[case(None, Some(3.0), None)]
        #[case(None, None, None)]
        fn test_maybe_sub(#[case] lhs: Option<f32>, #[case] rhs: Option<f32>, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_sub(rhs), expected);
        }
    }

    mod lhs_option_f32_rhs_f32 {

        use crate::math::MaybeMath;
        use rstest::rstest;

        #[rstest]
        #[case(Some(3.0), 5.0, Some(3.0))]
        #[case(Some(5.0), 3.0, Some(3.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_min(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_min(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), 5.0, Some(5.0))]
        #[case(Some(5.0), 3.0, Some(5.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_max(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_max(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), 5.0, Some(8.0))]
        #[case(Some(5.0), 3.0, Some(8.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_add(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_add(rhs), expected);
        }

        #[rstest]
        #[case(Some(3.0), 5.0, Some(-2.0))]
        #[case(Some(5.0), 3.0, Some(2.0))]
        #[case(None, 3.0, None)]
        fn test_maybe_sub(#[case] lhs: Option<f32>, #[case] rhs: f32, #[case] expected: Option<f32>) {
            assert_eq!(lhs.maybe_sub(rhs), expected);
        }
    }

    mod lhs_f32_rhs_option_f32 {

        use crate::math::MaybeMath;
        use rstest::rstest;

        #[rstest]
        #[case(3.0, Some(5.0), 3.0)]
        #[case(5.0, Some(3.0), 3.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_min(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_min(rhs), expected);
        }

        #[rstest]
        #[case(3.0, Some(5.0), 5.0)]
        #[case(5.0, Some(3.0), 5.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_max(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_max(rhs), expected);
        }

        #[rstest]
        #[case(3.0, Some(5.0), 8.0)]
        #[case(5.0, Some(3.0), 8.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_add(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_add(rhs), expected);
        }

        #[rstest]
        #[case(3.0, Some(5.0), -2.0)]
        #[case(5.0, Some(3.0), 2.0)]
        #[case(3.0, None, 3.0)]
        fn test_maybe_sub(#[case] lhs: f32, #[case] rhs: Option<f32>, #[case] expected: f32) {
            assert_eq!(lhs.maybe_sub(rhs), expected);
        }
    }
}
