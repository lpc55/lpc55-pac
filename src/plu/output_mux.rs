#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTPUT_MUX {
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
#[doc = "Possible values of the field `OUTPUTn`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUTNR {
    #[doc = "The PLU output 0."]
    PLU_OUTPUT0,
    #[doc = "The PLU output 1."]
    PLU_OUTPUT1,
    #[doc = "The PLU output 2."]
    PLU_OUTPUT2,
    #[doc = "The PLU output 3."]
    PLU_OUTPUT3,
    #[doc = "The PLU output 4."]
    PLU_OUTPUT4,
    #[doc = "The PLU output 5."]
    PLU_OUTPUT5,
    #[doc = "The PLU output 6."]
    PLU_OUTPUT6,
    #[doc = "The PLU output 7."]
    PLU_OUTPUT7,
    #[doc = "The PLU output 8."]
    PLU_OUTPUT8,
    #[doc = "The PLU output 9."]
    PLU_OUTPUT9,
    #[doc = "The PLU output 10."]
    PLU_OUTPUT10,
    #[doc = "The PLU output 11."]
    PLU_OUTPUT11,
    #[doc = "The PLU output 12."]
    PLU_OUTPUT12,
    #[doc = "The PLU output 13."]
    PLU_OUTPUT13,
    #[doc = "The PLU output 14."]
    PLU_OUTPUT14,
    #[doc = "The PLU output 15."]
    PLU_OUTPUT15,
    #[doc = "The PLU output 16."]
    PLU_OUTPUT16,
    #[doc = "The PLU output 17."]
    PLU_OUTPUT17,
    #[doc = "The PLU output 18."]
    PLU_OUTPUT18,
    #[doc = "The PLU output 19."]
    PLU_OUTPUT19,
    #[doc = "The PLU output 20."]
    PLU_OUTPUT20,
    #[doc = "The PLU output 21."]
    PLU_OUTPUT21,
    #[doc = "The PLU output 22."]
    PLU_OUTPUT22,
    #[doc = "The PLU output 23."]
    PLU_OUTPUT23,
    #[doc = "The PLU output 24."]
    PLU_OUTPUT24,
    #[doc = "The PLU output 25."]
    PLU_OUTPUT25,
    #[doc = "state(0)."]
    STATE0,
    #[doc = "state(1)."]
    STATE1,
    #[doc = "state(2)."]
    STATE2,
    #[doc = "state(3)."]
    STATE3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OUTPUTNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTPUTNR::PLU_OUTPUT0 => 0,
            OUTPUTNR::PLU_OUTPUT1 => 1,
            OUTPUTNR::PLU_OUTPUT2 => 2,
            OUTPUTNR::PLU_OUTPUT3 => 3,
            OUTPUTNR::PLU_OUTPUT4 => 4,
            OUTPUTNR::PLU_OUTPUT5 => 5,
            OUTPUTNR::PLU_OUTPUT6 => 6,
            OUTPUTNR::PLU_OUTPUT7 => 7,
            OUTPUTNR::PLU_OUTPUT8 => 8,
            OUTPUTNR::PLU_OUTPUT9 => 9,
            OUTPUTNR::PLU_OUTPUT10 => 10,
            OUTPUTNR::PLU_OUTPUT11 => 11,
            OUTPUTNR::PLU_OUTPUT12 => 12,
            OUTPUTNR::PLU_OUTPUT13 => 13,
            OUTPUTNR::PLU_OUTPUT14 => 14,
            OUTPUTNR::PLU_OUTPUT15 => 15,
            OUTPUTNR::PLU_OUTPUT16 => 16,
            OUTPUTNR::PLU_OUTPUT17 => 17,
            OUTPUTNR::PLU_OUTPUT18 => 18,
            OUTPUTNR::PLU_OUTPUT19 => 19,
            OUTPUTNR::PLU_OUTPUT20 => 20,
            OUTPUTNR::PLU_OUTPUT21 => 21,
            OUTPUTNR::PLU_OUTPUT22 => 22,
            OUTPUTNR::PLU_OUTPUT23 => 23,
            OUTPUTNR::PLU_OUTPUT24 => 24,
            OUTPUTNR::PLU_OUTPUT25 => 25,
            OUTPUTNR::STATE0 => 26,
            OUTPUTNR::STATE1 => 27,
            OUTPUTNR::STATE2 => 28,
            OUTPUTNR::STATE3 => 29,
            OUTPUTNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTPUTNR {
        match value {
            0 => OUTPUTNR::PLU_OUTPUT0,
            1 => OUTPUTNR::PLU_OUTPUT1,
            2 => OUTPUTNR::PLU_OUTPUT2,
            3 => OUTPUTNR::PLU_OUTPUT3,
            4 => OUTPUTNR::PLU_OUTPUT4,
            5 => OUTPUTNR::PLU_OUTPUT5,
            6 => OUTPUTNR::PLU_OUTPUT6,
            7 => OUTPUTNR::PLU_OUTPUT7,
            8 => OUTPUTNR::PLU_OUTPUT8,
            9 => OUTPUTNR::PLU_OUTPUT9,
            10 => OUTPUTNR::PLU_OUTPUT10,
            11 => OUTPUTNR::PLU_OUTPUT11,
            12 => OUTPUTNR::PLU_OUTPUT12,
            13 => OUTPUTNR::PLU_OUTPUT13,
            14 => OUTPUTNR::PLU_OUTPUT14,
            15 => OUTPUTNR::PLU_OUTPUT15,
            16 => OUTPUTNR::PLU_OUTPUT16,
            17 => OUTPUTNR::PLU_OUTPUT17,
            18 => OUTPUTNR::PLU_OUTPUT18,
            19 => OUTPUTNR::PLU_OUTPUT19,
            20 => OUTPUTNR::PLU_OUTPUT20,
            21 => OUTPUTNR::PLU_OUTPUT21,
            22 => OUTPUTNR::PLU_OUTPUT22,
            23 => OUTPUTNR::PLU_OUTPUT23,
            24 => OUTPUTNR::PLU_OUTPUT24,
            25 => OUTPUTNR::PLU_OUTPUT25,
            26 => OUTPUTNR::STATE0,
            27 => OUTPUTNR::STATE1,
            28 => OUTPUTNR::STATE2,
            29 => OUTPUTNR::STATE3,
            i => OUTPUTNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT0`"]
    #[inline]
    pub fn is_plu_output0(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT0
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT1`"]
    #[inline]
    pub fn is_plu_output1(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT1
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT2`"]
    #[inline]
    pub fn is_plu_output2(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT2
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT3`"]
    #[inline]
    pub fn is_plu_output3(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT3
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT4`"]
    #[inline]
    pub fn is_plu_output4(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT4
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT5`"]
    #[inline]
    pub fn is_plu_output5(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT5
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT6`"]
    #[inline]
    pub fn is_plu_output6(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT6
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT7`"]
    #[inline]
    pub fn is_plu_output7(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT7
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT8`"]
    #[inline]
    pub fn is_plu_output8(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT8
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT9`"]
    #[inline]
    pub fn is_plu_output9(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT9
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT10`"]
    #[inline]
    pub fn is_plu_output10(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT10
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT11`"]
    #[inline]
    pub fn is_plu_output11(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT11
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT12`"]
    #[inline]
    pub fn is_plu_output12(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT12
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT13`"]
    #[inline]
    pub fn is_plu_output13(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT13
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT14`"]
    #[inline]
    pub fn is_plu_output14(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT14
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT15`"]
    #[inline]
    pub fn is_plu_output15(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT15
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT16`"]
    #[inline]
    pub fn is_plu_output16(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT16
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT17`"]
    #[inline]
    pub fn is_plu_output17(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT17
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT18`"]
    #[inline]
    pub fn is_plu_output18(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT18
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT19`"]
    #[inline]
    pub fn is_plu_output19(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT19
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT20`"]
    #[inline]
    pub fn is_plu_output20(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT20
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT21`"]
    #[inline]
    pub fn is_plu_output21(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT21
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT22`"]
    #[inline]
    pub fn is_plu_output22(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT22
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT23`"]
    #[inline]
    pub fn is_plu_output23(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT23
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT24`"]
    #[inline]
    pub fn is_plu_output24(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT24
    }
    #[doc = "Checks if the value of the field is `PLU_OUTPUT25`"]
    #[inline]
    pub fn is_plu_output25(&self) -> bool {
        *self == OUTPUTNR::PLU_OUTPUT25
    }
    #[doc = "Checks if the value of the field is `STATE0`"]
    #[inline]
    pub fn is_state0(&self) -> bool {
        *self == OUTPUTNR::STATE0
    }
    #[doc = "Checks if the value of the field is `STATE1`"]
    #[inline]
    pub fn is_state1(&self) -> bool {
        *self == OUTPUTNR::STATE1
    }
    #[doc = "Checks if the value of the field is `STATE2`"]
    #[inline]
    pub fn is_state2(&self) -> bool {
        *self == OUTPUTNR::STATE2
    }
    #[doc = "Checks if the value of the field is `STATE3`"]
    #[inline]
    pub fn is_state3(&self) -> bool {
        *self == OUTPUTNR::STATE3
    }
}
#[doc = "Values that can be written to the field `OUTPUTn`"]
pub enum OUTPUTNW {
    #[doc = "The PLU output 0."]
    PLU_OUTPUT0,
    #[doc = "The PLU output 1."]
    PLU_OUTPUT1,
    #[doc = "The PLU output 2."]
    PLU_OUTPUT2,
    #[doc = "The PLU output 3."]
    PLU_OUTPUT3,
    #[doc = "The PLU output 4."]
    PLU_OUTPUT4,
    #[doc = "The PLU output 5."]
    PLU_OUTPUT5,
    #[doc = "The PLU output 6."]
    PLU_OUTPUT6,
    #[doc = "The PLU output 7."]
    PLU_OUTPUT7,
    #[doc = "The PLU output 8."]
    PLU_OUTPUT8,
    #[doc = "The PLU output 9."]
    PLU_OUTPUT9,
    #[doc = "The PLU output 10."]
    PLU_OUTPUT10,
    #[doc = "The PLU output 11."]
    PLU_OUTPUT11,
    #[doc = "The PLU output 12."]
    PLU_OUTPUT12,
    #[doc = "The PLU output 13."]
    PLU_OUTPUT13,
    #[doc = "The PLU output 14."]
    PLU_OUTPUT14,
    #[doc = "The PLU output 15."]
    PLU_OUTPUT15,
    #[doc = "The PLU output 16."]
    PLU_OUTPUT16,
    #[doc = "The PLU output 17."]
    PLU_OUTPUT17,
    #[doc = "The PLU output 18."]
    PLU_OUTPUT18,
    #[doc = "The PLU output 19."]
    PLU_OUTPUT19,
    #[doc = "The PLU output 20."]
    PLU_OUTPUT20,
    #[doc = "The PLU output 21."]
    PLU_OUTPUT21,
    #[doc = "The PLU output 22."]
    PLU_OUTPUT22,
    #[doc = "The PLU output 23."]
    PLU_OUTPUT23,
    #[doc = "The PLU output 24."]
    PLU_OUTPUT24,
    #[doc = "The PLU output 25."]
    PLU_OUTPUT25,
    #[doc = "state(0)."]
    STATE0,
    #[doc = "state(1)."]
    STATE1,
    #[doc = "state(2)."]
    STATE2,
    #[doc = "state(3)."]
    STATE3,
}
impl OUTPUTNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTPUTNW::PLU_OUTPUT0 => 0,
            OUTPUTNW::PLU_OUTPUT1 => 1,
            OUTPUTNW::PLU_OUTPUT2 => 2,
            OUTPUTNW::PLU_OUTPUT3 => 3,
            OUTPUTNW::PLU_OUTPUT4 => 4,
            OUTPUTNW::PLU_OUTPUT5 => 5,
            OUTPUTNW::PLU_OUTPUT6 => 6,
            OUTPUTNW::PLU_OUTPUT7 => 7,
            OUTPUTNW::PLU_OUTPUT8 => 8,
            OUTPUTNW::PLU_OUTPUT9 => 9,
            OUTPUTNW::PLU_OUTPUT10 => 10,
            OUTPUTNW::PLU_OUTPUT11 => 11,
            OUTPUTNW::PLU_OUTPUT12 => 12,
            OUTPUTNW::PLU_OUTPUT13 => 13,
            OUTPUTNW::PLU_OUTPUT14 => 14,
            OUTPUTNW::PLU_OUTPUT15 => 15,
            OUTPUTNW::PLU_OUTPUT16 => 16,
            OUTPUTNW::PLU_OUTPUT17 => 17,
            OUTPUTNW::PLU_OUTPUT18 => 18,
            OUTPUTNW::PLU_OUTPUT19 => 19,
            OUTPUTNW::PLU_OUTPUT20 => 20,
            OUTPUTNW::PLU_OUTPUT21 => 21,
            OUTPUTNW::PLU_OUTPUT22 => 22,
            OUTPUTNW::PLU_OUTPUT23 => 23,
            OUTPUTNW::PLU_OUTPUT24 => 24,
            OUTPUTNW::PLU_OUTPUT25 => 25,
            OUTPUTNW::STATE0 => 26,
            OUTPUTNW::STATE1 => 27,
            OUTPUTNW::STATE2 => 28,
            OUTPUTNW::STATE3 => 29,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTPUTNW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTPUTNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTPUTNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The PLU output 0."]
    #[inline]
    pub fn plu_output0(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT0)
    }
    #[doc = "The PLU output 1."]
    #[inline]
    pub fn plu_output1(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT1)
    }
    #[doc = "The PLU output 2."]
    #[inline]
    pub fn plu_output2(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT2)
    }
    #[doc = "The PLU output 3."]
    #[inline]
    pub fn plu_output3(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT3)
    }
    #[doc = "The PLU output 4."]
    #[inline]
    pub fn plu_output4(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT4)
    }
    #[doc = "The PLU output 5."]
    #[inline]
    pub fn plu_output5(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT5)
    }
    #[doc = "The PLU output 6."]
    #[inline]
    pub fn plu_output6(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT6)
    }
    #[doc = "The PLU output 7."]
    #[inline]
    pub fn plu_output7(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT7)
    }
    #[doc = "The PLU output 8."]
    #[inline]
    pub fn plu_output8(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT8)
    }
    #[doc = "The PLU output 9."]
    #[inline]
    pub fn plu_output9(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT9)
    }
    #[doc = "The PLU output 10."]
    #[inline]
    pub fn plu_output10(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT10)
    }
    #[doc = "The PLU output 11."]
    #[inline]
    pub fn plu_output11(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT11)
    }
    #[doc = "The PLU output 12."]
    #[inline]
    pub fn plu_output12(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT12)
    }
    #[doc = "The PLU output 13."]
    #[inline]
    pub fn plu_output13(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT13)
    }
    #[doc = "The PLU output 14."]
    #[inline]
    pub fn plu_output14(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT14)
    }
    #[doc = "The PLU output 15."]
    #[inline]
    pub fn plu_output15(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT15)
    }
    #[doc = "The PLU output 16."]
    #[inline]
    pub fn plu_output16(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT16)
    }
    #[doc = "The PLU output 17."]
    #[inline]
    pub fn plu_output17(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT17)
    }
    #[doc = "The PLU output 18."]
    #[inline]
    pub fn plu_output18(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT18)
    }
    #[doc = "The PLU output 19."]
    #[inline]
    pub fn plu_output19(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT19)
    }
    #[doc = "The PLU output 20."]
    #[inline]
    pub fn plu_output20(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT20)
    }
    #[doc = "The PLU output 21."]
    #[inline]
    pub fn plu_output21(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT21)
    }
    #[doc = "The PLU output 22."]
    #[inline]
    pub fn plu_output22(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT22)
    }
    #[doc = "The PLU output 23."]
    #[inline]
    pub fn plu_output23(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT23)
    }
    #[doc = "The PLU output 24."]
    #[inline]
    pub fn plu_output24(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT24)
    }
    #[doc = "The PLU output 25."]
    #[inline]
    pub fn plu_output25(self) -> &'a mut W {
        self.variant(OUTPUTNW::PLU_OUTPUT25)
    }
    #[doc = "state(0)."]
    #[inline]
    pub fn state0(self) -> &'a mut W {
        self.variant(OUTPUTNW::STATE0)
    }
    #[doc = "state(1)."]
    #[inline]
    pub fn state1(self) -> &'a mut W {
        self.variant(OUTPUTNW::STATE1)
    }
    #[doc = "state(2)."]
    #[inline]
    pub fn state2(self) -> &'a mut W {
        self.variant(OUTPUTNW::STATE2)
    }
    #[doc = "state(3)."]
    #[inline]
    pub fn state3(self) -> &'a mut W {
        self.variant(OUTPUTNW::STATE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:4 - Selects the source to be connected to PLU Output 0."]
    #[inline]
    pub fn outputn(&self) -> OUTPUTNR {
        OUTPUTNR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:4 - Selects the source to be connected to PLU Output 0."]
    #[inline]
    pub fn outputn(&mut self) -> _OUTPUTNW {
        _OUTPUTNW { w: self }
    }
}
