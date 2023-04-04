//! This file includes benchmarks for tree creation
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use taffy::prelude::*;
use taffy::style::Style;

mod helpers;

#[cfg(feature = "yoga_benchmark")]
use helpers::yoga_helpers;
#[cfg(feature = "yoga_benchmark")]
use slotmap::SlotMap;
#[cfg(feature = "yoga_benchmark")]
use yoga_helpers::yg;

/// Build a random leaf node
fn build_random_leaf(taffy: &mut Taffy) -> Node {
    taffy.new_with_children(Style::DEFAULT, &[]).unwrap()
}

/// A tree with many children that have shallow depth
fn build_taffy_flat_hierarchy(total_node_count: u32, use_with_capacity: bool) -> (Taffy, Node) {
    let mut taffy =  Taffy::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> = (0..sub_children_count).map(|_| build_random_leaf(&mut taffy)).collect();
        let node = taffy.new_with_children(Style::DEFAULT, &sub_children).unwrap();

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = taffy.new_with_children(Style::DEFAULT, children.as_slice()).unwrap();
    (taffy, root)
}

#[cfg(feature = "yoga_benchmark")]
/// A tree with many children that have shallow depth
fn build_yoga_flat_hierarchy(total_node_count: u32) -> (yg::YogaTree, Node) {
    let mut tree = SlotMap::new();
    let mut rng = ChaCha8Rng::seed_from_u64(12345);
    let mut children = Vec::new();
    let mut node_count = 0;

    while node_count < total_node_count {
        let sub_children_count = rng.gen_range(1..=4);
        let sub_children: Vec<Node> =
            (0..sub_children_count).map(|_| yoga_helpers::new_default_style_with_children(&mut tree, vec![])).collect();
        let node = yoga_helpers::new_default_style_with_children(&mut tree, sub_children);

        children.push(node);
        node_count += 1 + sub_children_count;
    }

    let root = yoga_helpers::new_default_style_with_children(&mut tree, children);
    (tree, root)
}

fn taffy_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tree creation");
    for node_count in [1_000u32, 10_000, 100_000].iter() {
        #[cfg(feature = "yoga_benchmark")]
        let benchmark_id = BenchmarkId::new(format!("Yoga"), node_count);
        #[cfg(feature = "yoga_benchmark")]
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter(|| {
                let (taffy, root) = build_yoga_flat_hierarchy(node_count);
                std::hint::black_box(taffy);
                std::hint::black_box(root);
            })
        });
        let benchmark_id = BenchmarkId::new(format!("Taffy::new"), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter(|| {
                let (tree, root) = build_taffy_flat_hierarchy(node_count, false);
                std::hint::black_box(tree);
                std::hint::black_box(root);
            })
        });

        let benchmark_id = BenchmarkId::new(format!("Taffy::with_capacity"), node_count);
        group.bench_with_input(benchmark_id, node_count, |b, &node_count| {
            b.iter(|| {
                let (tree, root) = build_taffy_flat_hierarchy(node_count, true);
                std::hint::black_box(tree);
                std::hint::black_box(root);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, taffy_benchmarks);
criterion_main!(benches);
