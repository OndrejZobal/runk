use std::fmt;
use std::ops::AddAssign;

pub enum Var {
    N(u64),
    Z(i64),
}

impl Var {
    // TODO
    // pub fn fit_from<T>(num: T) -> Var {
    //
    // }

    pub fn fit_into(&self, num: &mut Var) -> bool {
        match num {
            Var::Z(z1) => {
                match self {
                    Var::Z(z2) => {
                        *z1 = *z2;
                        return true;
                    },
                    Var::N(n2) => {
                        let converted = i64::try_from(*n2);
                        if !converted.is_ok() {
                            return false;
                        }

                        *z1 = converted.unwrap();
                        return true;
                    }
                }

            },

            Var::N(n1) => {
                match self {
                    Var::Z(z2) => {
                        let converted = u64::try_from(*z2);
                        if !converted.is_ok() {
                            return false;
                        }

                        *n1 = converted.unwrap();
                        return true;

                    },
                    Var::N(n2) => {
                        *n1 = *n2;
                        return true;
                    }
                }
            },
        }
    }

    /// Changes the variable type into a one that fits the best.
    pub fn best_fit(&mut self) {
        *self = match self {
            Var::Z(z) => {
                if z <= &mut 0 {
                    Var::N(i64::try_into(*z).unwrap())
                }
                else {
                    Var::Z(*z)
                }
            },
            Var::N(n) => {
                Var::N(*n)
            },
        }
    }

    // pub fn bruh<T> (num: &T) -> Var {
    //
    // }
}

impl Clone for Var {
    fn clone(&self) -> Self {
        match self {
            Var::N(n) => {
                Var::N(*n)
            },
            Var::Z(z) => {
                Var::Z(*z)
            }
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Var::N(n) => {
                write!(f, "N({})", n)
            },
            Var::Z(z) => {
                write!(f, "Z({})", z)
            }
        }
    }
}

impl AddAsign for Var {
    /// This is bad
    fn try_add(num1: &impl AddAssign, num2: &impl AddAssign)  {

    }

    fn add_assign(&mut self, other: &Var) {
        match self {
            // Self is an integer.
            Var::Z(z) => {
                match other => {
                    // Other is an integer
                    Var::Z(o_z) => {
                        if try_add(z, o_z) {

                        }
                    },
                    // Other is a naturatl number.
                    Var::N(o_n) => {

                    },
                }
            },
            // Self is a natural number.
            Var::N(n) => {

            },
        }
    }
}
