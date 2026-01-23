//! Benchmarks for starmap system

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use uqm::core::{Point, StarColor, StarIndex, StarType};
use uqm::starmap::{Star, StarMap};

fn bench_star_distance_calculation(c: &mut Criterion) {
    let star1 = Star {
        index: StarIndex::new(0),
        position: Point::new(0, 0),
        star_type: StarType::Dwarf,
        color: StarColor::Yellow,
        num_planets: 1,
        name: "Star1".to_string(),
        visited: false,
    };
    
    let star2 = Star {
        index: StarIndex::new(1),
        position: Point::new(1000, 1000),
        star_type: StarType::Giant,
        color: StarColor::Red,
        num_planets: 3,
        name: "Star2".to_string(),
        visited: false,
    };
    
    c.bench_function("star_distance_calculation", |b| {
        b.iter(|| {
            black_box(star1.distance_to(black_box(&star2)))
        });
    });
}

fn bench_find_closest_star(c: &mut Criterion) {
    let mut map = StarMap::new();
    
    // Create 100 random stars
    for i in 0..100 {
        map.stars.push(Star {
            index: StarIndex::new(i),
            position: Point::new((i * 73) % 10000, (i * 131) % 10000),
            star_type: StarType::Dwarf,
            color: StarColor::Yellow,
            num_planets: (i % 10) as u8,
            name: format!("Star{}", i),
            visited: false,
        });
    }
    
    c.bench_function("find_closest_star_100_systems", |b| {
        b.iter(|| {
            black_box(map.find_closest_star(black_box(Point::new(5000, 5000))))
        });
    });
}

fn bench_star_creation(c: &mut Criterion) {
    c.bench_function("star_creation", |b| {
        let mut i = 0;
        b.iter(|| {
            let _star = Star {
                index: StarIndex::new(i),
                position: Point::new(black_box(100), black_box(200)),
                star_type: StarType::Dwarf,
                color: StarColor::Yellow,
                num_planets: 5,
                name: format!("Star{}", i),
                visited: false,
            };
            i += 1;
        });
    });
}

criterion_group!(
    benches,
    bench_star_distance_calculation,
    bench_find_closest_star,
    bench_star_creation
);
criterion_main!(benches);
