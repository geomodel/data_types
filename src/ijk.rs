#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct IJK {
    pub i: usize,
    pub j: usize,
    pub k: usize,
}

impl IJK {
    pub fn zero() -> Self {
        Self{
            i: 0,
            j: 0,
            k: 0,
        }
    }
}

impl std::convert::From<crate::RelIJK> for IJK {
    fn from(src: crate::RelIJK) -> Self {
        Self {
            i: src.i as usize,
            j: src.j as usize,
            k: src.k as usize,
        }
    }
}

impl std::ops::Add<crate::RelIJK> for IJK {
    type Output = Self;

    fn add(self, other: crate::RelIJK) -> Self {
        Self {
            i: self.i.wrapping_add(other.i as usize),
            j: self.j.wrapping_add(other.j as usize),
            k: self.k.wrapping_add(other.k as usize),
        }
    }
}
impl std::ops::Sub<crate::RelIJK> for IJK {
    type Output = Self;

    fn sub(self, other: crate::RelIJK) -> Self {
        Self {
            i: self.i.wrapping_sub(other.i as usize),
            j: self.j.wrapping_sub(other.j as usize),
            k: self.k.wrapping_sub(other.k as usize),
        }
    }
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod arithmetic {
    use super::*;

    #[test]
    fn check_add_negative() {
        let a = IJK::zero();
        let b = crate::RelIJK{ i: -2, j: -3, k: -4 };
        assert!( a == ((a+b) - b) );
    }
    #[test]
    fn check_add_simple() {
        let a = IJK::zero();
        let b = crate::RelIJK{ i: 2, j: 3, k: 4 };
        let r = IJK{ i: 2, j: 3, k: 4 };
        assert!( r == (a+b) );
    }

    #[test]
    fn check_sub() {
        let a = IJK{ i: 1, j: 2, k: 3 };
        let b = crate::RelIJK{ i: 1, j: 2, k: 3 };
        let r = IJK::zero();
        assert!( r == (a - b) );
    }
}
