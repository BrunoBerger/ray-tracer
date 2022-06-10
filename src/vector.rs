
#[derive(Debug, Clone, Copy, PartialEq)]
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
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;
        (x*x + y*y + z*z).sqrt()
    }
    pub fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalise(&self) -> Vector {
        let len = self.length();
        *self * (1.0/len)
    }
    pub fn scale(&self, other: Vector) -> Vector {
        Vector{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f64 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}
pub fn cross(v1: &Vector, v2: &Vector) -> Vector {
    Vector{
        x: v1.y*v2.z - v1.z*v2.y,
        y: v1.z*v2.x - v1.x*v2.z,
        z: v1.x*v2.y - v1.y*v2.x
    }
}


impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        Vector{x: -self.x, y: -self.y, z: -self.z}
    }
}
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Vector {
        Vector{x: self.x*rhs, y: self.y*rhs, z: self.z*rhs}
    }
}
impl std::ops::Div<i32> for Vector {
    type Output = Vector;
    fn div(self, rhs: i32) -> Vector {
        self * (1.0/rhs as f64)
    }
}
impl std::ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Vector {
        self * (1.0/rhs)
    }
}
impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector{x: self.x+other.x, y: self.y+other.y, z: self.z+other.z}
    }
}
impl std::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = *self + other
    }
}
impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
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