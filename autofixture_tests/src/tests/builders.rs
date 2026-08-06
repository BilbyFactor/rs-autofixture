use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[test]
fn numeric_builder_with_range_exclusive() {
    let mut f = Fixture::new();
    let mut builder = f.build::<u32>();

    for _ in 0..100 {
        let v = builder
            .with_range(10..20)
            .create();

        assert!((10..20).contains(&v), "expected {v} in 10..20");
    }
}

#[test]
fn numeric_builder_with_range_inclusive() {
    let mut f = Fixture::new();
    let mut builder = f.build::<i32>();

    for _ in 0..100 {
        let v = builder
            .with_range(-5..=5)
            .create();

        assert!((-5..=5).contains(&v), "expected {v} in -5..=5");
    }
}

#[test]
fn numeric_builder_with_range_unbounded_end() {
    let mut f = Fixture::new();
    let mut builder = f.build::<u8>();

    for _ in 0..100 {
        let v = builder
            .with_range(200..)
            .create();

        assert!(v >= 200, "expected {v} >= 200");
    }
}

#[test]
fn numeric_builder_with_range_unbounded_start() {
    let mut f = Fixture::new();
    let mut builder = f.build::<i8>();

    for _ in 0..100 {
        let v = builder
            .with_range(..=0)
            .create();

        assert!(v <= 0, "expected {v} <= 0");
    }
}

#[test]
fn numeric_builder_with_options() {
    let mut f = Fixture::new();
    let mut builder = f.build::<u32>();

    for _ in 0..100 {
        let mut opts = vec![10, 20, 30];

        let v = builder
            .with_options(&mut opts)
            .create();

        assert!(
            [10, 20, 30].contains(&v),
            "expected {v} to be one of [10, 20, 30]"
        );
    }
}

#[test]
fn numeric_builder_options_clears_range() {
    let mut f = Fixture::new();
    let mut builder = f.build::<u32>();

    for _ in 0..100 {
        let mut opts = vec![1, 2, 3];

        let v = builder
            .with_range(1000..2000)
            .with_options(&mut opts)
            .create();

        assert!(
            [1, 2, 3].contains(&v),
            "expected {v} to be one of [1, 2, 3] after options cleared range"
        );
    }
}

#[test]
fn numeric_builder_range_clears_options() {
    let mut f = Fixture::new();
    let mut builder = f.build::<u32>();

    for _ in 0..100 {
        let mut opts = vec![5000, 6000, 7000];

        let v = builder
            .with_options(&mut opts)
            .with_range(0..10)
            .create();

        assert!(
            (0..10).contains(&v),
            "expected {v} in 0..10 after range cleared options"
        );
    }
}

#[test]
fn float_builder_with_range() {
    let mut f = Fixture::new();
    let mut builder = f.build::<f64>();

    for _ in 0..100 {
        let v = builder
            .with_range(0.0..1.0)
            .create();

        assert!((0.0..1.0).contains(&v), "expected {v} in 0.0..1.0");
    }
}

#[test]
fn char_builder_with_options() {
    let mut f = Fixture::new();
    let mut builder = f.build::<char>();

    for _ in 0..50 {
        let mut opts = vec!['x', 'y', 'z'];

        let v = builder
            .with_options(&mut opts)
            .create();

        assert!(
            ['x', 'y', 'z'].contains(&v),
            "expected {v} to be one of ['x', 'y', 'z']"
        );
    }
}
