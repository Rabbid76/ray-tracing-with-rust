use crate::random;
use crate::types::{FSize, Vector3};
use core::mem::MaybeUninit;

pub struct Perlin {
    rand: Vec<Vector3>,
    perm_x: Vec<u8>,
    perm_y: Vec<u8>,
    perm_z: Vec<u8>,
}

impl Perlin {
    pub fn new() -> Perlin {
        Perlin {
            rand: Perlin::generate(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    fn generate() -> Vec<Vector3> {
        let v: Vec<Vector3> = (0..256)
            .map(|_| glm::normalize(random::generate_vector3()))
            .collect();
        v
    }

    fn generate_perm() -> Vec<u8> {
        let mut perm: Vec<u8> = (0..256).map(|x| x as u8).collect();
        Perlin::permute(&mut perm);
        perm
    }

    fn permute(v: &mut Vec<u8>) {
        for i in 255..1 {
            let target = (random::generate_size() * (i as FSize + 1.0)) as usize;
            v.swap(i, target);
        }
    }

    pub fn noise(&self, p: &Vector3) -> FSize {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[Vector3; 2]; 2]; 2] = unsafe { MaybeUninit::uninit().assume_init() };
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                        .clone();
                }
            }
        }
        Perlin::trilinear_interpolation(&c, u, v, w)
    }

    fn trilinear_interpolation(c: &[[[Vector3; 2]; 2]; 2], u: FSize, v: FSize, w: FSize) -> FSize {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += glm::dot(
                        Vector3::new(u - i as FSize, v - j as FSize, w - k as FSize),
                        c[i][j][k],
                    ) * ((i as FSize * uu) + (1.0 - i as FSize) * (1.0 - uu))
                        * ((j as FSize * vv) + (1.0 - j as FSize) * (1.0 - vv))
                        * ((k as FSize * ww) + (1.0 - k as FSize) * (1.0 - ww));
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: Vector3, depth: usize) -> FSize {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }
}

// TODO test
