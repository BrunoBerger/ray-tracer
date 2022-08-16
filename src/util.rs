
use crate::vector::Vector;
use rand_distr::{UnitSphere, Distribution};

pub fn rndm_on_sphere() -> Vector {
    let v: [f32; 3] = UnitSphere.sample(&mut rand::thread_rng());
    Vector::new(v[0], v[1], v[2])
}

// https://stackoverflow.com/questions/26983355/is-there-a-way-to-combine-multiple-traits-in-order-to-define-a-new-trait
// https://stackoverflow.com/questions/30291584/macro-for-defining-trait-aliases
// trait master_trait: Debug + Clone + Copy + serde::Serialize + serde::Deserialize {}
// impl<T> master_trait for T where T: Debug + Clone + Copy + serde::Serialize + serde::Deserialize {}

// in Carg.toml:
// serde = { version = "1.0.143", features = ["derive"] }
// serde_json = "1.0.83"
