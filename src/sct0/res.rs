#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RES {
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
#[doc = "Possible values of the field `O0RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O0RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O0RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O0RESR::NO_CHANGE => 0,
            O0RESR::SET => 1,
            O0RESR::CLEAR => 2,
            O0RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O0RESR {
        match value {
            0 => O0RESR::NO_CHANGE,
            1 => O0RESR::SET,
            2 => O0RESR::CLEAR,
            3 => O0RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O0RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O0RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O0RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O1RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O1RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O1RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O1RESR::NO_CHANGE => 0,
            O1RESR::SET => 1,
            O1RESR::CLEAR => 2,
            O1RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O1RESR {
        match value {
            0 => O1RESR::NO_CHANGE,
            1 => O1RESR::SET,
            2 => O1RESR::CLEAR,
            3 => O1RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O1RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O1RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O1RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O2RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O2RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O2RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O2RESR::NO_CHANGE => 0,
            O2RESR::SET => 1,
            O2RESR::CLEAR => 2,
            O2RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O2RESR {
        match value {
            0 => O2RESR::NO_CHANGE,
            1 => O2RESR::SET,
            2 => O2RESR::CLEAR,
            3 => O2RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O2RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O2RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O2RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O3RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O3RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O3RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O3RESR::NO_CHANGE => 0,
            O3RESR::SET => 1,
            O3RESR::CLEAR => 2,
            O3RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O3RESR {
        match value {
            0 => O3RESR::NO_CHANGE,
            1 => O3RESR::SET,
            2 => O3RESR::CLEAR,
            3 => O3RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O3RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O3RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O3RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O4RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O4RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O4RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O4RESR::NO_CHANGE => 0,
            O4RESR::SET => 1,
            O4RESR::CLEAR => 2,
            O4RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O4RESR {
        match value {
            0 => O4RESR::NO_CHANGE,
            1 => O4RESR::SET,
            2 => O4RESR::CLEAR,
            3 => O4RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O4RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O4RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O4RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O4RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O5RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O5RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O5RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O5RESR::NO_CHANGE => 0,
            O5RESR::SET => 1,
            O5RESR::CLEAR => 2,
            O5RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O5RESR {
        match value {
            0 => O5RESR::NO_CHANGE,
            1 => O5RESR::SET,
            2 => O5RESR::CLEAR,
            3 => O5RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O5RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O5RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O5RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O5RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O6RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O6RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O6RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O6RESR::NO_CHANGE => 0,
            O6RESR::SET => 1,
            O6RESR::CLEAR => 2,
            O6RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O6RESR {
        match value {
            0 => O6RESR::NO_CHANGE,
            1 => O6RESR::SET,
            2 => O6RESR::CLEAR,
            3 => O6RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O6RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O6RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O6RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O6RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O7RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O7RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O7RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O7RESR::NO_CHANGE => 0,
            O7RESR::SET => 1,
            O7RESR::CLEAR => 2,
            O7RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O7RESR {
        match value {
            0 => O7RESR::NO_CHANGE,
            1 => O7RESR::SET,
            2 => O7RESR::CLEAR,
            3 => O7RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O7RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O7RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O7RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O7RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O8RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O8RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O8RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O8RESR::NO_CHANGE => 0,
            O8RESR::SET => 1,
            O8RESR::CLEAR => 2,
            O8RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O8RESR {
        match value {
            0 => O8RESR::NO_CHANGE,
            1 => O8RESR::SET,
            2 => O8RESR::CLEAR,
            3 => O8RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O8RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O8RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O8RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O8RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O9RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O9RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O9RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O9RESR::NO_CHANGE => 0,
            O9RESR::SET => 1,
            O9RESR::CLEAR => 2,
            O9RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O9RESR {
        match value {
            0 => O9RESR::NO_CHANGE,
            1 => O9RESR::SET,
            2 => O9RESR::CLEAR,
            3 => O9RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O9RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O9RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O9RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O9RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O10RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O10RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O10RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O10RESR::NO_CHANGE => 0,
            O10RESR::SET => 1,
            O10RESR::CLEAR => 2,
            O10RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O10RESR {
        match value {
            0 => O10RESR::NO_CHANGE,
            1 => O10RESR::SET,
            2 => O10RESR::CLEAR,
            3 => O10RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O10RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O10RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O10RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O10RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O11RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O11RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O11RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O11RESR::NO_CHANGE => 0,
            O11RESR::SET => 1,
            O11RESR::CLEAR => 2,
            O11RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O11RESR {
        match value {
            0 => O11RESR::NO_CHANGE,
            1 => O11RESR::SET,
            2 => O11RESR::CLEAR,
            3 => O11RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O11RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O11RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O11RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O11RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O12RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O12RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O12RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O12RESR::NO_CHANGE => 0,
            O12RESR::SET => 1,
            O12RESR::CLEAR => 2,
            O12RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O12RESR {
        match value {
            0 => O12RESR::NO_CHANGE,
            1 => O12RESR::SET,
            2 => O12RESR::CLEAR,
            3 => O12RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O12RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O12RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O12RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O12RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O13RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O13RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O13RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O13RESR::NO_CHANGE => 0,
            O13RESR::SET => 1,
            O13RESR::CLEAR => 2,
            O13RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O13RESR {
        match value {
            0 => O13RESR::NO_CHANGE,
            1 => O13RESR::SET,
            2 => O13RESR::CLEAR,
            3 => O13RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O13RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O13RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O13RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O13RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O14RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O14RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O14RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O14RESR::NO_CHANGE => 0,
            O14RESR::SET => 1,
            O14RESR::CLEAR => 2,
            O14RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O14RESR {
        match value {
            0 => O14RESR::NO_CHANGE,
            1 => O14RESR::SET,
            2 => O14RESR::CLEAR,
            3 => O14RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O14RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O14RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O14RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O14RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O15RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O15RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O15RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O15RESR::NO_CHANGE => 0,
            O15RESR::SET => 1,
            O15RESR::CLEAR => 2,
            O15RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O15RESR {
        match value {
            0 => O15RESR::NO_CHANGE,
            1 => O15RESR::SET,
            2 => O15RESR::CLEAR,
            3 => O15RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O15RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == O15RESR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == O15RESR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O15RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O0RES`"]
pub enum O0RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O0RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O0RESW::NO_CHANGE => 0,
            O0RESW::SET => 1,
            O0RESW::CLEAR => 2,
            O0RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O0RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O0RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O0RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O0RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O0RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O0RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O0RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O1RES`"]
pub enum O1RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O1RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O1RESW::NO_CHANGE => 0,
            O1RESW::SET => 1,
            O1RESW::CLEAR => 2,
            O1RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O1RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O1RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O1RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O1RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O1RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O1RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O1RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O2RES`"]
pub enum O2RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O2RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O2RESW::NO_CHANGE => 0,
            O2RESW::SET => 1,
            O2RESW::CLEAR => 2,
            O2RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O2RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O2RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O2RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O2RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O2RESW::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O2RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O2RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O3RES`"]
pub enum O3RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O3RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O3RESW::NO_CHANGE => 0,
            O3RESW::SET => 1,
            O3RESW::CLEAR => 2,
            O3RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O3RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O3RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O3RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O3RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O3RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O3RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O3RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O4RES`"]
pub enum O4RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O4RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O4RESW::NO_CHANGE => 0,
            O4RESW::SET => 1,
            O4RESW::CLEAR => 2,
            O4RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O4RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O4RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O4RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O4RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O4RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O4RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O4RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O5RES`"]
pub enum O5RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O5RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O5RESW::NO_CHANGE => 0,
            O5RESW::SET => 1,
            O5RESW::CLEAR => 2,
            O5RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O5RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O5RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O5RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O5RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O5RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O5RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O5RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O6RES`"]
pub enum O6RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O6RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O6RESW::NO_CHANGE => 0,
            O6RESW::SET => 1,
            O6RESW::CLEAR => 2,
            O6RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O6RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O6RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O6RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O6RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O6RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O6RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O6RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O7RES`"]
pub enum O7RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O7RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O7RESW::NO_CHANGE => 0,
            O7RESW::SET => 1,
            O7RESW::CLEAR => 2,
            O7RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O7RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O7RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O7RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O7RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O7RESW::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O7RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O7RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O8RES`"]
pub enum O8RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O8RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O8RESW::NO_CHANGE => 0,
            O8RESW::SET => 1,
            O8RESW::CLEAR => 2,
            O8RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O8RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O8RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O8RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O8RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O8RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O8RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O8RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O9RES`"]
pub enum O9RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O9RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O9RESW::NO_CHANGE => 0,
            O9RESW::SET => 1,
            O9RESW::CLEAR => 2,
            O9RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O9RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O9RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O9RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O9RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O9RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O9RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O9RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O10RES`"]
pub enum O10RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O10RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O10RESW::NO_CHANGE => 0,
            O10RESW::SET => 1,
            O10RESW::CLEAR => 2,
            O10RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O10RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O10RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O10RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O10RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O10RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O10RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O10RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O11RES`"]
pub enum O11RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O11RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O11RESW::NO_CHANGE => 0,
            O11RESW::SET => 1,
            O11RESW::CLEAR => 2,
            O11RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O11RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O11RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O11RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O11RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O11RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O11RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O11RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O12RES`"]
pub enum O12RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O12RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O12RESW::NO_CHANGE => 0,
            O12RESW::SET => 1,
            O12RESW::CLEAR => 2,
            O12RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O12RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O12RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O12RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O12RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O12RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O12RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O12RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O13RES`"]
pub enum O13RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O13RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O13RESW::NO_CHANGE => 0,
            O13RESW::SET => 1,
            O13RESW::CLEAR => 2,
            O13RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O13RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O13RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O13RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O13RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O13RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O13RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O13RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O14RES`"]
pub enum O14RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O14RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O14RESW::NO_CHANGE => 0,
            O14RESW::SET => 1,
            O14RESW::CLEAR => 2,
            O14RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O14RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O14RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O14RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O14RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O14RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O14RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O14RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O15RES`"]
pub enum O15RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET,
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    CLEAR,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O15RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O15RESW::NO_CHANGE => 0,
            O15RESW::SET => 1,
            O15RESW::CLEAR => 2,
            O15RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O15RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O15RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O15RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O15RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(O15RESW::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(O15RESW::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O15RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline]
    pub fn o0res(&self) -> O0RESR {
        O0RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline]
    pub fn o1res(&self) -> O1RESR {
        O1RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline]
    pub fn o2res(&self) -> O2RESR {
        O2RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline]
    pub fn o3res(&self) -> O3RESR {
        O3RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline]
    pub fn o4res(&self) -> O4RESR {
        O4RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline]
    pub fn o5res(&self) -> O5RESR {
        O5RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline]
    pub fn o6res(&self) -> O6RESR {
        O6RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline]
    pub fn o7res(&self) -> O7RESR {
        O7RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline]
    pub fn o8res(&self) -> O8RESR {
        O8RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline]
    pub fn o9res(&self) -> O9RESR {
        O9RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline]
    pub fn o10res(&self) -> O10RESR {
        O10RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline]
    pub fn o11res(&self) -> O11RESR {
        O11RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline]
    pub fn o12res(&self) -> O12RESR {
        O12RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline]
    pub fn o13res(&self) -> O13RESR {
        O13RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline]
    pub fn o14res(&self) -> O14RESR {
        O14RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline]
    pub fn o15res(&self) -> O15RESR {
        O15RESR::_from({
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
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline]
    pub fn o0res(&mut self) -> _O0RESW {
        _O0RESW { w: self }
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline]
    pub fn o1res(&mut self) -> _O1RESW {
        _O1RESW { w: self }
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline]
    pub fn o2res(&mut self) -> _O2RESW {
        _O2RESW { w: self }
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline]
    pub fn o3res(&mut self) -> _O3RESW {
        _O3RESW { w: self }
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline]
    pub fn o4res(&mut self) -> _O4RESW {
        _O4RESW { w: self }
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline]
    pub fn o5res(&mut self) -> _O5RESW {
        _O5RESW { w: self }
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline]
    pub fn o6res(&mut self) -> _O6RESW {
        _O6RESW { w: self }
    }
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline]
    pub fn o7res(&mut self) -> _O7RESW {
        _O7RESW { w: self }
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline]
    pub fn o8res(&mut self) -> _O8RESW {
        _O8RESW { w: self }
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline]
    pub fn o9res(&mut self) -> _O9RESW {
        _O9RESW { w: self }
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline]
    pub fn o10res(&mut self) -> _O10RESW {
        _O10RESW { w: self }
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline]
    pub fn o11res(&mut self) -> _O11RESW {
        _O11RESW { w: self }
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline]
    pub fn o12res(&mut self) -> _O12RESW {
        _O12RESW { w: self }
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline]
    pub fn o13res(&mut self) -> _O13RESW {
        _O13RESW { w: self }
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline]
    pub fn o14res(&mut self) -> _O14RESW {
        _O14RESW { w: self }
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline]
    pub fn o15res(&mut self) -> _O15RESW {
        _O15RESW { w: self }
    }
}
