#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains a random 32 bit number which is computed on demand, at each time it is read"]
    pub random_number: RANDOM_NUMBER,
    #[doc = "0x04 - This register contains a random 32 bit number which is pre-computed"]
    pub encrypted_number: ENCRYPTED_NUMBER,
    #[doc = "0x08 - no description available"]
    pub counter_val: COUNTER_VAL,
    #[doc = "0x0c - no description available"]
    pub counter_cfg: COUNTER_CFG,
    #[doc = "0x10 - no description available"]
    pub online_test_cfg: ONLINE_TEST_CFG,
    #[doc = "0x14 - no description available"]
    pub online_test_val: ONLINE_TEST_VAL,
    #[doc = "0x18 - no description available"]
    pub misc_cfg: MISC_CFG,
    _reserved0: [u8; 4056usize],
    #[doc = "0xff4 - Powerdown mode (standard but certainly useless here)"]
    pub powerdown: POWERDOWN,
    _reserved1: [u8; 4usize],
    #[doc = "0xffc - IP identifier"]
    pub moduleid: MODULEID,
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
pub struct RANDOM_NUMBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
pub mod random_number;
#[doc = "This register contains a random 32 bit number which is pre-computed"]
pub struct ENCRYPTED_NUMBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This register contains a random 32 bit number which is pre-computed"]
pub mod encrypted_number;
#[doc = "no description available"]
pub struct COUNTER_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod counter_val;
#[doc = "no description available"]
pub struct COUNTER_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod counter_cfg;
#[doc = "no description available"]
pub struct ONLINE_TEST_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod online_test_cfg;
#[doc = "no description available"]
pub struct ONLINE_TEST_VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod online_test_val;
#[doc = "no description available"]
pub struct MISC_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod misc_cfg;
#[doc = "Powerdown mode (standard but certainly useless here)"]
pub struct POWERDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Powerdown mode (standard but certainly useless here)"]
pub mod powerdown;
#[doc = "IP identifier"]
pub struct MODULEID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP identifier"]
pub mod moduleid;
