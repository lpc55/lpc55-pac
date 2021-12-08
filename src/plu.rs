#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x14 - no description available"]
    pub lut0: LUT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x20..0x34 - no description available"]
    pub lut1: LUT,
    _reserved2: [u8; 0x0c],
    #[doc = "0x40..0x54 - no description available"]
    pub lut2: LUT,
    _reserved3: [u8; 0x0c],
    #[doc = "0x60..0x74 - no description available"]
    pub lut3: LUT,
    _reserved4: [u8; 0x0c],
    #[doc = "0x80..0x94 - no description available"]
    pub lut4: LUT,
    _reserved5: [u8; 0x0c],
    #[doc = "0xa0..0xb4 - no description available"]
    pub lut5: LUT,
    _reserved6: [u8; 0x0c],
    #[doc = "0xc0..0xd4 - no description available"]
    pub lut6: LUT,
    _reserved7: [u8; 0x0c],
    #[doc = "0xe0..0xf4 - no description available"]
    pub lut7: LUT,
    _reserved8: [u8; 0x0c],
    #[doc = "0x100..0x114 - no description available"]
    pub lut8: LUT,
    _reserved9: [u8; 0x0c],
    #[doc = "0x120..0x134 - no description available"]
    pub lut9: LUT,
    _reserved10: [u8; 0x0c],
    #[doc = "0x140..0x154 - no description available"]
    pub lut10: LUT,
    _reserved11: [u8; 0x0c],
    #[doc = "0x160..0x174 - no description available"]
    pub lut11: LUT,
    _reserved12: [u8; 0x0c],
    #[doc = "0x180..0x194 - no description available"]
    pub lut12: LUT,
    _reserved13: [u8; 0x0c],
    #[doc = "0x1a0..0x1b4 - no description available"]
    pub lut13: LUT,
    _reserved14: [u8; 0x0c],
    #[doc = "0x1c0..0x1d4 - no description available"]
    pub lut14: LUT,
    _reserved15: [u8; 0x0c],
    #[doc = "0x1e0..0x1f4 - no description available"]
    pub lut15: LUT,
    _reserved16: [u8; 0x0c],
    #[doc = "0x200..0x214 - no description available"]
    pub lut16: LUT,
    _reserved17: [u8; 0x0c],
    #[doc = "0x220..0x234 - no description available"]
    pub lut17: LUT,
    _reserved18: [u8; 0x0c],
    #[doc = "0x240..0x254 - no description available"]
    pub lut18: LUT,
    _reserved19: [u8; 0x0c],
    #[doc = "0x260..0x274 - no description available"]
    pub lut19: LUT,
    _reserved20: [u8; 0x0c],
    #[doc = "0x280..0x294 - no description available"]
    pub lut20: LUT,
    _reserved21: [u8; 0x0c],
    #[doc = "0x2a0..0x2b4 - no description available"]
    pub lut21: LUT,
    _reserved22: [u8; 0x0c],
    #[doc = "0x2c0..0x2d4 - no description available"]
    pub lut22: LUT,
    _reserved23: [u8; 0x0c],
    #[doc = "0x2e0..0x2f4 - no description available"]
    pub lut23: LUT,
    _reserved24: [u8; 0x0c],
    #[doc = "0x300..0x314 - no description available"]
    pub lut24: LUT,
    _reserved25: [u8; 0x0c],
    #[doc = "0x320..0x334 - no description available"]
    pub lut25: LUT,
    _reserved26: [u8; 0x04cc],
    #[doc = "0x800..0x868 - Specifies the Truth Table contents for LUTLUTn"]
    pub lut_truth: [crate::Reg<lut_truth::LUT_TRUTH_SPEC>; 26],
    _reserved27: [u8; 0x98],
    #[doc = "0x900 - Provides the current state of the 8 designated PLU Outputs."]
    pub outputs: crate::Reg<outputs::OUTPUTS_SPEC>,
    #[doc = "0x904 - Wakeup interrupt control for PLU"]
    pub wakeint_ctrl: crate::Reg<wakeint_ctrl::WAKEINT_CTRL_SPEC>,
    _reserved29: [u8; 0x02f8],
    #[doc = "0xc00..0xc20 - Selects the source to be connected to PLU Output OUTPUT_n"]
    pub output_mux: [crate::Reg<output_mux::OUTPUT_MUX_SPEC>; 8],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct LUT {
    #[doc = "0x00..0x14 - LUTn input x MUX"]
    pub lut_inp_mux: [crate::Reg<self::lut::lut_inp_mux::LUT_INP_MUX_SPEC>; 5],
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod lut;
#[doc = "LUT_TRUTH register accessor: an alias for `Reg<LUT_TRUTH_SPEC>`"]
pub type LUT_TRUTH = crate::Reg<lut_truth::LUT_TRUTH_SPEC>;
#[doc = "Specifies the Truth Table contents for LUTLUTn"]
pub mod lut_truth;
#[doc = "OUTPUTS register accessor: an alias for `Reg<OUTPUTS_SPEC>`"]
pub type OUTPUTS = crate::Reg<outputs::OUTPUTS_SPEC>;
#[doc = "Provides the current state of the 8 designated PLU Outputs."]
pub mod outputs;
#[doc = "WAKEINT_CTRL register accessor: an alias for `Reg<WAKEINT_CTRL_SPEC>`"]
pub type WAKEINT_CTRL = crate::Reg<wakeint_ctrl::WAKEINT_CTRL_SPEC>;
#[doc = "Wakeup interrupt control for PLU"]
pub mod wakeint_ctrl;
#[doc = "OUTPUT_MUX register accessor: an alias for `Reg<OUTPUT_MUX_SPEC>`"]
pub type OUTPUT_MUX = crate::Reg<output_mux::OUTPUT_MUX_SPEC>;
#[doc = "Selects the source to be connected to PLU Output OUTPUT_n"]
pub mod output_mux;
