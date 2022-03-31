
// #[derive(Debug, Clone, Copy)]
// pub struct Vector {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64
// }

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(x: f64, y: f64, z:f64) -> Vector {
        Vector{x, y, z}
    }

    pub fn distance(&self, other: &Vector)-> f64 {
        let tx = self.x + other.x;
        let ty = self.y + other.y;
        let tz = self.z + other.z;
        (tx*tx + ty*ty + tz*tz).sqrt()
    }
    
    pub fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f64{
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        // let X = self.x*rhs;
        // let Y = self.y*rhs;
        // let Z = self.z*rhs;
        // Vector::new(X, Y, Z)
        Vector{x: self.x*rhs, y: self.y*rhs, z: self.z*rhs}
    }
}

impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector{
        Vector{x: self.x+other.x, y: self.y+other.y, z: self.z+other.z}
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}


#[test]
fn both_len_meth_eq(){
    let v1 = Vector::new(1.0, 0.0, 0.0);
    let len1 = v1.length();
    let len2 = v1.distance(&Vector::new(0.0, 0.0, 0.0));
    assert_eq!(len1, len2);

}