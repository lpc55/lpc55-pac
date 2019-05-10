#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub sec_ctrl_flash_rom_slave_rule: SEC_CTRL_FLASH_ROM_SLAVE_RULE,
    _reserved0: [u8; 12usize],
    #[doc = "0x10 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule0: SEC_CTRL_FLASH_MEM_RULE0,
    #[doc = "0x14 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule1: SEC_CTRL_FLASH_MEM_RULE1,
    #[doc = "0x18 - Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
    pub sec_ctrl_flash_mem_rule2: SEC_CTRL_FLASH_MEM_RULE2,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule0: SEC_CTRL_ROM_MEM_RULE0,
    #[doc = "0x24 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule1: SEC_CTRL_ROM_MEM_RULE1,
    #[doc = "0x28 - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule2: SEC_CTRL_ROM_MEM_RULE2,
    #[doc = "0x2c - Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
    pub sec_ctrl_rom_mem_rule3: SEC_CTRL_ROM_MEM_RULE3,
    #[doc = "0x30 - Security access rules for RAMX slaves."]
    pub sec_ctrl_ramx_slave_rule: SEC_CTRL_RAMX_SLAVE_RULE,
    _reserved2: [u8; 12usize],
    #[doc = "0x40 - no description available"]
    pub sec_ctrl_ramx_mem_rule0: SEC_CTRL_RAMX_MEM_RULE0,
    _reserved3: [u8; 12usize],
    #[doc = "0x50 - Security access rules for RAM0 slaves."]
    pub sec_ctrl_ram0_slave_rule: SEC_CTRL_RAM0_SLAVE_RULE,
    _reserved4: [u8; 12usize],
    #[doc = "0x60 - no description available"]
    pub sec_ctrl_ram0_mem_rule0: SEC_CTRL_RAM0_MEM_RULE0,
    #[doc = "0x64 - no description available"]
    pub sec_ctrl_ram0_mem_rule1: SEC_CTRL_RAM0_MEM_RULE1,
    _reserved5: [u8; 8usize],
    #[doc = "0x70 - Security access rules for RAM1 slaves."]
    pub sec_ctrl_ram1_slave_rule: SEC_CTRL_RAM1_SLAVE_RULE,
    _reserved6: [u8; 12usize],
    #[doc = "0x80 - no description available"]
    pub sec_ctrl_ram1_mem_rule0: SEC_CTRL_RAM1_MEM_RULE0,
    #[doc = "0x84 - no description available"]
    pub sec_ctrl_ram1_mem_rule1: SEC_CTRL_RAM1_MEM_RULE1,
    _reserved7: [u8; 8usize],
    #[doc = "0x90 - Security access rules for RAM2 slaves."]
    pub sec_ctrl_ram2_slave_rule: SEC_CTRL_RAM2_SLAVE_RULE,
    _reserved8: [u8; 12usize],
    #[doc = "0xa0 - no description available"]
    pub sec_ctrl_ram2_mem_rule0: SEC_CTRL_RAM2_MEM_RULE0,
    #[doc = "0xa4 - no description available"]
    pub sec_ctrl_ram2_mem_rule1: SEC_CTRL_RAM2_MEM_RULE1,
    _reserved9: [u8; 8usize],
    #[doc = "0xb0 - Security access rules for RAM3 slaves."]
    pub sec_ctrl_ram3_slave_rule: SEC_CTRL_RAM3_SLAVE_RULE,
    _reserved10: [u8; 12usize],
    #[doc = "0xc0 - no description available"]
    pub sec_ctrl_ram3_mem_rule0: SEC_CTRL_RAM3_MEM_RULE0,
    #[doc = "0xc4 - no description available"]
    pub sec_ctrl_ram3_mem_rule1: SEC_CTRL_RAM3_MEM_RULE1,
    _reserved11: [u8; 8usize],
    #[doc = "0xd0 - Security access rules for RAM4 slaves."]
    pub sec_ctrl_ram4_slave_rule: SEC_CTRL_RAM4_SLAVE_RULE,
    _reserved12: [u8; 12usize],
    #[doc = "0xe0 - no description available"]
    pub sec_ctrl_ram4_mem_rule0: SEC_CTRL_RAM4_MEM_RULE0,
    _reserved13: [u8; 12usize],
    #[doc = "0xf0 - no description available"]
    pub sec_ctrl_apb_bridge_slave_rule: SEC_CTRL_APB_BRIDGE_SLAVE_RULE,
    _reserved14: [u8; 12usize],
    #[doc = "0x100 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl0: SEC_CTRL_APB_BRIDGE0_MEM_CTRL0,
    #[doc = "0x104 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl1: SEC_CTRL_APB_BRIDGE0_MEM_CTRL1,
    #[doc = "0x108 - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl2: SEC_CTRL_APB_BRIDGE0_MEM_CTRL2,
    #[doc = "0x10c - Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
    pub sec_ctrl_apb_bridge0_mem_ctrl3: SEC_CTRL_APB_BRIDGE0_MEM_CTRL3,
    #[doc = "0x110 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl0: SEC_CTRL_APB_BRIDGE1_MEM_CTRL0,
    #[doc = "0x114 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl1: SEC_CTRL_APB_BRIDGE1_MEM_CTRL1,
    #[doc = "0x118 - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl2: SEC_CTRL_APB_BRIDGE1_MEM_CTRL2,
    #[doc = "0x11c - Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
    pub sec_ctrl_apb_bridge1_mem_ctrl3: SEC_CTRL_APB_BRIDGE1_MEM_CTRL3,
    #[doc = "0x120 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb0_0_slave_rule: SEC_CTRL_AHB0_0_SLAVE_RULE,
    #[doc = "0x124 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb0_1_slave_rule: SEC_CTRL_AHB0_1_SLAVE_RULE,
    _reserved15: [u8; 8usize],
    #[doc = "0x130 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb1_0_slave_rule: SEC_CTRL_AHB1_0_SLAVE_RULE,
    #[doc = "0x134 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb1_1_slave_rule: SEC_CTRL_AHB1_1_SLAVE_RULE,
    _reserved16: [u8; 12usize],
    #[doc = "0x144 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb2_0_slave_rule: SEC_CTRL_AHB2_0_SLAVE_RULE,
    #[doc = "0x148 - Security access rules for AHB peripherals."]
    pub sec_ctrl_ahb2_1_slave_rule: SEC_CTRL_AHB2_1_SLAVE_RULE,
    _reserved17: [u8; 8usize],
    #[doc = "0x154 - no description available"]
    pub sec_ctrl_ahb2_0_mem_rule: SEC_CTRL_AHB2_0_MEM_RULE,
    _reserved18: [u8; 8usize],
    #[doc = "0x160 - no description available"]
    pub sec_ctrl_usb_hs_slave_rule: SEC_CTRL_USB_HS_SLAVE_RULE,
    _reserved19: [u8; 12usize],
    #[doc = "0x170 - no description available"]
    pub sec_ctrl_usb_hs_mem_rule: SEC_CTRL_USB_HS_MEM_RULE,
    _reserved20: [u8; 3212usize],
    #[doc = "0xe00 - most recent security violation address for AHB layer n"]
    pub sec_vio_addr: [SEC_VIO_ADDR; 18],
    _reserved21: [u8; 56usize],
    #[doc = "0xe80 - most recent security violation miscellaneous information for AHB layer n"]
    pub sec_vio_misc_info: [SEC_VIO_MISC_INFO; 18],
    _reserved22: [u8; 56usize],
    #[doc = "0xf00 - security violation address/information registers valid flags"]
    pub sec_vio_info_valid: SEC_VIO_INFO_VALID,
    _reserved23: [u8; 124usize],
    #[doc = "0xf80 - Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
    pub sec_gpio_mask0: SEC_GPIO_MASK0,
    #[doc = "0xf84 - Secure GPIO mask for port 1 pins."]
    pub sec_gpio_mask1: SEC_GPIO_MASK1,
    #[doc = "0xf88 - Secure GPIO mask for port 2 pins."]
    pub sec_gpio_mask2: SEC_GPIO_MASK2,
    #[doc = "0xf8c - Secure GPIO mask for port 3 pins."]
    pub sec_gpio_mask3: SEC_GPIO_MASK3,
    #[doc = "0xf90 - Secure Interrupt mask for CPU1"]
    pub sec_cpu_int_mask0: SEC_CPU_INT_MASK0,
    #[doc = "0xf94 - Secure Interrupt mask for CPU1"]
    pub sec_cpu_int_mask1: SEC_CPU_INT_MASK1,
    _reserved24: [u8; 36usize],
    #[doc = "0xfbc - Security General Purpose register access control."]
    pub sec_mask_lock: SEC_MASK_LOCK,
    _reserved25: [u8; 16usize],
    #[doc = "0xfd0 - master secure level register"]
    pub master_sec_level: MASTER_SEC_LEVEL,
    #[doc = "0xfd4 - master secure level anti-pole register"]
    pub master_sec_anti_pol_reg: MASTER_SEC_ANTI_POL_REG,
    _reserved26: [u8; 20usize],
    #[doc = "0xfec - Miscalleneous control signals for in CM33 (CPU0)"]
    pub cm33_lock_reg: CM33_LOCK_REG,
    #[doc = "0xff0 - Miscalleneous control signals for in micro-CM33 (CPU1)"]
    pub mcm33_lock_reg: MCM33_LOCK_REG,
    _reserved27: [u8; 4usize],
    #[doc = "0xff8 - secure control duplicate register"]
    pub misc_ctrl_dp_reg: MISC_CTRL_DP_REG,
    #[doc = "0xffc - secure control register"]
    pub misc_ctrl_reg: MISC_CTRL_REG,
}
#[doc = "no description available"]
pub struct SEC_CTRL_FLASH_ROM_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_flash_rom_slave_rule;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub struct SEC_CTRL_FLASH_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule0;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub struct SEC_CTRL_FLASH_MEM_RULE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule1;
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub struct SEC_CTRL_FLASH_MEM_RULE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for FLASH sector 0 to sector 20. Each Flash sector is 32 Kbytes. There are 20 FLASH sectors in total."]
pub mod sec_ctrl_flash_mem_rule2;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub struct SEC_CTRL_ROM_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule0;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub struct SEC_CTRL_ROM_MEM_RULE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule1;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub struct SEC_CTRL_ROM_MEM_RULE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule2;
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub struct SEC_CTRL_ROM_MEM_RULE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for ROM sector 0 to sector 31. Each ROM sector is 4 Kbytes. There are 32 ROM sectors in total."]
pub mod sec_ctrl_rom_mem_rule3;
#[doc = "Security access rules for RAMX slaves."]
pub struct SEC_CTRL_RAMX_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for RAMX slaves."]
pub mod sec_ctrl_ramx_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_RAMX_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ramx_mem_rule0;
#[doc = "Security access rules for RAM0 slaves."]
pub struct SEC_CTRL_RAM0_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for RAM0 slaves."]
pub mod sec_ctrl_ram0_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM0_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram0_mem_rule0;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM0_MEM_RULE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram0_mem_rule1;
#[doc = "Security access rules for RAM1 slaves."]
pub struct SEC_CTRL_RAM1_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for RAM1 slaves."]
pub mod sec_ctrl_ram1_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM1_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram1_mem_rule0;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM1_MEM_RULE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram1_mem_rule1;
#[doc = "Security access rules for RAM2 slaves."]
pub struct SEC_CTRL_RAM2_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for RAM2 slaves."]
pub mod sec_ctrl_ram2_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM2_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram2_mem_rule0;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM2_MEM_RULE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram2_mem_rule1;
#[doc = "Security access rules for RAM3 slaves."]
pub struct SEC_CTRL_RAM3_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for RAM3 slaves."]
pub mod sec_ctrl_ram3_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM3_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram3_mem_rule0;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM3_MEM_RULE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram3_mem_rule1;
#[doc = "Security access rules for RAM4 slaves."]
pub struct SEC_CTRL_RAM4_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for RAM4 slaves."]
pub mod sec_ctrl_ram4_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_RAM4_MEM_RULE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ram4_mem_rule0;
#[doc = "no description available"]
pub struct SEC_CTRL_APB_BRIDGE_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_apb_bridge_slave_rule;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl0;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl1;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl2;
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE0_MEM_CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 0 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 0 sectors in total."]
pub mod sec_ctrl_apb_bridge0_mem_ctrl3;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl0;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl1;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl2;
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub struct SEC_CTRL_APB_BRIDGE1_MEM_CTRL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for APB Bridge 1 peripherals. Each APB bridge sector is 4 Kbytes. There are 32 APB Bridge 1 sectors in total."]
pub mod sec_ctrl_apb_bridge1_mem_ctrl3;
#[doc = "Security access rules for AHB peripherals."]
pub struct SEC_CTRL_AHB0_0_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb0_0_slave_rule;
#[doc = "Security access rules for AHB peripherals."]
pub struct SEC_CTRL_AHB0_1_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb0_1_slave_rule;
#[doc = "Security access rules for AHB peripherals."]
pub struct SEC_CTRL_AHB1_0_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb1_0_slave_rule;
#[doc = "Security access rules for AHB peripherals."]
pub struct SEC_CTRL_AHB1_1_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb1_1_slave_rule;
#[doc = "Security access rules for AHB peripherals."]
pub struct SEC_CTRL_AHB2_0_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb2_0_slave_rule;
#[doc = "Security access rules for AHB peripherals."]
pub struct SEC_CTRL_AHB2_1_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security access rules for AHB peripherals."]
pub mod sec_ctrl_ahb2_1_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_AHB2_0_MEM_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_ahb2_0_mem_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_USB_HS_SLAVE_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_usb_hs_slave_rule;
#[doc = "no description available"]
pub struct SEC_CTRL_USB_HS_MEM_RULE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "no description available"]
pub mod sec_ctrl_usb_hs_mem_rule;
#[doc = "most recent security violation address for AHB layer n"]
pub struct SEC_VIO_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "most recent security violation address for AHB layer n"]
pub mod sec_vio_addr;
#[doc = "most recent security violation miscellaneous information for AHB layer n"]
pub struct SEC_VIO_MISC_INFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "most recent security violation miscellaneous information for AHB layer n"]
pub mod sec_vio_misc_info;
#[doc = "security violation address/information registers valid flags"]
pub struct SEC_VIO_INFO_VALID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "security violation address/information registers valid flags"]
pub mod sec_vio_info_valid;
#[doc = "Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
pub struct SEC_GPIO_MASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure GPIO mask for port 0 pins. This register is used to block leakage of Secure interface (GPIOs, I2C, UART configured as secure peripherals) pin states to non-secure world."]
pub mod sec_gpio_mask0;
#[doc = "Secure GPIO mask for port 1 pins."]
pub struct SEC_GPIO_MASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure GPIO mask for port 1 pins."]
pub mod sec_gpio_mask1;
#[doc = "Secure GPIO mask for port 2 pins."]
pub struct SEC_GPIO_MASK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure GPIO mask for port 2 pins."]
pub mod sec_gpio_mask2;
#[doc = "Secure GPIO mask for port 3 pins."]
pub struct SEC_GPIO_MASK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure GPIO mask for port 3 pins."]
pub mod sec_gpio_mask3;
#[doc = "Secure Interrupt mask for CPU1"]
pub struct SEC_CPU_INT_MASK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Interrupt mask for CPU1"]
pub mod sec_cpu_int_mask0;
#[doc = "Secure Interrupt mask for CPU1"]
pub struct SEC_CPU_INT_MASK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Secure Interrupt mask for CPU1"]
pub mod sec_cpu_int_mask1;
#[doc = "Security General Purpose register access control."]
pub struct SEC_MASK_LOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security General Purpose register access control."]
pub mod sec_mask_lock;
#[doc = "master secure level register"]
pub struct MASTER_SEC_LEVEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master secure level register"]
pub mod master_sec_level;
#[doc = "master secure level anti-pole register"]
pub struct MASTER_SEC_ANTI_POL_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "master secure level anti-pole register"]
pub mod master_sec_anti_pol_reg;
#[doc = "Miscalleneous control signals for in CM33 (CPU0)"]
pub struct CM33_LOCK_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscalleneous control signals for in CM33 (CPU0)"]
pub mod cm33_lock_reg;
#[doc = "Miscalleneous control signals for in micro-CM33 (CPU1)"]
pub struct MCM33_LOCK_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Miscalleneous control signals for in micro-CM33 (CPU1)"]
pub mod mcm33_lock_reg;
#[doc = "secure control duplicate register"]
pub struct MISC_CTRL_DP_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "secure control duplicate register"]
pub mod misc_ctrl_dp_reg;
#[doc = "secure control register"]
pub struct MISC_CTRL_REG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "secure control register"]
pub mod misc_ctrl_reg;
