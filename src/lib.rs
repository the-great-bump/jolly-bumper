use semver::Version;

const STABLE: Version = Version::new(1, 0, 0);

#[derive(sqlx::FromRow, Debug)]
pub struct Crate {
    pub id: i32,
    pub name: String,
    pub repository: Option<String>,
    pub downloads: i32,
    pub version: String,
}

impl Crate {
    pub fn is_stable(&self) -> bool {
        let version = Version::parse(&self.version).expect("failed to parse version");

        version >= STABLE
    }
}
