use std::{io::Write, os::windows::ffi::OsStrExt, thread, time::Duration};

use addin1c::{name, AddinResult, MethodInfo, SimpleAddin, Variant};
use smallvec::SmallVec;

pub struct Addin {}

impl Addin {
    pub fn new() -> Addin {
        Addin {}
    }

    fn pid(&mut self, ret_value: &mut Variant) -> AddinResult {
        ret_value.set_i32(std::process::id() as i32);
        Ok(())
    }

    fn env(&mut self, name: &mut Variant, ret_value: &mut Variant) -> AddinResult {
        let name = name.get_string()?;
        match std::env::var_os(name) {
            Some(var) => {
                let buf: SmallVec<[u16; 64]> = var.to_string_lossy().encode_utf16().collect();
                ret_value.set_str1c(buf.as_slice())?;
            }
            None => ret_value.set_empty(),
        }
        Ok(())
    }

    fn envs(&mut self, ret_value: &mut Variant) -> AddinResult {
        let mut buf = Vec::<u16>::new();
        for (name, value) in std::env::vars_os() {
            buf.extend(name.to_string_lossy().encode_utf16());
            buf.push('=' as u16);
            buf.extend(value.to_string_lossy().encode_utf16());
            buf.push('\n' as u16);
        }
        ret_value.set_str1c(buf.as_slice())?;
        Ok(())
    }

    fn current_dir(&mut self, ret_value: &mut Variant) -> AddinResult {
        let mut buf = SmallVec::<[u16; 256]>::new();
        buf.extend(std::env::current_dir()?.as_os_str().encode_wide());
        ret_value.set_str1c(buf.as_slice())?;
        Ok(())
    }

    fn current_exe(&mut self, ret_value: &mut Variant) -> AddinResult {
        let mut buf = SmallVec::<[u16; 256]>::new();
        buf.extend(std::env::current_exe()?.as_os_str().encode_wide());
        ret_value.set_str1c(buf.as_slice())?;
        Ok(())
    }

    fn print(&mut self, param: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        let s = param.get_string()?;
        std::io::stdout().write_all(s.as_bytes())?;
        Ok(())
    }

    fn eprint(&mut self, param: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        let s = param.get_string()?;
        std::io::stderr().write_all(s.as_bytes())?;
        Ok(())
    }

    fn sleep(&mut self, param: &mut Variant, _ret_value: &mut Variant) -> AddinResult {
        let millis = param.get_i32()?;
        thread::sleep(Duration::from_millis(millis as _));
        Ok(())
    }
}

impl SimpleAddin for Addin {
    fn name() -> &'static [u16] {
        name!("Utils")
    }

    fn get_info() -> u16 {
        1000
    }

    fn methods() -> &'static [MethodInfo<Self>]
    where
        Self: Sized,
    {
        &[
            MethodInfo {
                name: name!("Pid"),
                method: addin1c::Methods::Method0(Self::pid),
            },
            MethodInfo {
                name: name!("Env"),
                method: addin1c::Methods::Method1(Self::env),
            },
            MethodInfo {
                name: name!("Envs"),
                method: addin1c::Methods::Method0(Self::envs),
            },
            MethodInfo {
                name: name!("CurrentDir"),
                method: addin1c::Methods::Method0(Self::current_dir),
            },
            MethodInfo {
                name: name!("CurrentExe"),
                method: addin1c::Methods::Method0(Self::current_exe),
            },
            MethodInfo {
                name: name!("Print"),
                method: addin1c::Methods::Method1(Self::print),
            },
            MethodInfo {
                name: name!("EPrint"),
                method: addin1c::Methods::Method1(Self::eprint),
            },
            MethodInfo {
                name: name!("Sleep"),
                method: addin1c::Methods::Method1(Self::sleep),
            },
        ]
    }
}
