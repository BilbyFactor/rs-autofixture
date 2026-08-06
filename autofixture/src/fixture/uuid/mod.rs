//! AutoFixture implementation for [`Uuid`](https://docs.rs/uuid).
//!
//! By default, `create()` and the builder produce v4 (random) UUIDs.
//!
//! With the `uuid-extra` feature enabled, the builder gains version selection
//! methods: `with_v1`, `with_v3`, `with_v5`, `with_v6`, `with_v7`, and `with_v8`.

use uuid::Uuid;

use crate::fixture::{AutoFixture, Fixture, builder::FixtureBuilder};

#[cfg(feature = "uuid-extra")]
use crate::fixture::FixtureExt;

// Without the uuid-extra feature, the builder never uses fixture...
#[allow(dead_code)]
pub struct UuidBuilder<'b> {
    fixture: &'b mut Fixture,

    #[cfg(feature = "uuid-extra")]
    version: UuidVersion,
}

#[cfg(feature = "uuid-extra")]
enum UuidVersion {
    V4,
    V1,
    V3 { namespace: Uuid, name: Vec<u8> },
    V5 { namespace: Uuid, name: Vec<u8> },
    V6,
    V7,
    V8 { buf: [u8; 16] },
}

#[cfg(feature = "uuid-extra")]
impl<'b> UuidBuilder<'b> {
    /// Generate a v1 `Uuid` (timestamp + random node ID).
    pub fn with_v1(&mut self) -> &mut Self {
        self.version = UuidVersion::V1;
        self
    }

    /// Generate a v3 `Uuid` (MD5 namespace + name).
    /// Uses a random namespace and name if either are not specified.
    ///
    /// # Arguments
    /// * `namespace` - the MD5 namespace to use.
    /// * `name` - the MD5 name to use.
    pub fn with_v3(&mut self, namespace: Uuid, name: &[u8]) -> &mut Self {
        self.version = UuidVersion::V3 {
            namespace,
            name: name.to_vec(),
        };

        self
    }

    /// Generate a v5 `Uuid` (SHA-1 namespace + name).
    /// Uses a random namespace and name if not specified.
    ///
    /// # Arguments
    /// * `namespace` - the SHA-1 namespace to use.
    /// * `name` - the SHA-1 name to use.
    pub fn with_v5(&mut self, namespace: Uuid, name: &[u8]) -> &mut Self {
        self.version = UuidVersion::V5 {
            namespace,
            name: name.to_vec(),
        };

        self
    }

    /// Generate a v6 `Uuid` (reordered timestamp + random node ID).
    pub fn with_v6(&mut self) -> &mut Self {
        self.version = UuidVersion::V6;
        self
    }

    /// Generate a v7 `Uuid` (Unix timestamp + random).
    pub fn with_v7(&mut self) -> &mut Self {
        self.version = UuidVersion::V7;
        self
    }

    /// Generate a v8 `Uuid` from a given 16-byte buffer.
    ///
    /// # Arguments
    /// * `buf` - the 16-byte input buffer to generate using.
    pub fn with_v8(&mut self, buf: [u8; 16]) -> &mut Self {
        self.version = UuidVersion::V8 { buf };
        self
    }
}

impl<'b> FixtureBuilder<'b> for UuidBuilder<'b> {
    type F = Uuid;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            #[cfg(feature = "uuid-extra")]
            version: UuidVersion::V4,
        }
    }

    fn create(&mut self) -> Self::F {
        #[cfg(not(feature = "uuid-extra"))]
        {
            Uuid::new_v4()
        }

        #[cfg(feature = "uuid-extra")]
        {
            use rand::RngExt;
            use uuid::Timestamp;

            match &self.version {
                UuidVersion::V4 => Uuid::new_v4(),
                UuidVersion::V1 => {
                    let ts = Timestamp::now(uuid::NoContext);
                    let node_id: [u8; 6] = self
                        .fixture
                        .rng()
                        .random();

                    Uuid::new_v1(ts, &node_id)
                }
                UuidVersion::V3 { namespace, name } => Uuid::new_v3(namespace, name),
                UuidVersion::V5 { namespace, name } => Uuid::new_v5(namespace, name),
                UuidVersion::V6 => {
                    let ts = Timestamp::now(uuid::NoContext);
                    let node_id: [u8; 6] = self
                        .fixture
                        .rng()
                        .random();

                    Uuid::new_v6(ts, &node_id)
                }
                UuidVersion::V7 => {
                    let ts = Timestamp::now(uuid::NoContext);

                    Uuid::new_v7(ts)
                }
                UuidVersion::V8 { buf } => Uuid::new_v8(*buf),
            }
        }
    }
}

impl AutoFixture for Uuid {
    type Builder<'b> = UuidBuilder<'b>;

    fn create(_f: &mut Fixture) -> Self {
        Uuid::new_v4()
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        UuidBuilder::new(f)
    }
}
