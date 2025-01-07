#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct IJK {
    pub i: usize,
    pub j: usize,
    pub k: usize,
}

impl std::convert::From<crate::RelIJK> for IJK {
    fn from( src: crate::RelIJK) -> Self {
        Self {
            i: src.i as usize,
            j: src.j as usize,
            k: src.k as usize,
        }
    }
}

impl std::ops::Add for IJK {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}
impl std::ops::Sub for IJK {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
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
    fn check_add() {
        let a = IJK{ i: 1, j: 2, k: 3 };
        let b = IJK{ i: 2, j: 3, k: 4 };
        let r = IJK{ i: 3, j: 5, k: 7 };
        assert!( r == (a+b) );
    }
    #[test]
    fn check_sub() {
        let a = IJK{ i: 1, j: 2, k: 3 };
        let b = IJK{ i: 2, j: 3, k: 4 };
        let r = IJK{ i: 1, j: 1, k: 1 };
        assert!( r == (b-a) );
    }
}
