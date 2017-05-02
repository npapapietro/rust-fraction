use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;


fn main() {
    let x:i32 = 7;
    let y:i32 = 13;
    let f = Fraction::new(x,y);
    let g = &f + & 2i32;

    //let i:Fraction<i32> = 2i32 - f ;
    println!("{}",g);
    println!("{}",f);
}
struct Fraction<T>{
    numerator: T,
    denominator: T,
}
impl<T> Fraction<T>{
    fn new(num: T, denom : T)->Fraction<T>{
        Fraction{numerator : num, denominator : denom}
    }
}

//allows print('{}',Fraction)
impl<T> fmt::Display for Fraction<T> where T : fmt::Display{
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f, "{}/{}" , self.numerator,self.denominator )
    }
}

/*
*
*               Over loaded add functions
*           Fraction+Fraction, Fraction+T
*
*/

impl<T:Copy + Add<T, Output = T>+Mul<T,Output=T>> Add<Fraction<T>> for Fraction<T>{
    type Output = Fraction<T>;
    fn add(self, other : Fraction<T>)->Fraction<T>{
        let denom =self.denominator * other.denominator;
        let numer = self.numerator * other.denominator + self.denominator * other.numerator;
        Fraction{numerator: numer,  denominator:denom}
    }
}
impl<'a, 'b, T:Copy + Add<T, Output = T>+Mul<T,Output=T>> Add<&'b Fraction<T>> for &'a Fraction<T>{
    type Output = Fraction<T>;
    fn add(self, other : &Fraction<T>)->Fraction<T>{
        let denom =self.denominator * other.denominator;
        let numer = self.numerator * other.denominator + self.denominator * other.numerator;
        Fraction{numerator: numer,  denominator:denom}
    }
}

impl<T:Copy + Add<T, Output = T>+Mul<T,Output=T>> Add<T> for Fraction<T>{
    type Output = Fraction<T>;
    fn add(self, other : T)->Fraction<T>{
        let denom =self.denominator;
        let numer = self.numerator  + self.denominator * other;
        Fraction{numerator: numer,  denominator:denom}
    }
}

impl<'a,'b, T:Copy + Add<T, Output = T>+Mul<T,Output=T>> Add<&'b T> for &'a Fraction<T>{
    type Output = Fraction<T>;
    fn add(self, other : &T)->Fraction<T>{
        let denom =self.denominator;
        let numer = self.numerator  + self.denominator * (*other);
        Fraction{numerator: numer,  denominator:denom}
    }
}

/*
*
*               Over loaded sub functions
*           Fraction-Fraction, Fraction-T
*
*/

impl<T:Copy + Sub<T, Output = T>+Mul<T,Output=T>> Sub<Fraction<T>> for Fraction<T>{
    type Output = Fraction<T>;
    fn sub(self, other : Fraction<T>)->Fraction<T>{
        let denom =self.denominator * other.denominator;
        let numer = self.numerator * other.denominator - self.denominator * other.numerator;
        Fraction{numerator: numer,  denominator:denom}
    }
}
impl<'a, 'b, T:Copy + Sub<T, Output = T>+Mul<T,Output=T>> Sub<&'b Fraction<T>> for &'a Fraction<T>{
    type Output = Fraction<T>;
    fn sub(self, other : &Fraction<T>)->Fraction<T>{
        let denom =self.denominator * other.denominator;
        let numer = self.numerator * other.denominator - self.denominator * other.numerator;
        Fraction{numerator: numer,  denominator:denom}
    }
}

impl<T:Copy + Sub<T, Output = T>+Mul<T,Output=T>> Sub<T> for Fraction<T>{
    type Output = Fraction<T>;
    fn sub(self, other : T)->Fraction<T>{
        let denom =self.denominator;
        let numer = self.numerator  - self.denominator * other;
        Fraction{numerator: numer,  denominator:denom}
    }
}

impl<'a,'b, T:Copy + Sub<T, Output = T>+Mul<T,Output=T>> Sub<&'b T> for &'a Fraction<T>{
    type Output = Fraction<T>;
    fn sub(self, other : &T)->Fraction<T>{
        let denom =self.denominator;
        let numer = self.numerator  - self.denominator * (*other);
        Fraction{numerator: numer,  denominator:denom}
    }
}

/*
*
*               Over loaded + - * / functions
*           T-Fraction, T + Fraction, T*Fraction, T/Fraction
*
*/



impl Sub<Fraction<i32>> for i32{
    type Output = Fraction<i32>;
    fn sub(self, other:Fraction<i32>)->Fraction<i32>{
        Fraction{numerator: self*other.denominator-other.numerator,denominator:other.denominator}
    }
}
impl Sub<Fraction<i64>> for i64{
    type Output = Fraction<i64>;
    fn sub(self, other:Fraction<i64>)->Fraction<i64>{
        Fraction{numerator: self*other.denominator-other.numerator,denominator:other.denominator}
    }
}

impl Mul<Fraction<i32>> for i32{
    type Output = Fraction<i32>;
    fn mul(self, other:Fraction<i32>)->Fraction<i32>{
        Fraction{numerator: self*other.numerator,denominator:other.denominator}
    }
}
impl Mul<Fraction<i64>> for i64{
    type Output = Fraction<i64>;
    fn mul(self, other:Fraction<i64>)->Fraction<i64>{
        Fraction{numerator: self*other.numerator,denominator:other.denominator}
    }
}
