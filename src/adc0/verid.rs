#[doc = "Reader of register VERID"]
pub type R = crate::R<u32, super::VERID>;
#[doc = "Resolution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RES_A {
    #[doc = "0: Up to 13-bit differential/12-bit single ended resolution supported."]
    RES_0 = 0,
    #[doc = "1: Up to 16-bit differential/16-bit single ended resolution supported."]
    RES_1 = 1,
}
impl From<RES_A> for bool {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, RES_A>;
impl RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            false => RES_A::RES_0,
            true => RES_A::RES_1,
        }
    }
    #[doc = "Checks if the value of the field is `RES_0`"]
    #[inline(always)]
    pub fn is_res_0(&self) -> bool {
        *self == RES_A::RES_0
    }
    #[doc = "Checks if the value of the field is `RES_1`"]
    #[inline(always)]
    pub fn is_res_1(&self) -> bool {
        *self == RES_A::RES_1
    }
}
#[doc = "Differential Supported\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFEN_A {
    #[doc = "0: Differential operation not supported."]
    DIFFEN_0 = 0,
    #[doc = "1: Differential operation supported. CMDLa\\[CTYPE\\]
controls fields implemented."]
    DIFFEN_1 = 1,
}
impl From<DIFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIFFEN`"]
pub type DIFFEN_R = crate::R<bool, DIFFEN_A>;
impl DIFFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFEN_A {
        match self.bits {
            false => DIFFEN_A::DIFFEN_0,
            true => DIFFEN_A::DIFFEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIFFEN_0`"]
    #[inline(always)]
    pub fn is_diffen_0(&self) -> bool {
        *self == DIFFEN_A::DIFFEN_0
    }
    #[doc = "Checks if the value of the field is `DIFFEN_1`"]
    #[inline(always)]
    pub fn is_diffen_1(&self) -> bool {
        *self == DIFFEN_A::DIFFEN_1
    }
}
#[doc = "Multi Vref Implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MVI_A {
    #[doc = "0: Single voltage reference high (VREFH) input supported."]
    MVI_0 = 0,
    #[doc = "1: Multiple voltage reference high (VREFH) inputs supported."]
    MVI_1 = 1,
}
impl From<MVI_A> for bool {
    #[inline(always)]
    fn from(variant: MVI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MVI`"]
pub type MVI_R = crate::R<bool, MVI_A>;
impl MVI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MVI_A {
        match self.bits {
            false => MVI_A::MVI_0,
            true => MVI_A::MVI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MVI_0`"]
    #[inline(always)]
    pub fn is_mvi_0(&self) -> bool {
        *self == MVI_A::MVI_0
    }
    #[doc = "Checks if the value of the field is `MVI_1`"]
    #[inline(always)]
    pub fn is_mvi_1(&self) -> bool {
        *self == MVI_A::MVI_1
    }
}
#[doc = "Channel Scale Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSW_A {
    #[doc = "0: Channel scaling not supported."]
    CSW_0 = 0,
    #[doc = "1: Channel scaling supported. 1-bit CSCALE control field."]
    CSW_1 = 1,
    #[doc = "6: Channel scaling supported. 6-bit CSCALE control field."]
    CSW_6 = 6,
}
impl From<CSW_A> for u8 {
    #[inline(always)]
    fn from(variant: CSW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSW`"]
pub type CSW_R = crate::R<u8, CSW_A>;
impl CSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSW_A::CSW_0),
            1 => Val(CSW_A::CSW_1),
            6 => Val(CSW_A::CSW_6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CSW_0`"]
    #[inline(always)]
    pub fn is_csw_0(&self) -> bool {
        *self == CSW_A::CSW_0
    }
    #[doc = "Checks if the value of the field is `CSW_1`"]
    #[inline(always)]
    pub fn is_csw_1(&self) -> bool {
        *self == CSW_A::CSW_1
    }
    #[doc = "Checks if the value of the field is `CSW_6`"]
    #[inline(always)]
    pub fn is_csw_6(&self) -> bool {
        *self == CSW_A::CSW_6
    }
}
#[doc = "Voltage Reference 1 Range Control Bit Implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR1RNGI_A {
    #[doc = "0: Range control not required. CFG\\[VREF1RNG\\]
is not implemented."]
    VR1RNGI_0 = 0,
    #[doc = "1: Range control required. CFG\\[VREF1RNG\\]
is implemented."]
    VR1RNGI_1 = 1,
}
impl From<VR1RNGI_A> for bool {
    #[inline(always)]
    fn from(variant: VR1RNGI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VR1RNGI`"]
pub type VR1RNGI_R = crate::R<bool, VR1RNGI_A>;
impl VR1RNGI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VR1RNGI_A {
        match self.bits {
            false => VR1RNGI_A::VR1RNGI_0,
            true => VR1RNGI_A::VR1RNGI_1,
        }
    }
    #[doc = "Checks if the value of the field is `VR1RNGI_0`"]
    #[inline(always)]
    pub fn is_vr1rngi_0(&self) -> bool {
        *self == VR1RNGI_A::VR1RNGI_0
    }
    #[doc = "Checks if the value of the field is `VR1RNGI_1`"]
    #[inline(always)]
    pub fn is_vr1rngi_1(&self) -> bool {
        *self == VR1RNGI_A::VR1RNGI_1
    }
}
#[doc = "Internal ADC Clock implemented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IADCKI_A {
    #[doc = "0: Internal clock source not implemented."]
    IADCKI_0 = 0,
    #[doc = "1: Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    IADCKI_1 = 1,
}
impl From<IADCKI_A> for bool {
    #[inline(always)]
    fn from(variant: IADCKI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IADCKI`"]
pub type IADCKI_R = crate::R<bool, IADCKI_A>;
impl IADCKI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IADCKI_A {
        match self.bits {
            false => IADCKI_A::IADCKI_0,
            true => IADCKI_A::IADCKI_1,
        }
    }
    #[doc = "Checks if the value of the field is `IADCKI_0`"]
    #[inline(always)]
    pub fn is_iadcki_0(&self) -> bool {
        *self == IADCKI_A::IADCKI_0
    }
    #[doc = "Checks if the value of the field is `IADCKI_1`"]
    #[inline(always)]
    pub fn is_iadcki_1(&self) -> bool {
        *self == IADCKI_A::IADCKI_1
    }
}
#[doc = "Calibration Function Implemented\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALOFSI_A {
    #[doc = "0: Calibration Not Implemented."]
    CALOFSI_0 = 0,
    #[doc = "1: Calibration Implemented."]
    CALOFSI_1 = 1,
}
impl From<CALOFSI_A> for bool {
    #[inline(always)]
    fn from(variant: CALOFSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALOFSI`"]
pub type CALOFSI_R = crate::R<bool, CALOFSI_A>;
impl CALOFSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALOFSI_A {
        match self.bits {
            false => CALOFSI_A::CALOFSI_0,
            true => CALOFSI_A::CALOFSI_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALOFSI_0`"]
    #[inline(always)]
    pub fn is_calofsi_0(&self) -> bool {
        *self == CALOFSI_A::CALOFSI_0
    }
    #[doc = "Checks if the value of the field is `CALOFSI_1`"]
    #[inline(always)]
    pub fn is_calofsi_1(&self) -> bool {
        *self == CALOFSI_A::CALOFSI_1
    }
}
#[doc = "Number of Single Ended Outputs Supported\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUM_SEC_A {
    #[doc = "0: This design supports one single ended conversion at a time."]
    NUM_SEC_0 = 0,
    #[doc = "1: This design supports two simultanious single ended conversions."]
    NUM_SEC_1 = 1,
}
impl From<NUM_SEC_A> for bool {
    #[inline(always)]
    fn from(variant: NUM_SEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NUM_SEC`"]
pub type NUM_SEC_R = crate::R<bool, NUM_SEC_A>;
impl NUM_SEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUM_SEC_A {
        match self.bits {
            false => NUM_SEC_A::NUM_SEC_0,
            true => NUM_SEC_A::NUM_SEC_1,
        }
    }
    #[doc = "Checks if the value of the field is `NUM_SEC_0`"]
    #[inline(always)]
    pub fn is_num_sec_0(&self) -> bool {
        *self == NUM_SEC_A::NUM_SEC_0
    }
    #[doc = "Checks if the value of the field is `NUM_SEC_1`"]
    #[inline(always)]
    pub fn is_num_sec_1(&self) -> bool {
        *self == NUM_SEC_A::NUM_SEC_1
    }
}
#[doc = "Number of FIFOs\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUM_FIFO_A {
    #[doc = "0: N/A"]
    NUM_FIFO_0 = 0,
    #[doc = "1: This design supports one result FIFO."]
    NUM_FIFO_1 = 1,
    #[doc = "2: This design supports two result FIFOs."]
    NUM_FIFO_2 = 2,
    #[doc = "3: This design supports three result FIFOs."]
    NUM_FIFO_3 = 3,
    #[doc = "4: This design supports four result FIFOs."]
    NUM_FIFO_4 = 4,
}
impl From<NUM_FIFO_A> for u8 {
    #[inline(always)]
    fn from(variant: NUM_FIFO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NUM_FIFO`"]
pub type NUM_FIFO_R = crate::R<u8, NUM_FIFO_A>;
impl NUM_FIFO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NUM_FIFO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NUM_FIFO_A::NUM_FIFO_0),
            1 => Val(NUM_FIFO_A::NUM_FIFO_1),
            2 => Val(NUM_FIFO_A::NUM_FIFO_2),
            3 => Val(NUM_FIFO_A::NUM_FIFO_3),
            4 => Val(NUM_FIFO_A::NUM_FIFO_4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_0`"]
    #[inline(always)]
    pub fn is_num_fifo_0(&self) -> bool {
        *self == NUM_FIFO_A::NUM_FIFO_0
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_1`"]
    #[inline(always)]
    pub fn is_num_fifo_1(&self) -> bool {
        *self == NUM_FIFO_A::NUM_FIFO_1
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_2`"]
    #[inline(always)]
    pub fn is_num_fifo_2(&self) -> bool {
        *self == NUM_FIFO_A::NUM_FIFO_2
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_3`"]
    #[inline(always)]
    pub fn is_num_fifo_3(&self) -> bool {
        *self == NUM_FIFO_A::NUM_FIFO_3
    }
    #[doc = "Checks if the value of the field is `NUM_FIFO_4`"]
    #[inline(always)]
    pub fn is_num_fifo_4(&self) -> bool {
        *self == NUM_FIFO_A::NUM_FIFO_4
    }
}
#[doc = "Reader of field `MINOR`"]
pub type MINOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Differential Supported"]
    #[inline(always)]
    pub fn diffen(&self) -> DIFFEN_R {
        DIFFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Multi Vref Implemented"]
    #[inline(always)]
    pub fn mvi(&self) -> MVI_R {
        MVI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Channel Scale Width"]
    #[inline(always)]
    pub fn csw(&self) -> CSW_R {
        CSW_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub fn vr1rngi(&self) -> VR1RNGI_R {
        VR1RNGI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Internal ADC Clock implemented"]
    #[inline(always)]
    pub fn iadcki(&self) -> IADCKI_R {
        IADCKI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Calibration Function Implemented"]
    #[inline(always)]
    pub fn calofsi(&self) -> CALOFSI_R {
        CALOFSI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Number of Single Ended Outputs Supported"]
    #[inline(always)]
    pub fn num_sec(&self) -> NUM_SEC_R {
        NUM_SEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Number of FIFOs"]
    #[inline(always)]
    pub fn num_fifo(&self) -> NUM_FIFO_R {
        NUM_FIFO_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Minor Version Number"]
    #[inline(always)]
    pub fn minor(&self) -> MINOR_R {
        MINOR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
