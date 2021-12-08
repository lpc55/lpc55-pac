#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd4],
    #[doc = "0xd4 - Value replacement 7"]
    pub value7: crate::Reg<value7::VALUE7_SPEC>,
    #[doc = "0xd8 - Value replacement 6"]
    pub value6: crate::Reg<value6::VALUE6_SPEC>,
    #[doc = "0xdc - Value replacement 5"]
    pub value5: crate::Reg<value5::VALUE5_SPEC>,
    #[doc = "0xe0 - Value replacement 4"]
    pub value4: crate::Reg<value4::VALUE4_SPEC>,
    #[doc = "0xe4 - Value replacement 3"]
    pub value3: crate::Reg<value3::VALUE3_SPEC>,
    #[doc = "0xe8 - Value replacement 2"]
    pub value2: crate::Reg<value2::VALUE2_SPEC>,
    #[doc = "0xec - Value replacement 1"]
    pub value1: crate::Reg<value1::VALUE1_SPEC>,
    #[doc = "0xf0 - Value replacement 0"]
    pub value0: crate::Reg<value0::VALUE0_SPEC>,
    #[doc = "0xf4 - Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0xfc - Enable register"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x100 - Replacement address 0"]
    pub address0: crate::Reg<address0::ADDRESS0_SPEC>,
    #[doc = "0x104 - Replacement address 2"]
    pub address1: crate::Reg<address1::ADDRESS1_SPEC>,
    #[doc = "0x108 - Replacement address 2"]
    pub address2: crate::Reg<address2::ADDRESS2_SPEC>,
    #[doc = "0x10c - Replacement address 3"]
    pub address3: crate::Reg<address3::ADDRESS3_SPEC>,
    #[doc = "0x110 - Replacement address 4"]
    pub address4: crate::Reg<address4::ADDRESS4_SPEC>,
    #[doc = "0x114 - Replacement address 5"]
    pub address5: crate::Reg<address5::ADDRESS5_SPEC>,
    #[doc = "0x118 - Replacement address 6"]
    pub address6: crate::Reg<address6::ADDRESS6_SPEC>,
    #[doc = "0x11c - Replacement address 7"]
    pub address7: crate::Reg<address7::ADDRESS7_SPEC>,
    #[doc = "0x120 - Replacement address 8"]
    pub address8: crate::Reg<address8::ADDRESS8_SPEC>,
    #[doc = "0x124 - Replacement address 9"]
    pub address9: crate::Reg<address9::ADDRESS9_SPEC>,
    #[doc = "0x128 - Replacement address 10"]
    pub address10: crate::Reg<address10::ADDRESS10_SPEC>,
    #[doc = "0x12c - Replacement address 11"]
    pub address11: crate::Reg<address11::ADDRESS11_SPEC>,
    #[doc = "0x130 - Replacement address 12"]
    pub address12: crate::Reg<address12::ADDRESS12_SPEC>,
    #[doc = "0x134 - Replacement address 13"]
    pub address13: crate::Reg<address13::ADDRESS13_SPEC>,
    #[doc = "0x138 - Replacement address 14"]
    pub address14: crate::Reg<address14::ADDRESS14_SPEC>,
    #[doc = "0x13c - Replacement address 15"]
    pub address15: crate::Reg<address15::ADDRESS15_SPEC>,
}
#[doc = "VALUE7 register accessor: an alias for `Reg<VALUE7_SPEC>`"]
pub type VALUE7 = crate::Reg<value7::VALUE7_SPEC>;
#[doc = "Value replacement 7"]
pub mod value7;
#[doc = "VALUE6 register accessor: an alias for `Reg<VALUE6_SPEC>`"]
pub type VALUE6 = crate::Reg<value6::VALUE6_SPEC>;
#[doc = "Value replacement 6"]
pub mod value6;
#[doc = "VALUE5 register accessor: an alias for `Reg<VALUE5_SPEC>`"]
pub type VALUE5 = crate::Reg<value5::VALUE5_SPEC>;
#[doc = "Value replacement 5"]
pub mod value5;
#[doc = "VALUE4 register accessor: an alias for `Reg<VALUE4_SPEC>`"]
pub type VALUE4 = crate::Reg<value4::VALUE4_SPEC>;
#[doc = "Value replacement 4"]
pub mod value4;
#[doc = "VALUE3 register accessor: an alias for `Reg<VALUE3_SPEC>`"]
pub type VALUE3 = crate::Reg<value3::VALUE3_SPEC>;
#[doc = "Value replacement 3"]
pub mod value3;
#[doc = "VALUE2 register accessor: an alias for `Reg<VALUE2_SPEC>`"]
pub type VALUE2 = crate::Reg<value2::VALUE2_SPEC>;
#[doc = "Value replacement 2"]
pub mod value2;
#[doc = "VALUE1 register accessor: an alias for `Reg<VALUE1_SPEC>`"]
pub type VALUE1 = crate::Reg<value1::VALUE1_SPEC>;
#[doc = "Value replacement 1"]
pub mod value1;
#[doc = "VALUE0 register accessor: an alias for `Reg<VALUE0_SPEC>`"]
pub type VALUE0 = crate::Reg<value0::VALUE0_SPEC>;
#[doc = "Value replacement 0"]
pub mod value0;
#[doc = "CONTROL register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register"]
pub mod control;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable register"]
pub mod enable;
#[doc = "ADDRESS0 register accessor: an alias for `Reg<ADDRESS0_SPEC>`"]
pub type ADDRESS0 = crate::Reg<address0::ADDRESS0_SPEC>;
#[doc = "Replacement address 0"]
pub mod address0;
#[doc = "ADDRESS1 register accessor: an alias for `Reg<ADDRESS1_SPEC>`"]
pub type ADDRESS1 = crate::Reg<address1::ADDRESS1_SPEC>;
#[doc = "Replacement address 2"]
pub mod address1;
#[doc = "ADDRESS2 register accessor: an alias for `Reg<ADDRESS2_SPEC>`"]
pub type ADDRESS2 = crate::Reg<address2::ADDRESS2_SPEC>;
#[doc = "Replacement address 2"]
pub mod address2;
#[doc = "ADDRESS3 register accessor: an alias for `Reg<ADDRESS3_SPEC>`"]
pub type ADDRESS3 = crate::Reg<address3::ADDRESS3_SPEC>;
#[doc = "Replacement address 3"]
pub mod address3;
#[doc = "ADDRESS4 register accessor: an alias for `Reg<ADDRESS4_SPEC>`"]
pub type ADDRESS4 = crate::Reg<address4::ADDRESS4_SPEC>;
#[doc = "Replacement address 4"]
pub mod address4;
#[doc = "ADDRESS5 register accessor: an alias for `Reg<ADDRESS5_SPEC>`"]
pub type ADDRESS5 = crate::Reg<address5::ADDRESS5_SPEC>;
#[doc = "Replacement address 5"]
pub mod address5;
#[doc = "ADDRESS6 register accessor: an alias for `Reg<ADDRESS6_SPEC>`"]
pub type ADDRESS6 = crate::Reg<address6::ADDRESS6_SPEC>;
#[doc = "Replacement address 6"]
pub mod address6;
#[doc = "ADDRESS7 register accessor: an alias for `Reg<ADDRESS7_SPEC>`"]
pub type ADDRESS7 = crate::Reg<address7::ADDRESS7_SPEC>;
#[doc = "Replacement address 7"]
pub mod address7;
#[doc = "ADDRESS8 register accessor: an alias for `Reg<ADDRESS8_SPEC>`"]
pub type ADDRESS8 = crate::Reg<address8::ADDRESS8_SPEC>;
#[doc = "Replacement address 8"]
pub mod address8;
#[doc = "ADDRESS9 register accessor: an alias for `Reg<ADDRESS9_SPEC>`"]
pub type ADDRESS9 = crate::Reg<address9::ADDRESS9_SPEC>;
#[doc = "Replacement address 9"]
pub mod address9;
#[doc = "ADDRESS10 register accessor: an alias for `Reg<ADDRESS10_SPEC>`"]
pub type ADDRESS10 = crate::Reg<address10::ADDRESS10_SPEC>;
#[doc = "Replacement address 10"]
pub mod address10;
#[doc = "ADDRESS11 register accessor: an alias for `Reg<ADDRESS11_SPEC>`"]
pub type ADDRESS11 = crate::Reg<address11::ADDRESS11_SPEC>;
#[doc = "Replacement address 11"]
pub mod address11;
#[doc = "ADDRESS12 register accessor: an alias for `Reg<ADDRESS12_SPEC>`"]
pub type ADDRESS12 = crate::Reg<address12::ADDRESS12_SPEC>;
#[doc = "Replacement address 12"]
pub mod address12;
#[doc = "ADDRESS13 register accessor: an alias for `Reg<ADDRESS13_SPEC>`"]
pub type ADDRESS13 = crate::Reg<address13::ADDRESS13_SPEC>;
#[doc = "Replacement address 13"]
pub mod address13;
#[doc = "ADDRESS14 register accessor: an alias for `Reg<ADDRESS14_SPEC>`"]
pub type ADDRESS14 = crate::Reg<address14::ADDRESS14_SPEC>;
#[doc = "Replacement address 14"]
pub mod address14;
#[doc = "ADDRESS15 register accessor: an alias for `Reg<ADDRESS15_SPEC>`"]
pub type ADDRESS15 = crate::Reg<address15::ADDRESS15_SPEC>;
#[doc = "Replacement address 15"]
pub mod address15;
