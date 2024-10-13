use std::{error::Error, time::Instant};

use addin1c::{name, AddinResult, MethodInfo, PropInfo, SimpleAddin, Variant};

pub struct Addin {
    last_error: Option<Box<dyn Error>>,
    instant: Instant,
}

impl Addin {
    pub fn new() -> Addin {
        Addin {
            last_error: None,
            instant: Instant::now(),
        }
    }

    fn last_error(&mut self, value: &mut Variant) -> AddinResult {
        match &self.last_error {
            Some(err) => value
                .set_str1c(err.to_string().as_str())
                .map_err(|e| e.into()),
            None => value.set_str1c("").map_err(|e| e.into()),
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

    fn save_error(&mut self, err: Option<Box<dyn Error>>) {
        self.last_error = err;
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

    fn properties() -> &'static [PropInfo<Self>] {
        &[PropInfo {
            name: name!("LastError"),
            getter: Some(Self::last_error),
            setter: None,
        }]
    }
}
