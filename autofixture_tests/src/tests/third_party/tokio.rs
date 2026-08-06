use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;
use tokio::sync::{Mutex, RwLock};

#[tokio::test]
async fn tokio_mutex_creates_successfully() {
    let mut f = Fixture::new();
    let m: Mutex<u32> = f.create();
    let _value = *m.lock().await;
}

#[tokio::test]
async fn tokio_mutex_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Mutex<u32>>();
    let _value = builder.create();
}

#[tokio::test]
async fn tokio_rwlock_creates_successfully() {
    let mut f = Fixture::new();
    let l: RwLock<u32> = f.create();
    let _value = *l.read().await;
}

#[tokio::test]
async fn tokio_rwlock_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<RwLock<u32>>();
    let _value = builder.create();
}

#[tokio::test]
async fn tokio_mutex_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<u32> = {
        let mut collected = Vec::with_capacity(10);

        for _ in 0..10 {
            let m: Mutex<u32> = f.create();
            collected.push(*m.lock().await);
        }

        collected
    };

    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    assert!(!all_same, "expected tokio::sync::Mutex<u32> values to vary");
}
