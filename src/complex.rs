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
