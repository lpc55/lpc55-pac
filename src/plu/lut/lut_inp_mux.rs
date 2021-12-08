#[doc = "Register `LUT_INP_MUX%s` reader"]
pub struct R(crate::R<LUT_INP_MUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_INP_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_INP_MUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_INP_MUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT_INP_MUX%s` writer"]
pub struct W(crate::W<LUT_INP_MUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_INP_MUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LUT_INP_MUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_INP_MUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LUTN_INPX_A {
    #[doc = "0: The PLU primary inputs 0."]
    PLU_INPUTS0 = 0,
    #[doc = "1: The PLU primary inputs 1."]
    PLU_INPUTS1 = 1,
    #[doc = "2: The PLU primary inputs 2."]
    PLU_INPUTS2 = 2,
    #[doc = "3: The PLU primary inputs 3."]
    PLU_INPUTS3 = 3,
    #[doc = "4: The PLU primary inputs 4."]
    PLU_INPUTS4 = 4,
    #[doc = "5: The PLU primary inputs 5."]
    PLU_INPUTS5 = 5,
    #[doc = "6: The output of LUT0."]
    LUT_OUTPUTS0 = 6,
    #[doc = "7: The output of LUT1."]
    LUT_OUTPUTS1 = 7,
    #[doc = "8: The output of LUT2."]
    LUT_OUTPUTS2 = 8,
    #[doc = "9: The output of LUT3."]
    LUT_OUTPUTS3 = 9,
    #[doc = "10: The output of LUT4."]
    LUT_OUTPUTS4 = 10,
    #[doc = "11: The output of LUT5."]
    LUT_OUTPUTS5 = 11,
    #[doc = "12: The output of LUT6."]
    LUT_OUTPUTS6 = 12,
    #[doc = "13: The output of LUT7."]
    LUT_OUTPUTS7 = 13,
    #[doc = "14: The output of LUT8."]
    LUT_OUTPUTS8 = 14,
    #[doc = "15: The output of LUT9."]
    LUT_OUTPUTS9 = 15,
    #[doc = "16: The output of LUT10."]
    LUT_OUTPUTS10 = 16,
    #[doc = "17: The output of LUT11."]
    LUT_OUTPUTS11 = 17,
    #[doc = "18: The output of LUT12."]
    LUT_OUTPUTS12 = 18,
    #[doc = "19: The output of LUT13."]
    LUT_OUTPUTS13 = 19,
    #[doc = "20: The output of LUT14."]
    LUT_OUTPUTS14 = 20,
    #[doc = "21: The output of LUT15."]
    LUT_OUTPUTS15 = 21,
    #[doc = "22: The output of LUT16."]
    LUT_OUTPUTS16 = 22,
    #[doc = "23: The output of LUT17."]
    LUT_OUTPUTS17 = 23,
    #[doc = "24: The output of LUT18."]
    LUT_OUTPUTS18 = 24,
    #[doc = "25: The output of LUT19."]
    LUT_OUTPUTS19 = 25,
    #[doc = "26: The output of LUT20."]
    LUT_OUTPUTS20 = 26,
    #[doc = "27: The output of LUT21."]
    LUT_OUTPUTS21 = 27,
    #[doc = "28: The output of LUT22."]
    LUT_OUTPUTS22 = 28,
    #[doc = "29: The output of LUT23."]
    LUT_OUTPUTS23 = 29,
    #[doc = "30: The output of LUT24."]
    LUT_OUTPUTS24 = 30,
    #[doc = "31: The output of LUT25."]
    LUT_OUTPUTS25 = 31,
    #[doc = "32: state(0)."]
    STATE0 = 32,
    #[doc = "33: state(1)."]
    STATE1 = 33,
    #[doc = "34: state(2)."]
    STATE2 = 34,
    #[doc = "35: state(3)."]
    STATE3 = 35,
}
impl From<LUTN_INPX_A> for u8 {
    #[inline(always)]
    fn from(variant: LUTN_INPX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LUTn_INPx` reader - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
pub struct LUTN_INPX_R(crate::FieldReader<u8, LUTN_INPX_A>);
impl LUTN_INPX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LUTN_INPX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LUTN_INPX_A> {
        match self.bits {
            0 => Some(LUTN_INPX_A::PLU_INPUTS0),
            1 => Some(LUTN_INPX_A::PLU_INPUTS1),
            2 => Some(LUTN_INPX_A::PLU_INPUTS2),
            3 => Some(LUTN_INPX_A::PLU_INPUTS3),
            4 => Some(LUTN_INPX_A::PLU_INPUTS4),
            5 => Some(LUTN_INPX_A::PLU_INPUTS5),
            6 => Some(LUTN_INPX_A::LUT_OUTPUTS0),
            7 => Some(LUTN_INPX_A::LUT_OUTPUTS1),
            8 => Some(LUTN_INPX_A::LUT_OUTPUTS2),
            9 => Some(LUTN_INPX_A::LUT_OUTPUTS3),
            10 => Some(LUTN_INPX_A::LUT_OUTPUTS4),
            11 => Some(LUTN_INPX_A::LUT_OUTPUTS5),
            12 => Some(LUTN_INPX_A::LUT_OUTPUTS6),
            13 => Some(LUTN_INPX_A::LUT_OUTPUTS7),
            14 => Some(LUTN_INPX_A::LUT_OUTPUTS8),
            15 => Some(LUTN_INPX_A::LUT_OUTPUTS9),
            16 => Some(LUTN_INPX_A::LUT_OUTPUTS10),
            17 => Some(LUTN_INPX_A::LUT_OUTPUTS11),
            18 => Some(LUTN_INPX_A::LUT_OUTPUTS12),
            19 => Some(LUTN_INPX_A::LUT_OUTPUTS13),
            20 => Some(LUTN_INPX_A::LUT_OUTPUTS14),
            21 => Some(LUTN_INPX_A::LUT_OUTPUTS15),
            22 => Some(LUTN_INPX_A::LUT_OUTPUTS16),
            23 => Some(LUTN_INPX_A::LUT_OUTPUTS17),
            24 => Some(LUTN_INPX_A::LUT_OUTPUTS18),
            25 => Some(LUTN_INPX_A::LUT_OUTPUTS19),
            26 => Some(LUTN_INPX_A::LUT_OUTPUTS20),
            27 => Some(LUTN_INPX_A::LUT_OUTPUTS21),
            28 => Some(LUTN_INPX_A::LUT_OUTPUTS22),
            29 => Some(LUTN_INPX_A::LUT_OUTPUTS23),
            30 => Some(LUTN_INPX_A::LUT_OUTPUTS24),
            31 => Some(LUTN_INPX_A::LUT_OUTPUTS25),
            32 => Some(LUTN_INPX_A::STATE0),
            33 => Some(LUTN_INPX_A::STATE1),
            34 => Some(LUTN_INPX_A::STATE2),
            35 => Some(LUTN_INPX_A::STATE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS0`"]
    #[inline(always)]
    pub fn is_plu_inputs0(&self) -> bool {
        **self == LUTN_INPX_A::PLU_INPUTS0
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS1`"]
    #[inline(always)]
    pub fn is_plu_inputs1(&self) -> bool {
        **self == LUTN_INPX_A::PLU_INPUTS1
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS2`"]
    #[inline(always)]
    pub fn is_plu_inputs2(&self) -> bool {
        **self == LUTN_INPX_A::PLU_INPUTS2
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS3`"]
    #[inline(always)]
    pub fn is_plu_inputs3(&self) -> bool {
        **self == LUTN_INPX_A::PLU_INPUTS3
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS4`"]
    #[inline(always)]
    pub fn is_plu_inputs4(&self) -> bool {
        **self == LUTN_INPX_A::PLU_INPUTS4
    }
    #[doc = "Checks if the value of the field is `PLU_INPUTS5`"]
    #[inline(always)]
    pub fn is_plu_inputs5(&self) -> bool {
        **self == LUTN_INPX_A::PLU_INPUTS5
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS0`"]
    #[inline(always)]
    pub fn is_lut_outputs0(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS0
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS1`"]
    #[inline(always)]
    pub fn is_lut_outputs1(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS1
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS2`"]
    #[inline(always)]
    pub fn is_lut_outputs2(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS2
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS3`"]
    #[inline(always)]
    pub fn is_lut_outputs3(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS3
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS4`"]
    #[inline(always)]
    pub fn is_lut_outputs4(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS4
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS5`"]
    #[inline(always)]
    pub fn is_lut_outputs5(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS5
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS6`"]
    #[inline(always)]
    pub fn is_lut_outputs6(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS6
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS7`"]
    #[inline(always)]
    pub fn is_lut_outputs7(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS7
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS8`"]
    #[inline(always)]
    pub fn is_lut_outputs8(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS8
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS9`"]
    #[inline(always)]
    pub fn is_lut_outputs9(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS9
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS10`"]
    #[inline(always)]
    pub fn is_lut_outputs10(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS10
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS11`"]
    #[inline(always)]
    pub fn is_lut_outputs11(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS11
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS12`"]
    #[inline(always)]
    pub fn is_lut_outputs12(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS12
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS13`"]
    #[inline(always)]
    pub fn is_lut_outputs13(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS13
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS14`"]
    #[inline(always)]
    pub fn is_lut_outputs14(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS14
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS15`"]
    #[inline(always)]
    pub fn is_lut_outputs15(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS15
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS16`"]
    #[inline(always)]
    pub fn is_lut_outputs16(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS16
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS17`"]
    #[inline(always)]
    pub fn is_lut_outputs17(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS17
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS18`"]
    #[inline(always)]
    pub fn is_lut_outputs18(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS18
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS19`"]
    #[inline(always)]
    pub fn is_lut_outputs19(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS19
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS20`"]
    #[inline(always)]
    pub fn is_lut_outputs20(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS20
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS21`"]
    #[inline(always)]
    pub fn is_lut_outputs21(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS21
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS22`"]
    #[inline(always)]
    pub fn is_lut_outputs22(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS22
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS23`"]
    #[inline(always)]
    pub fn is_lut_outputs23(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS23
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS24`"]
    #[inline(always)]
    pub fn is_lut_outputs24(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS24
    }
    #[doc = "Checks if the value of the field is `LUT_OUTPUTS25`"]
    #[inline(always)]
    pub fn is_lut_outputs25(&self) -> bool {
        **self == LUTN_INPX_A::LUT_OUTPUTS25
    }
    #[doc = "Checks if the value of the field is `STATE0`"]
    #[inline(always)]
    pub fn is_state0(&self) -> bool {
        **self == LUTN_INPX_A::STATE0
    }
    #[doc = "Checks if the value of the field is `STATE1`"]
    #[inline(always)]
    pub fn is_state1(&self) -> bool {
        **self == LUTN_INPX_A::STATE1
    }
    #[doc = "Checks if the value of the field is `STATE2`"]
    #[inline(always)]
    pub fn is_state2(&self) -> bool {
        **self == LUTN_INPX_A::STATE2
    }
    #[doc = "Checks if the value of the field is `STATE3`"]
    #[inline(always)]
    pub fn is_state3(&self) -> bool {
        **self == LUTN_INPX_A::STATE3
    }
}
impl core::ops::Deref for LUTN_INPX_R {
    type Target = crate::FieldReader<u8, LUTN_INPX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LUTn_INPx` writer - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
pub struct LUTN_INPX_W<'a> {
    w: &'a mut W,
}
impl<'a> LUTN_INPX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LUTN_INPX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The PLU primary inputs 0."]
    #[inline(always)]
    pub fn plu_inputs0(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::PLU_INPUTS0)
    }
    #[doc = "The PLU primary inputs 1."]
    #[inline(always)]
    pub fn plu_inputs1(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::PLU_INPUTS1)
    }
    #[doc = "The PLU primary inputs 2."]
    #[inline(always)]
    pub fn plu_inputs2(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::PLU_INPUTS2)
    }
    #[doc = "The PLU primary inputs 3."]
    #[inline(always)]
    pub fn plu_inputs3(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::PLU_INPUTS3)
    }
    #[doc = "The PLU primary inputs 4."]
    #[inline(always)]
    pub fn plu_inputs4(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::PLU_INPUTS4)
    }
    #[doc = "The PLU primary inputs 5."]
    #[inline(always)]
    pub fn plu_inputs5(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::PLU_INPUTS5)
    }
    #[doc = "The output of LUT0."]
    #[inline(always)]
    pub fn lut_outputs0(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS0)
    }
    #[doc = "The output of LUT1."]
    #[inline(always)]
    pub fn lut_outputs1(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS1)
    }
    #[doc = "The output of LUT2."]
    #[inline(always)]
    pub fn lut_outputs2(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS2)
    }
    #[doc = "The output of LUT3."]
    #[inline(always)]
    pub fn lut_outputs3(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS3)
    }
    #[doc = "The output of LUT4."]
    #[inline(always)]
    pub fn lut_outputs4(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS4)
    }
    #[doc = "The output of LUT5."]
    #[inline(always)]
    pub fn lut_outputs5(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS5)
    }
    #[doc = "The output of LUT6."]
    #[inline(always)]
    pub fn lut_outputs6(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS6)
    }
    #[doc = "The output of LUT7."]
    #[inline(always)]
    pub fn lut_outputs7(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS7)
    }
    #[doc = "The output of LUT8."]
    #[inline(always)]
    pub fn lut_outputs8(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS8)
    }
    #[doc = "The output of LUT9."]
    #[inline(always)]
    pub fn lut_outputs9(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS9)
    }
    #[doc = "The output of LUT10."]
    #[inline(always)]
    pub fn lut_outputs10(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS10)
    }
    #[doc = "The output of LUT11."]
    #[inline(always)]
    pub fn lut_outputs11(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS11)
    }
    #[doc = "The output of LUT12."]
    #[inline(always)]
    pub fn lut_outputs12(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS12)
    }
    #[doc = "The output of LUT13."]
    #[inline(always)]
    pub fn lut_outputs13(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS13)
    }
    #[doc = "The output of LUT14."]
    #[inline(always)]
    pub fn lut_outputs14(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS14)
    }
    #[doc = "The output of LUT15."]
    #[inline(always)]
    pub fn lut_outputs15(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS15)
    }
    #[doc = "The output of LUT16."]
    #[inline(always)]
    pub fn lut_outputs16(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS16)
    }
    #[doc = "The output of LUT17."]
    #[inline(always)]
    pub fn lut_outputs17(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS17)
    }
    #[doc = "The output of LUT18."]
    #[inline(always)]
    pub fn lut_outputs18(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS18)
    }
    #[doc = "The output of LUT19."]
    #[inline(always)]
    pub fn lut_outputs19(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS19)
    }
    #[doc = "The output of LUT20."]
    #[inline(always)]
    pub fn lut_outputs20(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS20)
    }
    #[doc = "The output of LUT21."]
    #[inline(always)]
    pub fn lut_outputs21(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS21)
    }
    #[doc = "The output of LUT22."]
    #[inline(always)]
    pub fn lut_outputs22(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS22)
    }
    #[doc = "The output of LUT23."]
    #[inline(always)]
    pub fn lut_outputs23(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS23)
    }
    #[doc = "The output of LUT24."]
    #[inline(always)]
    pub fn lut_outputs24(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS24)
    }
    #[doc = "The output of LUT25."]
    #[inline(always)]
    pub fn lut_outputs25(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::LUT_OUTPUTS25)
    }
    #[doc = "state(0)."]
    #[inline(always)]
    pub fn state0(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::STATE0)
    }
    #[doc = "state(1)."]
    #[inline(always)]
    pub fn state1(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::STATE1)
    }
    #[doc = "state(2)."]
    #[inline(always)]
    pub fn state2(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::STATE2)
    }
    #[doc = "state(3)."]
    #[inline(always)]
    pub fn state3(self) -> &'a mut W {
        self.variant(LUTN_INPX_A::STATE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[inline(always)]
    pub fn lutn_inpx(&self) -> LUTN_INPX_R {
        LUTN_INPX_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selects the input source to be connected to LUT0 input0. For each LUT, the slot associated with the output from LUTn itself is tied low."]
    #[inline(always)]
    pub fn lutn_inpx(&mut self) -> LUTN_INPX_W {
        LUTN_INPX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUTn input x MUX\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_inp_mux](index.html) module"]
pub struct LUT_INP_MUX_SPEC;
impl crate::RegisterSpec for LUT_INP_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut_inp_mux::R](R) reader structure"]
impl crate::Readable for LUT_INP_MUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut_inp_mux::W](W) writer structure"]
impl crate::Writable for LUT_INP_MUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT_INP_MUX%s to value 0"]
impl crate::Resettable for LUT_INP_MUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
