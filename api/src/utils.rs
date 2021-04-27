use anyhow::anyhow;

pub trait Clown<T> {
    fn clown(self) -> Result<T, anyhow::Error>;
}

impl<R, E: std::fmt::Display> Clown<R> for Result<R, E> {
    fn clown(self) -> Result<R, anyhow::Error> {
        self.map_err(|e| anyhow!("{}", e))
    }
}
