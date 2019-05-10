#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTPUTDIRCTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR0R::INDEPENDENT => 0,
            SETCLR0R::L_REVERSED => 1,
            SETCLR0R::H_REVERSED => 2,
            SETCLR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR0R {
        match value {
            0 => SETCLR0R::INDEPENDENT,
            1 => SETCLR0R::L_REVERSED,
            2 => SETCLR0R::H_REVERSED,
            i => SETCLR0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR0R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR0R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR0R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR1R::INDEPENDENT => 0,
            SETCLR1R::L_REVERSED => 1,
            SETCLR1R::H_REVERSED => 2,
            SETCLR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR1R {
        match value {
            0 => SETCLR1R::INDEPENDENT,
            1 => SETCLR1R::L_REVERSED,
            2 => SETCLR1R::H_REVERSED,
            i => SETCLR1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR1R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR1R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR1R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR2R::INDEPENDENT => 0,
            SETCLR2R::L_REVERSED => 1,
            SETCLR2R::H_REVERSED => 2,
            SETCLR2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR2R {
        match value {
            0 => SETCLR2R::INDEPENDENT,
            1 => SETCLR2R::L_REVERSED,
            2 => SETCLR2R::H_REVERSED,
            i => SETCLR2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR2R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR2R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR2R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR3R::INDEPENDENT => 0,
            SETCLR3R::L_REVERSED => 1,
            SETCLR3R::H_REVERSED => 2,
            SETCLR3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR3R {
        match value {
            0 => SETCLR3R::INDEPENDENT,
            1 => SETCLR3R::L_REVERSED,
            2 => SETCLR3R::H_REVERSED,
            i => SETCLR3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR3R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR3R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR3R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR4R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR4R::INDEPENDENT => 0,
            SETCLR4R::L_REVERSED => 1,
            SETCLR4R::H_REVERSED => 2,
            SETCLR4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR4R {
        match value {
            0 => SETCLR4R::INDEPENDENT,
            1 => SETCLR4R::L_REVERSED,
            2 => SETCLR4R::H_REVERSED,
            i => SETCLR4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR4R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR4R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR4R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR5R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR5R::INDEPENDENT => 0,
            SETCLR5R::L_REVERSED => 1,
            SETCLR5R::H_REVERSED => 2,
            SETCLR5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR5R {
        match value {
            0 => SETCLR5R::INDEPENDENT,
            1 => SETCLR5R::L_REVERSED,
            2 => SETCLR5R::H_REVERSED,
            i => SETCLR5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR5R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR5R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR5R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR6R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR6R::INDEPENDENT => 0,
            SETCLR6R::L_REVERSED => 1,
            SETCLR6R::H_REVERSED => 2,
            SETCLR6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR6R {
        match value {
            0 => SETCLR6R::INDEPENDENT,
            1 => SETCLR6R::L_REVERSED,
            2 => SETCLR6R::H_REVERSED,
            i => SETCLR6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR6R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR6R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR6R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR7R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR7R::INDEPENDENT => 0,
            SETCLR7R::L_REVERSED => 1,
            SETCLR7R::H_REVERSED => 2,
            SETCLR7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR7R {
        match value {
            0 => SETCLR7R::INDEPENDENT,
            1 => SETCLR7R::L_REVERSED,
            2 => SETCLR7R::H_REVERSED,
            i => SETCLR7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR7R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR7R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR7R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR8R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR8R::INDEPENDENT => 0,
            SETCLR8R::L_REVERSED => 1,
            SETCLR8R::H_REVERSED => 2,
            SETCLR8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR8R {
        match value {
            0 => SETCLR8R::INDEPENDENT,
            1 => SETCLR8R::L_REVERSED,
            2 => SETCLR8R::H_REVERSED,
            i => SETCLR8R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR8R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR8R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR8R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR9R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR9R::INDEPENDENT => 0,
            SETCLR9R::L_REVERSED => 1,
            SETCLR9R::H_REVERSED => 2,
            SETCLR9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR9R {
        match value {
            0 => SETCLR9R::INDEPENDENT,
            1 => SETCLR9R::L_REVERSED,
            2 => SETCLR9R::H_REVERSED,
            i => SETCLR9R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR9R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR9R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR9R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR10R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR10R::INDEPENDENT => 0,
            SETCLR10R::L_REVERSED => 1,
            SETCLR10R::H_REVERSED => 2,
            SETCLR10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR10R {
        match value {
            0 => SETCLR10R::INDEPENDENT,
            1 => SETCLR10R::L_REVERSED,
            2 => SETCLR10R::H_REVERSED,
            i => SETCLR10R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR10R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR10R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR10R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR11R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR11R::INDEPENDENT => 0,
            SETCLR11R::L_REVERSED => 1,
            SETCLR11R::H_REVERSED => 2,
            SETCLR11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR11R {
        match value {
            0 => SETCLR11R::INDEPENDENT,
            1 => SETCLR11R::L_REVERSED,
            2 => SETCLR11R::H_REVERSED,
            i => SETCLR11R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR11R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR11R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR11R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR12R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR12R::INDEPENDENT => 0,
            SETCLR12R::L_REVERSED => 1,
            SETCLR12R::H_REVERSED => 2,
            SETCLR12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR12R {
        match value {
            0 => SETCLR12R::INDEPENDENT,
            1 => SETCLR12R::L_REVERSED,
            2 => SETCLR12R::H_REVERSED,
            i => SETCLR12R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR12R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR12R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR12R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR13R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR13R::INDEPENDENT => 0,
            SETCLR13R::L_REVERSED => 1,
            SETCLR13R::H_REVERSED => 2,
            SETCLR13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR13R {
        match value {
            0 => SETCLR13R::INDEPENDENT,
            1 => SETCLR13R::L_REVERSED,
            2 => SETCLR13R::H_REVERSED,
            i => SETCLR13R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR13R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR13R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR13R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR14R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR14R::INDEPENDENT => 0,
            SETCLR14R::L_REVERSED => 1,
            SETCLR14R::H_REVERSED => 2,
            SETCLR14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR14R {
        match value {
            0 => SETCLR14R::INDEPENDENT,
            1 => SETCLR14R::L_REVERSED,
            2 => SETCLR14R::H_REVERSED,
            i => SETCLR14R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR14R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR14R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR14R::H_REVERSED
    }
}
#[doc = "Possible values of the field `SETCLR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR15R {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR15R::INDEPENDENT => 0,
            SETCLR15R::L_REVERSED => 1,
            SETCLR15R::H_REVERSED => 2,
            SETCLR15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR15R {
        match value {
            0 => SETCLR15R::INDEPENDENT,
            1 => SETCLR15R::L_REVERSED,
            2 => SETCLR15R::H_REVERSED,
            i => SETCLR15R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline]
    pub fn is_independent(&self) -> bool {
        *self == SETCLR15R::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `L_REVERSED`"]
    #[inline]
    pub fn is_l_reversed(&self) -> bool {
        *self == SETCLR15R::L_REVERSED
    }
    #[doc = "Checks if the value of the field is `H_REVERSED`"]
    #[inline]
    pub fn is_h_reversed(&self) -> bool {
        *self == SETCLR15R::H_REVERSED
    }
}
#[doc = "Values that can be written to the field `SETCLR0`"]
pub enum SETCLR0W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR0W::INDEPENDENT => 0,
            SETCLR0W::L_REVERSED => 1,
            SETCLR0W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR0W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR0W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR0W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR1`"]
pub enum SETCLR1W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR1W::INDEPENDENT => 0,
            SETCLR1W::L_REVERSED => 1,
            SETCLR1W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR1W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR1W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR1W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR2`"]
pub enum SETCLR2W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR2W::INDEPENDENT => 0,
            SETCLR2W::L_REVERSED => 1,
            SETCLR2W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR2W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR2W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR2W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR3`"]
pub enum SETCLR3W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR3W::INDEPENDENT => 0,
            SETCLR3W::L_REVERSED => 1,
            SETCLR3W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR3W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR3W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR3W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR4`"]
pub enum SETCLR4W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR4W::INDEPENDENT => 0,
            SETCLR4W::L_REVERSED => 1,
            SETCLR4W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR4W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR4W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR4W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR4W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR5`"]
pub enum SETCLR5W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR5W::INDEPENDENT => 0,
            SETCLR5W::L_REVERSED => 1,
            SETCLR5W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR5W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR5W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR5W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR5W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR6`"]
pub enum SETCLR6W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR6W::INDEPENDENT => 0,
            SETCLR6W::L_REVERSED => 1,
            SETCLR6W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR6W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR6W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR6W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR6W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR7`"]
pub enum SETCLR7W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR7W::INDEPENDENT => 0,
            SETCLR7W::L_REVERSED => 1,
            SETCLR7W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR7W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR7W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR7W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR7W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR8`"]
pub enum SETCLR8W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR8W::INDEPENDENT => 0,
            SETCLR8W::L_REVERSED => 1,
            SETCLR8W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR8W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR8W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR8W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR8W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR8W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR9`"]
pub enum SETCLR9W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR9W::INDEPENDENT => 0,
            SETCLR9W::L_REVERSED => 1,
            SETCLR9W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR9W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR9W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR9W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR9W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR9W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR10`"]
pub enum SETCLR10W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR10W::INDEPENDENT => 0,
            SETCLR10W::L_REVERSED => 1,
            SETCLR10W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR10W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR10W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR10W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR10W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR10W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR11`"]
pub enum SETCLR11W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR11W::INDEPENDENT => 0,
            SETCLR11W::L_REVERSED => 1,
            SETCLR11W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR11W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR11W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR11W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR11W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR11W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR12`"]
pub enum SETCLR12W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR12W::INDEPENDENT => 0,
            SETCLR12W::L_REVERSED => 1,
            SETCLR12W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR12W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR12W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR12W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR12W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR12W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR13`"]
pub enum SETCLR13W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR13W::INDEPENDENT => 0,
            SETCLR13W::L_REVERSED => 1,
            SETCLR13W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR13W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR13W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR13W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR13W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR13W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR14`"]
pub enum SETCLR14W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR14W::INDEPENDENT => 0,
            SETCLR14W::L_REVERSED => 1,
            SETCLR14W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR14W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR14W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR14W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR14W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR14W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR15`"]
pub enum SETCLR15W {
    #[doc = "Set and clear do not depend on the direction of any counter."]
    INDEPENDENT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    L_REVERSED,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    H_REVERSED,
}
impl SETCLR15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR15W::INDEPENDENT => 0,
            SETCLR15W::L_REVERSED => 1,
            SETCLR15W::H_REVERSED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR15W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR15W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on the direction of any counter."]
    #[inline]
    pub fn independent(self) -> &'a mut W {
        self.variant(SETCLR15W::INDEPENDENT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn l_reversed(self) -> &'a mut W {
        self.variant(SETCLR15W::L_REVERSED)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn h_reversed(self) -> &'a mut W {
        self.variant(SETCLR15W::H_REVERSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr0(&self) -> SETCLR0R {
        SETCLR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr1(&self) -> SETCLR1R {
        SETCLR1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr2(&self) -> SETCLR2R {
        SETCLR2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr3(&self) -> SETCLR3R {
        SETCLR3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr4(&self) -> SETCLR4R {
        SETCLR4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr5(&self) -> SETCLR5R {
        SETCLR5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr6(&self) -> SETCLR6R {
        SETCLR6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr7(&self) -> SETCLR7R {
        SETCLR7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr8(&self) -> SETCLR8R {
        SETCLR8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr9(&self) -> SETCLR9R {
        SETCLR9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr10(&self) -> SETCLR10R {
        SETCLR10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr11(&self) -> SETCLR11R {
        SETCLR11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr12(&self) -> SETCLR12R {
        SETCLR12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr13(&self) -> SETCLR13R {
        SETCLR13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr14(&self) -> SETCLR14R {
        SETCLR14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr15(&self) -> SETCLR15R {
        SETCLR15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr0(&mut self) -> _SETCLR0W {
        _SETCLR0W { w: self }
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr1(&mut self) -> _SETCLR1W {
        _SETCLR1W { w: self }
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr2(&mut self) -> _SETCLR2W {
        _SETCLR2W { w: self }
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr3(&mut self) -> _SETCLR3W {
        _SETCLR3W { w: self }
    }
    #[doc = "Bits 8:9 - Set/clear operation on output 4. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr4(&mut self) -> _SETCLR4W {
        _SETCLR4W { w: self }
    }
    #[doc = "Bits 10:11 - Set/clear operation on output 5. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr5(&mut self) -> _SETCLR5W {
        _SETCLR5W { w: self }
    }
    #[doc = "Bits 12:13 - Set/clear operation on output 6. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr6(&mut self) -> _SETCLR6W {
        _SETCLR6W { w: self }
    }
    #[doc = "Bits 14:15 - Set/clear operation on output 7. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr7(&mut self) -> _SETCLR7W {
        _SETCLR7W { w: self }
    }
    #[doc = "Bits 16:17 - Set/clear operation on output 8. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr8(&mut self) -> _SETCLR8W {
        _SETCLR8W { w: self }
    }
    #[doc = "Bits 18:19 - Set/clear operation on output 9. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr9(&mut self) -> _SETCLR9W {
        _SETCLR9W { w: self }
    }
    #[doc = "Bits 20:21 - Set/clear operation on output 10. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr10(&mut self) -> _SETCLR10W {
        _SETCLR10W { w: self }
    }
    #[doc = "Bits 22:23 - Set/clear operation on output 11. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr11(&mut self) -> _SETCLR11W {
        _SETCLR11W { w: self }
    }
    #[doc = "Bits 24:25 - Set/clear operation on output 12. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr12(&mut self) -> _SETCLR12W {
        _SETCLR12W { w: self }
    }
    #[doc = "Bits 26:27 - Set/clear operation on output 13. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr13(&mut self) -> _SETCLR13W {
        _SETCLR13W { w: self }
    }
    #[doc = "Bits 28:29 - Set/clear operation on output 14. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr14(&mut self) -> _SETCLR14W {
        _SETCLR14W { w: self }
    }
    #[doc = "Bits 30:31 - Set/clear operation on output 15. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr15(&mut self) -> _SETCLR15W {
        _SETCLR15W { w: self }
    }
}
