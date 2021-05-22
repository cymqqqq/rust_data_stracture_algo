use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
struct Complex<T> {
    /// Real portion of the complex number
    re: T,

    /// Imaginary portion of the complex number
    im: T
}
#[cfg(skip)]
impl Add for Complex<i32> {
    type Output = Complex<i32>;
    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}
impl<T> Add for Complex<T>
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}
impl<T> Mul for Complex<T>
    where T:Add<Output=T>+Mul<Output=T>+Sub<Output=T>+Copy
{
    type Output=Self;
    fn mul(self,rhs:Self)->Self{
         Complex { re: self.re * rhs.re - self.im * rhs.im,
                  im: self.re * rhs.im + self.im * rhs.re }
    }
}
impl Add<Complex<f64>> for f64 {
    type Output = Complex<f64>;
    fn add(self, rhs: Complex<f64>) -> Complex<f64> {
        Complex { re: rhs.re + self, im: rhs.im }
    }
}
use std::ops::Neg;

impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;
    fn neg(self) -> Self::Output {
        Complex { re: -self.re, im: -self.im }
    }
}
use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}
impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: Eq> Eq for Complex<T> { }
use std::fmt;

// To make the formatting examples mesh with the rest of this file, I've adapted
// them to work on the type `Complex<f64>`, where the book simply defines a new
// non-generic `Complex` type. The only changes are adding `<f64>`, and changing
// the field names.

#[cfg(skip)]
impl fmt::Display for Complex<f64> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let i_sign = if self.i < 0.0 { '-' } else { '+' };
        write!(dest, "{} {} {}i", self.r, i_sign, f64::abs(self.i))
    }
}

impl fmt::Display for Complex<f64> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let (r, i) = (self.re, self.im);
        if dest.alternate() {
            let abs = f64::sqrt(r * r + i * i);
            let angle = f64::atan2(i, r) / std::f64::consts::PI * 180.0;
            write!(dest, "{} ∠ {}°", abs, angle)
        } else {
            let i_sign = if i < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", r, i_sign, f64::abs(i))
        }
    }
}
