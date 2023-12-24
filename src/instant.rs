use std::time::Instant;

use addin1c::{name, AddinResult, MethodInfo, SimpleAddin, Variant};

pub struct Addin {
    instant: Instant,
}

impl Addin {
    pub fn new() -> Addin {
        Addin {
            instant: Instant::now(),
        }
    }

    fn start(&mut self, _ret_value: &mut Variant) -> AddinResult {
        self.instant = Instant::now();
        Ok(())
    }

    fn elapsed(&mut self, ret_value: &mut Variant) -> AddinResult {
        ret_value.set_f64(self.instant.elapsed().as_nanos() as f64 / 1000.);
        Ok(())
    }
}

impl SimpleAddin for Addin {
    fn name() -> &'static [u16] {
        name!("Instant")
    }

    fn methods() -> &'static [addin1c::MethodInfo<Self>]
    where
        Self: Sized,
    {
        &[
            MethodInfo {
                name: name!("Start"),
                method: addin1c::Methods::Method0(Self::start),
            },
            MethodInfo {
                name: name!("Elapsed"),
                method: addin1c::Methods::Method0(Self::elapsed),
            },
        ]
    }
}
