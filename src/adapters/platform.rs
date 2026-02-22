use crate::ports::Platform;

pub struct StdPlatform;

impl Platform for StdPlatform {
    fn os(&self) -> &'static str { std::env::consts::OS }

    fn arch(&self) -> &'static str { std::env::consts::ARCH }
}
