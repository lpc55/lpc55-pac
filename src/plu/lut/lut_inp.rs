#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LUT_INP {
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
#[doc = "Possible values of the field `LUT_INP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LUT_INPR {
    #[doc = "The PLU primary inputs 0."]
    PLU_INPUTS0,
    #[doc = "The PLU primary inputs 1."]
    PLU_INPUTS1,
    #[doc = "The PLU primary inputs 2."]
    PLU_INPUTS2,
    #[doc = "The PLU primary inputs 3."]
    PLU_INPUTS3,
    #[doc = "The PLU primary inputs 4."]
    PLU_INPUTS4,
    #[doc = "The PLU primary inputs 5."]
    PLU_INPUTS5,
    #[doc = "The output of LUT0."]
    LUT_OUTPUTS0,
    #[doc = "The output of LUT1."]
    LUT_OUTPUTS1,
    #[doc = "The output of LUT2."]
    LUT_OUTPUTS2,
    #[doc = "The output of LUT3."]
    LUT_OUTPUTS3,
    #[doc = "The output of LUT4."]
    LUT_OUTPUTS4,
    #[doc = "The output of LUT5."]
    LUT_OUTPUTS5,
    #[doc = "The output of LUT6."]
    LUT_OUTPUTS6,
    #[doc = "The output of LUT7."]
    LUT_OUTPUTS7,
    #[doc = "The output of LUT8."]
    LUT_OUTPUTS8,
    #[doc = "The output of LUT9."]
    LUT_OUTPUTS9,
    #[doc = "The output of LUT10."]
    LUT_OUTPUTS10,
    #[doc = "The output of LUT11."]
    LUT_OUTPUTS11,
    #[doc = "The output of LUT12."]
    LUT_OUTPUTS12,
    #[doc = "The output of LUT13."]
    LUT_OUTPUTS13,
    #[doc = "The output of LUT14."]
    LUT_OUTPUTS14,
    #[doc = "The output of LUT15."]
    LUT_OUTPUTS15,
    #[doc = "The output of LUT16."]
    LUT_OUTPUTS16,
    #[doc = "The output of LUT17."]
    LUT_OUTPUTS17,
    #[doc = "The output of LUT18."]
    LUT_OUTPUTS18,
    #[doc = "The output of LUT19."]
    LUT_OUTPUTS19,
    #[doc = "The output of LUT20."]
    LUT_OUTPUTS20,
    #[doc = "The output of LUT21."]
    LUT_OUTPUTS21,
    #[doc = "The output of LUT22."]
    LUT_OUTPUTS22,
    #[doc = "The output of LUT23."]
    LUT_OUTPUTS23,
    #[doc = "The output of LUT24."]
    LUT_OUTPUTS24,
    #[doc = "The output of LUT25."]
    LUT_OUTPUTS25,
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
impl LUT_INPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LUT_INPR::PLU_INPUTS0 => 0,
            LUT_INPR::PLU_INPUTS1 => 1,
            LUT_INPR::PLU_INPUTS2 => 2,
            LUT_INPR::PLU_INPUTS3 => 3,
            LUT_INPR::PLU_INPUTS4 => 4,
            LUT_INPR::PLU_INPUTS5 => 5,
            LUT_INPR::LUT_OUTPUTS0 => 6,
            LUT_INPR::LUT_OUTPUTS1 => 7,
            LUT_INPR::LUT_OUTPUTS2 => 8,
            LUT_INPR::LUT_OUTPUTS3 => 9,
            LUT_INPR::LUT_OUTPUTS4 => 10,
            LUT_INPR::LUT_OUTPUTS5 => 11,
            LUT_INPR::LUT_OUTPUTS6 => 12,
            LUT_INPR::LUT_OUTPUTS7 => 13,
            LUT_INPR::LUT_OUTPUTS8 => 14,
            LUT_INPR::LUT_OUTPUTS9 => 15,
            LUT_INPR::LUT_OUTPUTS10 => 16,
            LUT_INPR::LUT_OUTPUTS11 => 17,
            LUT_INPR::LUT_OUTPUTS12 => 18,
            LUT_INPR::LUT_OUTPUTS13 => 19,
            LUT_INPR::LUT_OUTPUTS14 => 20,
            LUT_INPR::LUT_OUTPUTS15 => 21,
            LUT_INPR::LUT_OUTPUTS16 => 22,
            LUT_INPR::LUT_OUTPUTS17 => 23,
            LUT_INPR::LUT_OUTPUTS18 => 24,
            LUT_INPR::LUT_OUTPUTS19 => 25,
            LUT_INPR::LUT_OUTPUTS20 => 26,
            LUT_INPR::LUT_OUTPUTS21 => 27,
            LUT_INPR::LUT_OUTPUTS22 => 28,
            LUT_INPR::LUT_OUTPUTS23 => 29,
            LUT_INPR::LUT_OUTPUTS24 => 30,
            LUT_INPR::LUT_OUTPUTS25 => 31,
            LUT_INPR::STATE0 => 32,
            LUT_INPR::STATE1 => 33,
            LUT_INPR::STATE2 => 34,
            LUT_INPR::STATE3 => 35,
            LUT_INPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LUT_INPR {
        match value {
            0 => LUT_INPR::PLU_INPUTS0,
            1 => LUT_INPR::PLU_INPUTS1,
            2 => LUT_INPR::PLU_INPUTS2,
            3 => LUT_INPR::PLU_INPUTS3,
            4 => LUT_INPR::PLU_INPUTS4,
            5 => LUT_INPR::PLU_INPUTS5,
            6 => LUT_INPR::LUT_OUTPUTS0,
            7 => LUT_INPR::LUT_OUTPUTS1,
            8 => LUT_INPR::LUT_OUTPUTS2,
            9 => LUT_INPR::LUT_OUTPUTS3,
            10 => LUT_INPR::LUT_OUTPUTS4,
            11 => LUT_INPR::LUT_OUTPUTS5,
            12 => LUT_INPR::LUT_OUTPUTS6,
            13 => LUT_INPR::LUT_OUTPUTS7,
            14 => LUT_INPR::LUT_OUTPUTS8,
            15 => LUT_INPR::LUT_OUTPUTS9,
            16 => LUT_INPR::LUT_OUTPUTS10,
            17 => LUT_INPR::LUT_OUTPUTS11,
            18 => LUT_INPR::LUT_OUTPUTS12,
            19 => LUT_INPR::LUT_OUTPUTS13,
            20 => LUT_INPR::LUT_OUTPUTS14,
            21 => LUT_INPR::LUT_OUTPUTS15,
            22 => LUT_INPR::LUT_OUTPUTS16,
            23 => LUT_INPR::LUT_OUTPUTS17,
            24 => LUT_INPR::LUT_OUTPUTS18,
            25 => LUT_INPR::LUT_OUTPUTS19,
            26 => LUT_INPR::LUT_OUTPUTS20,
            27 => LUT_INPR::LUT_OUTPUTS21,
            28 => LUT_INPR::LUT_OUTPUTS22,
            29 => LUT_INPR::LUT_OUTPUTS23,
            30 => LUT_INPR::LUT_OUTPUTS24,
            31 => LUT_INPR::LUT_OUTPUTS25,
            32 => LUT_INPR::STATE0,
            33 => LUT_INPR::STATE1,
            34 => LUT_INPR::STATE2,
            35 => LUT_INPR::STATE3,
            i => LUT_INPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS0`"]
    #[inline]
    pub fn is_plu_inputs0(&self) -> bool {
        *self == LUT_INPR::PLU_INPUTS0
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS1`"]
    #[inline]
    pub fn is_plu_inputs1(&self) -> bool {
        *self == LUT_INPR::PLU_INPUTS1
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS2`"]
    #[inline]
    pub fn is_plu_inputs2(&self) -> bool {
        *self == LUT_INPR::PLU_INPUTS2
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS3`"]
    #[inline]
    pub fn is_plu_inputs3(&self) -> bool {
        *self == LUT_INPR::PLU_INPUTS3
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS4`"]
    #[inline]
    pub fn is_plu_inputs4(&self) -> bool {
        *self == LUT_INPR::PLU_INPUTS4
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS5`"]
    #[inline]
    pub fn is_plu_inputs5(&self) -> bool {
        *self == LUT_INPR::PLU_INPUTS5
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS0`"]
    #[inline]
    pub fn is_lut_outputs0(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS0
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS1`"]
    #[inline]
    pub fn is_lut_outputs1(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS1
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS2`"]
    #[inline]
    pub fn is_lut_outputs2(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS2
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS3`"]
    #[inline]
    pub fn is_lut_outputs3(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS3
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS4`"]
    #[inline]
    pub fn is_lut_outputs4(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS4
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS5`"]
    #[inline]
    pub fn is_lut_outputs5(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS5
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS6`"]
    #[inline]
    pub fn is_lut_outputs6(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS6
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS7`"]
    #[inline]
    pub fn is_lut_outputs7(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS7
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS8`"]
    #[inline]
    pub fn is_lut_outputs8(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS8
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS9`"]
    #[inline]
    pub fn is_lut_outputs9(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS9
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS10`"]
    #[inline]
    pub fn is_lut_outputs10(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS10
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS11`"]
    #[inline]
    pub fn is_lut_outputs11(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS11
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS12`"]
    #[inline]
    pub fn is_lut_outputs12(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS12
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS13`"]
    #[inline]
    pub fn is_lut_outputs13(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS13
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS14`"]
    #[inline]
    pub fn is_lut_outputs14(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS14
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS15`"]
    #[inline]
    pub fn is_lut_outputs15(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS15
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS16`"]
    #[inline]
    pub fn is_lut_outputs16(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS16
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS17`"]
    #[inline]
    pub fn is_lut_outputs17(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS17
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS18`"]
    #[inline]
    pub fn is_lut_outputs18(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS18
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS19`"]
    #[inline]
    pub fn is_lut_outputs19(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS19
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS20`"]
    #[inline]
    pub fn is_lut_outputs20(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS20
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS21`"]
    #[inline]
    pub fn is_lut_outputs21(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS21
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS22`"]
    #[inline]
    pub fn is_lut_outputs22(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS22
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS23`"]
    #[inline]
    pub fn is_lut_outputs23(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS23
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS24`"]
    #[inline]
    pub fn is_lut_outputs24(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS24
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS25`"]
    #[inline]
    pub fn is_lut_outputs25(&self) -> bool {
        *self == LUT_INPR::LUT_OUTPUTS25
    }
    #[doc = "Checks if the value of the field is `STATE0`"]
    #[inline]
    pub fn is_state0(&self) -> bool {
        *self == LUT_INPR::STATE0
    }
    #[doc = "Checks if the value of the field is `STATE1`"]
    #[inline]
    pub fn is_state1(&self) -> bool {
        *self == LUT_INPR::STATE1
    }
    #[doc = "Checks if the value of the field is `STATE2`"]
    #[inline]
    pub fn is_state2(&self) -> bool {
        *self == LUT_INPR::STATE2
    }
    #[doc = "Checks if the value of the field is `STATE3`"]
    #[inline]
    pub fn is_state3(&self) -> bool {
        *self == LUT_INPR::STATE3
    }
}
#[doc = "Values that can be written to the field `LUT_INP`"]
pub enum LUT_INPW {
    #[doc = "The PLU primary inputs 0."]
    PLU_INPUTS0,
    #[doc = "The PLU primary inputs 1."]
    PLU_INPUTS1,
    #[doc = "The PLU primary inputs 2."]
    PLU_INPUTS2,
    #[doc = "The PLU primary inputs 3."]
    PLU_INPUTS3,
    #[doc = "The PLU primary inputs 4."]
    PLU_INPUTS4,
    #[doc = "The PLU primary inputs 5."]
    PLU_INPUTS5,
    #[doc = "The output of LUT0."]
    LUT_OUTPUTS0,
    #[doc = "The output of LUT1."]
    LUT_OUTPUTS1,
    #[doc = "The output of LUT2."]
    LUT_OUTPUTS2,
    #[doc = "The output of LUT3."]
    LUT_OUTPUTS3,
    #[doc = "The output of LUT4."]
    LUT_OUTPUTS4,
    #[doc = "The output of LUT5."]
    LUT_OUTPUTS5,
    #[doc = "The output of LUT6."]
    LUT_OUTPUTS6,
    #[doc = "The output of LUT7."]
    LUT_OUTPUTS7,
    #[doc = "The output of LUT8."]
    LUT_OUTPUTS8,
    #[doc = "The output of LUT9."]
    LUT_OUTPUTS9,
    #[doc = "The output of LUT10."]
    LUT_OUTPUTS10,
    #[doc = "The output of LUT11."]
    LUT_OUTPUTS11,
    #[doc = "The output of LUT12."]
    LUT_OUTPUTS12,
    #[doc = "The output of LUT13."]
    LUT_OUTPUTS13,
    #[doc = "The output of LUT14."]
    LUT_OUTPUTS14,
    #[doc = "The output of LUT15."]
    LUT_OUTPUTS15,
    #[doc = "The output of LUT16."]
    LUT_OUTPUTS16,
    #[doc = "The output of LUT17."]
    LUT_OUTPUTS17,
    #[doc = "The output of LUT18."]
    LUT_OUTPUTS18,
    #[doc = "The output of LUT19."]
    LUT_OUTPUTS19,
    #[doc = "The output of LUT20."]
    LUT_OUTPUTS20,
    #[doc = "The output of LUT21."]
    LUT_OUTPUTS21,
    #[doc = "The output of LUT22."]
    LUT_OUTPUTS22,
    #[doc = "The output of LUT23."]
    LUT_OUTPUTS23,
    #[doc = "The output of LUT24."]
    LUT_OUTPUTS24,
    #[doc = "The output of LUT25."]
    LUT_OUTPUTS25,
    #[doc = "state(0)."]
    STATE0,
    #[doc = "state(1)."]
    STATE1,
    #[doc = "state(2)."]
    STATE2,
    #[doc = "state(3)."]
    STATE3,
}
impl LUT_INPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LUT_INPW::PLU_INPUTS0 => 0,
            LUT_INPW::PLU_INPUTS1 => 1,
            LUT_INPW::PLU_INPUTS2 => 2,
            LUT_INPW::PLU_INPUTS3 => 3,
            LUT_INPW::PLU_INPUTS4 => 4,
            LUT_INPW::PLU_INPUTS5 => 5,
            LUT_INPW::LUT_OUTPUTS0 => 6,
            LUT_INPW::LUT_OUTPUTS1 => 7,
            LUT_INPW::LUT_OUTPUTS2 => 8,
            LUT_INPW::LUT_OUTPUTS3 => 9,
            LUT_INPW::LUT_OUTPUTS4 => 10,
            LUT_INPW::LUT_OUTPUTS5 => 11,
            LUT_INPW::LUT_OUTPUTS6 => 12,
            LUT_INPW::LUT_OUTPUTS7 => 13,
            LUT_INPW::LUT_OUTPUTS8 => 14,
            LUT_INPW::LUT_OUTPUTS9 => 15,
            LUT_INPW::LUT_OUTPUTS10 => 16,
            LUT_INPW::LUT_OUTPUTS11 => 17,
            LUT_INPW::LUT_OUTPUTS12 => 18,
            LUT_INPW::LUT_OUTPUTS13 => 19,
            LUT_INPW::LUT_OUTPUTS14 => 20,
            LUT_INPW::LUT_OUTPUTS15 => 21,
            LUT_INPW::LUT_OUTPUTS16 => 22,
            LUT_INPW::LUT_OUTPUTS17 => 23,
            LUT_INPW::LUT_OUTPUTS18 => 24,
            LUT_INPW::LUT_OUTPUTS19 => 25,
            LUT_INPW::LUT_OUTPUTS20 => 26,
            LUT_INPW::LUT_OUTPUTS21 => 27,
            LUT_INPW::LUT_OUTPUTS22 => 28,
            LUT_INPW::LUT_OUTPUTS23 => 29,
            LUT_INPW::LUT_OUTPUTS24 => 30,
            LUT_INPW::LUT_OUTPUTS25 => 31,
            LUT_INPW::STATE0 => 32,
            LUT_INPW::STATE1 => 33,
            LUT_INPW::STATE2 => 34,
            LUT_INPW::STATE3 => 35,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LUT_INPW<'a> {
    w: &'a mut W,
}
impl<'a> _LUT_INPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LUT_INPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The PLU primary inputs 0."]
    #[inline]
    pub fn plu_inputs0(self) -> &'a mut W {
        self.variant(LUT_INPW::PLU_INPUTS0)
    }
    #[doc = "The PLU primary inputs 1."]
    #[inline]
    pub fn plu_inputs1(self) -> &'a mut W {
        self.variant(LUT_INPW::PLU_INPUTS1)
    }
    #[doc = "The PLU primary inputs 2."]
    #[inline]
    pub fn plu_inputs2(self) -> &'a mut W {
        self.variant(LUT_INPW::PLU_INPUTS2)
    }
    #[doc = "The PLU primary inputs 3."]
    #[inline]
    pub fn plu_inputs3(self) -> &'a mut W {
        self.variant(LUT_INPW::PLU_INPUTS3)
    }
    #[doc = "The PLU primary inputs 4."]
    #[inline]
    pub fn plu_inputs4(self) -> &'a mut W {
        self.variant(LUT_INPW::PLU_INPUTS4)
    }
    #[doc = "The PLU primary inputs 5."]
    #[inline]
    pub fn plu_inputs5(self) -> &'a mut W {
        self.variant(LUT_INPW::PLU_INPUTS5)
    }
    #[doc = "The output of LUT0."]
    #[inline]
    pub fn lut_outputs0(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS0)
    }
    #[doc = "The output of LUT1."]
    #[inline]
    pub fn lut_outputs1(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS1)
    }
    #[doc = "The output of LUT2."]
    #[inline]
    pub fn lut_outputs2(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS2)
    }
    #[doc = "The output of LUT3."]
    #[inline]
    pub fn lut_outputs3(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS3)
    }
    #[doc = "The output of LUT4."]
    #[inline]
    pub fn lut_outputs4(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS4)
    }
    #[doc = "The output of LUT5."]
    #[inline]
    pub fn lut_outputs5(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS5)
    }
    #[doc = "The output of LUT6."]
    #[inline]
    pub fn lut_outputs6(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS6)
    }
    #[doc = "The output of LUT7."]
    #[inline]
    pub fn lut_outputs7(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS7)
    }
    #[doc = "The output of LUT8."]
    #[inline]
    pub fn lut_outputs8(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS8)
    }
    #[doc = "The output of LUT9."]
    #[inline]
    pub fn lut_outputs9(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS9)
    }
    #[doc = "The output of LUT10."]
    #[inline]
    pub fn lut_outputs10(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS10)
    }
    #[doc = "The output of LUT11."]
    #[inline]
    pub fn lut_outputs11(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS11)
    }
    #[doc = "The output of LUT12."]
    #[inline]
    pub fn lut_outputs12(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS12)
    }
    #[doc = "The output of LUT13."]
    #[inline]
    pub fn lut_outputs13(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS13)
    }
    #[doc = "The output of LUT14."]
    #[inline]
    pub fn lut_outputs14(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS14)
    }
    #[doc = "The output of LUT15."]
    #[inline]
    pub fn lut_outputs15(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS15)
    }
    #[doc = "The output of LUT16."]
    #[inline]
    pub fn lut_outputs16(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS16)
    }
    #[doc = "The output of LUT17."]
    #[inline]
    pub fn lut_outputs17(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS17)
    }
    #[doc = "The output of LUT18."]
    #[inline]
    pub fn lut_outputs18(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS18)
    }
    #[doc = "The output of LUT19."]
    #[inline]
    pub fn lut_outputs19(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS19)
    }
    #[doc = "The output of LUT20."]
    #[inline]
    pub fn lut_outputs20(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS20)
    }
    #[doc = "The output of LUT21."]
    #[inline]
    pub fn lut_outputs21(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS21)
    }
    #[doc = "The output of LUT22."]
    #[inline]
    pub fn lut_outputs22(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS22)
    }
    #[doc = "The output of LUT23."]
    #[inline]
    pub fn lut_outputs23(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS23)
    }
    #[doc = "The output of LUT24."]
    #[inline]
    pub fn lut_outputs24(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS24)
    }
    #[doc = "The output of LUT25."]
    #[inline]
    pub fn lut_outputs25(self) -> &'a mut W {
        self.variant(LUT_INPW::LUT_OUTPUTS25)
    }
    #[doc = "state(0)."]
    #[inline]
    pub fn state0(self) -> &'a mut W {
        self.variant(LUT_INPW::STATE0)
    }
    #[doc = "state(1)."]
    #[inline]
    pub fn state1(self) -> &'a mut W {
        self.variant(LUT_INPW::STATE1)
    }
    #[doc = "state(2)."]
    #[inline]
    pub fn state2(self) -> &'a mut W {
        self.variant(LUT_INPW::STATE2)
    }
    #[doc = "state(3)."]
    #[inline]
    pub fn state3(self) -> &'a mut W {
        self.variant(LUT_INPW::STATE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0."]
    #[inline]
    pub fn lut_inp(&self) -> LUT_INPR {
        LUT_INPR::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0."]
    #[inline]
    pub fn lut_inp(&mut self) -> _LUT_INPW {
        _LUT_INPW { w: self }
    }
}
