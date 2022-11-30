#[cfg(test)]
mod min_max_overrides {

    use taffy::prelude::*;
    use taffy::style::Constraints;

    #[test]
    fn min_overrides_max() {
        let mut taffy = Taffy::new();

        let child = taffy
            .new_leaf(Style {
                size_constraints: Size {
                    width: Constraints {
                        suggested: Dimension::Points(50.0),
                        min: Dimension::Points(100.0),
                        max: Dimension::Points(10.0),
                    },
                    height: Constraints {
                        suggested: Dimension::Points(50.0),
                        min: Dimension::Points(100.0),
                        max: Dimension::Points(10.0),
                    },
                    ..Default::default()
                },
                // size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                // min_size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
                // max_size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                child,
                Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        assert_eq!(taffy.layout(child).unwrap().size, Size { width: 100.0, height: 100.0 });
    }

    #[test]
    fn max_overrides_size() {
        let mut taffy = Taffy::new();

        let child = taffy
            .new_leaf(Style {
                // size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                // max_size: Size { width: Dimension::Points(10.0), height: Dimension::Points(10.0) },
                size_constraints: Size {
                    width: Constraints {
                        suggested: Dimension::Points(50.0),
                        max: Dimension::Points(10.0),
                        ..Default::default()
                    },
                    height: Constraints {
                        suggested: Dimension::Points(50.0),
                        max: Dimension::Points(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                child,
                Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        assert_eq!(taffy.layout(child).unwrap().size, Size { width: 10.0, height: 10.0 });
    }

    #[test]
    fn min_overrides_size() {
        let mut taffy = Taffy::new();

        let child = taffy
            .new_leaf(Style {
                // size: Size { width: Dimension::Points(50.0), height: Dimension::Points(50.0) },
                // min_size: Size { width: Dimension::Points(100.0), height: Dimension::Points(100.0) },
                size_constraints: Size {
                    width: Constraints {
                        suggested: Dimension::Points(50.0),
                        min: Dimension::Points(100.0),
                        ..Default::default()
                    },
                    height: Constraints {
                        suggested: Dimension::Points(50.0),
                        min: Dimension::Points(100.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .unwrap();

        taffy
            .compute_layout(
                child,
                Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
            )
            .unwrap();

        assert_eq!(taffy.layout(child).unwrap().size, Size { width: 100.0, height: 100.0 });
    }
}
