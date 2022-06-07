use std::fmt;
use std::ops::{ Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign };
use num_bigint::{ BigInt, ToBigInt };
use num_traits::{ Zero };
use std::result::Result;
use colored::Colorize;


/// Variants of this enum represent different variable types supported
/// in runk.
#[derive(PartialEq, Clone)]
pub enum Var {
    /// Natural number type
    N(BigInt),
    /// Integer type
    Z(BigInt),
}

// ------------
// CREATING VAR
// ------------
/// # Methods for **creating** varialbes.
impl Var {
    /// Creates a new variable of variant N with value as its value.
    pub fn z(value: num_bigint::BigInt) -> Result<Var, String> {
        if !Var::can_be_z(&value) {
            return Err(format!("Error converting \"{}\" to an integer.", value));
        }

        Ok(Var::Z(value))
    }

    /// Creates a new variable of variant N with value as its value.
    pub fn n(value: num_bigint::BigInt) -> Result<Var, String> {
        if !Var::can_be_n(&value) {
            return Err(format!("Error converting \"{}\" to natural number.", value));
        }

        Ok(Var::N(value))
    }

    /// Creates a numeric variable (can be of any variant) from value.
    pub fn create_numeric_var(value: Option<num_bigint::BigInt>) -> Self {
        Var::Z(match value {
            Some(v) => v,
            None => Zero::zero(),
        }).best_fit()
    }

    /// Creates a variable from
    pub fn from_str(value: &str) -> Result<Var, String> {
        match value.parse::<BigInt>() {
            Ok(i) => {
                Ok(Var::new(i))
            },
            Err(_e) => {
                Err(format!("Cannot convert \"{}\" to a number.", value.italic()))
            },
        }
    }

    /// Creates a new var holding a given number.
    pub fn new(value: impl num_bigint::ToBigInt) -> Var {
        let bigint = match ToBigInt::to_bigint(&value) {
            Some(b) => b,
            None => {
                eprintln!("Error during bigint conversion. This was never supposed to happen lol.");
                std::process::exit(1);
            },
        };

        if Var::can_be_n(&bigint) {
            return Var::N(bigint);
        }
        if Var::can_be_z(&bigint) {
            return Var::Z(bigint);
        }

        // Should never ever happen
        Var::Z(Zero::zero())
    }

}


// -----------
// TESTING VAR
// -----------
/// # Collection of methods testing properties of variable and its variants.
impl Var {
    /// Returns true if num can be represented as an integer
    pub fn can_be_z(_num: &BigInt) -> bool {
        true
    }

    /// Returns true if num can be represented as an natural number
    pub fn can_be_n(num: &BigInt) -> bool {
        num >= &ToBigInt::to_bigint(&0).unwrap()
    }

    /// Returns true if self and other are the same variant
    pub fn eq_type(&self, other: &Self) -> bool {
        match self {
            Var::N(_n) => {
                match other {
                    Var::N(_o_n) => {
                        true
                    },
                    Var::Z(_o_z) => {
                        false
                    },
                }
            },
            Var::Z(_z) => {
                match other {
                    Var::N(_o_n) => {
                        false
                    },
                    Var::Z(_o_z) => {
                        true
                    },
                }
            },
        }
    }

}


// --------------
// CONVERTING VAR
// --------------
/// # Collection of methods for converting variables (usually acros variants.)
impl Var {

    /// Returns a variable containing self's value as variant Z.
    pub fn get_z(&self) -> Result<Self, String> {
        // Getting original value.
        let value: &BigInt = match self {
            Var::N(n) => n,
            Var::Z(z) => return Result::Ok(self.clone()),
        };

        // Checking conditions.
        // In this case none.

        // Converting
        Result::Ok(Var::Z(value.clone()))
    }

    /// Returns a variable containing self's value as variant N.
    pub fn get_n(&self) -> Result<Self, String> {

        let value: &BigInt = match self {
            Var::N(_n) => return Result::Ok(self.clone()),
            Var::Z(z) => &z,
        };

        // Checking conditions.
        if !Var::can_be_n(&value) {
            return Result::Err(format!("Attempted to convert a negative number \"{}\" to N", value.to_string().italic()));
        }

        // Converting
        Result::Ok(Var::N(value.clone()))
    }

    /// Converts any var (if possible) to the N variant.
    pub fn to_n(&mut self) -> Result<Self, String> {
        let var = self.get_n()?;
        *self = var;
        Ok(self.clone())
    }

