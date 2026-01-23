//! Benchmarks for melee combat system
//!
//! These benchmarks measure performance-critical operations
//! to ensure we meet or exceed the original C implementation.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uqm::core::Vec2;
use uqm::melee::{Ship, ShipType};

fn bench_ship_update(c: &mut Criterion) {
    c.bench_function("ship_update_single_frame", |b| {
        let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
        ship.velocity = Vec2::new(100.0, 50.0);
        
        b.iter(|| {
            ship.update(black_box(0.016)); // 60 FPS frame time
        });
    });
}

fn bench_ship_creation(c: &mut Criterion) {
    c.bench_function("ship_creation", |b| {
        b.iter(|| {
            let _ship = Ship::new(
                black_box(ShipType::Earthling),
                black_box(Vec2::zero())
            );
        });
    });
}

fn bench_ship_damage(c: &mut Criterion) {
    c.bench_function("ship_take_damage", |b| {
        let mut ship = Ship::new(ShipType::Earthling, Vec2::zero());
        
        b.iter(|| {
            ship.take_damage(black_box(5));
            // Reset for next iteration
            ship.crew = 18;
        });
    });
}

fn bench_multiple_ships_update(c: &mut Criterion) {
    c.bench_function("update_10_ships", |b| {
        let mut ships: Vec<Ship> = (0..10)
            .map(|i| {
                let mut ship = Ship::new(ShipType::Earthling, Vec2::new(i as f32 * 100.0, 0.0));
                ship.velocity = Vec2::new(10.0, 10.0);
                ship
            })
            .collect();
        
        b.iter(|| {
            for ship in &mut ships {
                ship.update(black_box(0.016));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_ship_update,
    bench_ship_creation,
    bench_ship_damage,
    bench_multiple_ships_update
);
criterion_main!(benches);
