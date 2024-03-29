
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z:f32) -> Vector {
        Vector{x, y, z}
    }
    pub fn new_from_u8(x: u8, y: u8, z:u8) -> Vector {
        Vector{x: x as f32 / 255.0, 
            y: y as f32 / 255.0,
            z: z as f32 / 255.0}
    }
    pub fn new_from_one_float(f: f32) -> Vector {
        Vector{x: f, y: f, z: f}
    }
    pub fn distance(&self, other: &Vector)-> f32 {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;
        (x*x + y*y + z*z).sqrt()
    }
    pub fn length(&self) -> f32 {
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
    // pub fn to_img_rgb(&self) -> image::Rgb<u8> {
    //     image::Rgb([self.x as u8, self.y as u8, self.z as u8])
    // }
    pub fn encode(self) -> [u8; 3] {
        [
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8
        ]
    }
}

pub fn dot(v1: &Vector, v2: &Vector) -> f32 {
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
impl std::ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Vector {
        Vector{x: self.x*rhs, y: self.y*rhs, z: self.z*rhs}
    }
}
impl std::ops::Div<i32> for Vector {
    type Output = Vector;
    fn div(self, rhs: i32) -> Vector {
        self * (1.0/rhs as f32)
    }
}
impl std::ops::Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Vector {
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