    /// Converts any var (if possible) to the Z variant.
    pub fn to_z(&mut self) -> Result<Self, String> {
        let var = self.get_z()?;
        *self = var;
        Ok(self.clone())
    }

    /// Copies value of self into other **while preserving other's type**.
    pub fn fit_into<'a>(&self, other: &'a mut Var) -> Result<&'a Self, String> {
        *other = match other {
            Var::N(_n) => self.get_n()?,
            Var::Z(_z) => self.get_z()?,
        };

        Result::Ok(other)
    }

    /// Changes the variable type into a one that has the most specific type.
    ///
    /// ## Example
    /// - Positive integer will allways change to Var::N
    /// - Negative integer will always change to Var::Z
    pub fn best_fit(&mut self) -> Self {
        let result = self.to_n();
        if result.is_ok() {
            return result.unwrap().clone();
        }

        return self.clone();
    }
}


// ---
// Mic
// ---
/// # Mics functions.
impl Var {
    pub fn plain_string(&self) -> String {
        match self {
            Var::N(n) => {
                format!("{}", n)
            },
            Var::Z(z) => {
                format!("{}", z)
            }
        }
    }
}


// ====================
// TRAIT IMPLEMENTATION
// ====================


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


impl fmt::Debug for Var {
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

/// # Generating arithmetic operators.
macro_rules! ops_generate {
    ($trait1: ident, $fn1: ident, $trait2: ident, $fn2: ident) => {
        impl $trait1 for Var {
            type Output = Self;
            fn $fn1(self, other: Var) -> Self::Output {
                return match self {
                    Var::N(n) => {
                        match other {
                            Var::N(o_n) => {
                                Var::new(n.clone().$fn1(o_n))
                            },
                            Var::Z(o_z) => {
                                Var::new(n.clone().$fn1(o_z))
                            },
                        }
                    },
                    Var::Z(z) => {
                        match other {
                            Var::N(o_n) => {
                                Var::new(z.clone().$fn1(o_n))
                            },
                            Var::Z(o_z) => {
                                Var::new(z.clone().$fn1(o_z))
                            },
                        }
                    },
                };
            }
        }


        impl $trait2 for Var {
            fn $fn2(&mut self, other: Var) {
                *self = self.clone().$fn1(other);
            }
        }

    }
}
ops_generate!(Add, add, AddAssign, add_assign);
ops_generate!(Sub, sub, SubAssign, sub_assign);
ops_generate!(Mul, mul, MulAssign, mul_assign);
ops_generate!(Div, div, DivAssign, div_assign);


// ==========
// Unit tests
// ==========
#[test]
fn test_get_n() {
    let num1 = num_bigint::ToBigInt::to_bigint(&-20).unwrap();
    let num2 = num_bigint::ToBigInt::to_bigint(&20).unwrap();
    assert_eq!(Var::can_be_n(&num1), false);
    assert_eq!(Var::can_be_n(&num2), true);
}

#[test]
fn test_get_z() {
    let num1 = num_bigint::ToBigInt::to_bigint(&-20).unwrap();
    let num2 = num_bigint::ToBigInt::to_bigint(&20).unwrap();
    assert_eq!(Var::can_be_z(&num1), true);
    assert_eq!(Var::can_be_z(&num2), true);
}

#[test]
fn test_fit_into() {
    let var1 = Var::z(ToBigInt::to_bigint(&-30).unwrap()).unwrap();
    let mut var2 = Var::n(ToBigInt::to_bigint(&1).unwrap()).unwrap();
    let mut var3 = Var::z(ToBigInt::to_bigint(&50).unwrap()).unwrap();

    // Testing that conversion failed.
    // Conversion shoud fail because var1 is negative number
    // and var2 only holds natural numbers.
    assert_eq!(var1.fit_into(&mut var2).is_err(), true);
    // Testing that var2 remained unchanged.
    assert_eq!(var2, Var::n(ToBigInt::to_bigint(&1).unwrap()).unwrap());

    // Testing conversion success
    assert_eq!(var2.fit_into(&mut var3).is_ok(), true);
    // Testing that var2 and var3 hold the same value.
    assert_eq!(var2, var3.to_n().unwrap());
    // Testing that var3's variant remained unchanged.
    assert_eq!(var3.eq_type(&Var::z(ToBigInt::to_bigint(&50).unwrap()).unwrap()), false);
}
