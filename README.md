# taffy

[![GitHub CI](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml/badge.svg)](https://github.com/DioxusLabs/taffy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/taffy.svg)](https://crates.io/crates/taffy)
[![docs.rs](https://img.shields.io/docsrs/taffy)](https://docs.rs/taffy)

`taffy` is a flexible, high-performance, cross-platform UI layout library written in [Rust](https://www.rust-lang.org).

Currently, we only support a [flexbox](https://css-tricks.com/snippets/css/a-guide-to-flexbox/) layout algorithm, but support for other paradigms [is planned](https://github.com/DioxusLabs/taffy/issues/28).

This crate is a collaborative, cross-team project, and is designed to be used as a dependency for other UI and GUI libraries.
Right now, it powers:

- [Dioxus](https://dioxuslabs.com/): a React-like library for building fast, portable, and beautiful user interfaces with Rust
- [Bevy](https://bevyengine.org/): an ergonomic, ECS-first Rust game engine

## Usage

```rust
use taffy::prelude::*;

// First create an instance of Taffy
let mut taffy = Taffy::new();

// Create a tree of nodes using `taffy.new_leaf` and `taffy.new_with_children`.
// These functions both return a node id which can be used to refer to that node
// The Style struct is used to specify styling information
let header_node = taffy
    .new_leaf(
        Style {
            size_constraints: Size::suggested_from_points(800.0, 100.0),
            ..Default::default()
        },
    ).unwrap();

let body_node = taffy
    .new_leaf(
        Style {
            size_constraints: Size::suggested_from(Size { width: Dimension::Points(800.0), height: Dimension::Undefined }),
            flex_grow: 1.0,
            ..Default::default()
        },
    ).unwrap();

let root_node = taffy
    .new_with_children(
        Style {
            flex_direction: FlexDirection::Column,
            size_constraints: Size::suggested_from_points(800.0, 600.0 ),
            ..Default::default()
        },
        &[header_node, body_node],
    )
    .unwrap();

// Call compute_layout on the root of your tree to run the layout algorithm
taffy.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

// Inspect the computed layout using taffy.layout
assert_eq!(taffy.layout(root_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(root_node).unwrap().size.height, 600.0);
assert_eq!(taffy.layout(header_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(header_node).unwrap().size.height, 100.0);
assert_eq!(taffy.layout(body_node).unwrap().size.width, 800.0);
assert_eq!(taffy.layout(body_node).unwrap().size.height, 500.0); // This value was not set explicitly, but was computed by Taffy

```

## Benchmarks (vs. [Yoga](https://github.com/facebook/yoga))

- Run on a 2021 MacBook Pro with M1 Pro processor.
- Taffy benchmarks are using criterion (10 iterations).
- Yoga benchmarks run via it's node.js bindings (the `yoga-layout-prebuilt` npm package), they were run a few times manually and it was verified that variance in the numbers of each run was minimal. It should be noted that this is using an old version of Yoga.

(note that the table below contains multiple different units (milliseconds vs. microseconds vs. nanoseconds))

| Benchmark | Yoga | Taffy 0.2 |
| --- | --- | --- |
| yoga/10 nodes (1-level hierarchy) | 45.1670 µs | 33.297 ns |
| yoga/100 nodes (2-level hierarchy) | 134.1250 µs | 336.53 ns |
| yoga/1_000 nodes (3-level hierarchy) | 1.2221 ms | 3.8928 µs |
| yoga/10_000 nodes (4-level hierarchy) | 13.8672 ms | 36.162 µs |
| yoga/100_000 nodes (5-level hierarchy) | 141.5307 ms | 1.6404 ms |

Most popular websites seem to have between 3,000 and 10,000 nodes (although they also require text layout, which neither yoga nor taffy implement).

## Contributions

[Contributions welcome](https://github.com/DioxusLabs/taffy/blob/main/CONTRIBUTING.md):
if you'd like to use, improve or build `taffy`, feel free to join the conversation, open an [issue](https://github.com/DioxusLabs/taffy/issues) or submit a [PR](https://github.com/DioxusLabs/taffy/pulls).
If you have questions about how to use `taffy`, open a [discussion](https://github.com/DioxusLabs/taffy/discussions) so we can answer your questions in a way that others can find.
