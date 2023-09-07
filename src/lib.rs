use std::{ffi::c_int, mem::MaybeUninit};

mod c_imports {
    use super::Prng;
    #[link(name = "mt19937ar", kind = "static")]
    extern "C" {
        pub fn init_genrand(state: *mut Prng, seed: u32);
        pub fn init_by_array(state: *mut Prng, init_key: *const u32, key_length: usize);
        pub fn genrand_res53(state: *mut Prng) -> f64;
        pub fn genrand_uint32(state: *mut Prng) -> u32;
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Prng {
    index: c_int,
    state: [u32; 624],
}
impl Prng {
    pub fn new_init_genrand(seed: u32) -> Prng {
        let mut state = MaybeUninit::<Prng>::uninit();
        unsafe {
            c_imports::init_genrand(state.as_mut_ptr(), seed);
            state.assume_init()
        }
    }
    pub fn new_init_by_array(init_key: &[u32]) -> Prng {
        let mut state = MaybeUninit::<Prng>::uninit();
        unsafe {
            c_imports::init_by_array(state.as_mut_ptr(), init_key.as_ptr(), init_key.len());
            state.assume_init()
        }
    }
    pub fn init_genrand(&mut self, seed: u32) {
        unsafe {
            c_imports::init_genrand(self as *mut _, seed);
        }
    }
    pub fn init_by_array(&mut self, seed: &[u32]) {
        unsafe {
            c_imports::init_by_array(self as *mut _, seed.as_ptr(), seed.len());
        }
    }
    pub fn genrand_res53(&mut self) -> f64 {
        unsafe { c_imports::genrand_res53(self as *mut _) }
    }
    pub fn genrand_uint32(&mut self) -> u32 {
        unsafe { c_imports::genrand_uint32(self as *mut _) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut state = Prng::new_init_by_array(&[0]);
        assert_eq!(&state.genrand_res53().to_string()[..7], "0.84442");
        assert_eq!(&state.genrand_res53().to_string()[..7], "0.75795");
        assert_eq!(&state.genrand_res53().to_string()[..7], "0.42057");
        state.init_genrand(69);
        assert_eq!(&state.genrand_res53().to_string()[..7], "0.29624");
        assert_eq!(state.genrand_uint32(), 3474919369);
    }
}
