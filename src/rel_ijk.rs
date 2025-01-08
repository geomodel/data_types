#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct RelIJK {
    pub i: isize,
    pub j: isize,
    pub k: isize,
}

impl std::convert::From<crate::IJK> for RelIJK {
    fn from(src: crate::IJK) -> Self {
        Self {
            i: src.i as isize,
            j: src.j as isize,
            k: src.k as isize,
        }
    }
}

impl std::ops::Add for RelIJK {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}
impl std::ops::Neg for RelIJK {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}
impl std::ops::Sub for RelIJK {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
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
        let a = RelIJK { i: 1, j: 2, k: 3 };
        let b = RelIJK { i: 2, j: 3, k: 4 };
        let r = RelIJK { i: 3, j: 5, k: 7 };
        assert!(r == (a + b));
    }
    #[test]
    fn check_neg() {
        let a = RelIJK { i: 1, j: 2, k: 3 };
        let r = RelIJK {
            i: -1,
            j: -2,
            k: -3,
        };
        assert!(r == -a);
    }
    #[test]
    fn check_sub() {
        let a = RelIJK { i: 1, j: 2, k: 3 };
        let b = RelIJK { i: 2, j: 3, k: 4 };
        let r = RelIJK { i: 1, j: 1, k: 1 };
        assert!(r == (b - a));
    }
}
