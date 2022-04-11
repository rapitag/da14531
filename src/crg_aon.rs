#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hardware Reset control register"]
    pub hwr_ctrl_reg: crate::Reg<hwr_ctrl_reg::HWR_CTRL_REG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Reset status register"]
    pub reset_stat_reg: crate::Reg<reset_stat_reg::RESET_STAT_REG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - "]
    pub ram_lpmx_reg: crate::Reg<ram_lpmx_reg::RAM_LPMX_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Control the state retention of the GPIO ports"]
    pub pad_latch_reg: crate::Reg<pad_latch_reg::PAD_LATCH_REG_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - Hibernation control register"]
    pub hibern_ctrl_reg: crate::Reg<hibern_ctrl_reg::HIBERN_CTRL_REG_SPEC>,
    _reserved5: [u8; 0x0e],
    #[doc = "0x20 - "]
    pub power_aon_ctrl_reg: crate::Reg<power_aon_ctrl_reg::POWER_AON_CTRL_REG_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x24 - "]
    pub gp_data_reg: crate::Reg<gp_data_reg::GP_DATA_REG_SPEC>,
    _reserved7: [u8; 0xca],
    #[doc = "0xf0 - "]
    pub test_vdd_reg: crate::Reg<test_vdd_reg::TEST_VDD_REG_SPEC>,
}
#[doc = "GP_DATA_REG register accessor: an alias for `Reg<GP_DATA_REG_SPEC>`"]
pub type GP_DATA_REG = crate::Reg<gp_data_reg::GP_DATA_REG_SPEC>;
#[doc = ""]
pub mod gp_data_reg;
#[doc = "HIBERN_CTRL_REG register accessor: an alias for `Reg<HIBERN_CTRL_REG_SPEC>`"]
pub type HIBERN_CTRL_REG = crate::Reg<hibern_ctrl_reg::HIBERN_CTRL_REG_SPEC>;
#[doc = "Hibernation control register"]
pub mod hibern_ctrl_reg;
#[doc = "HWR_CTRL_REG register accessor: an alias for `Reg<HWR_CTRL_REG_SPEC>`"]
pub type HWR_CTRL_REG = crate::Reg<hwr_ctrl_reg::HWR_CTRL_REG_SPEC>;
#[doc = "Hardware Reset control register"]
pub mod hwr_ctrl_reg;
#[doc = "PAD_LATCH_REG register accessor: an alias for `Reg<PAD_LATCH_REG_SPEC>`"]
pub type PAD_LATCH_REG = crate::Reg<pad_latch_reg::PAD_LATCH_REG_SPEC>;
#[doc = "Control the state retention of the GPIO ports"]
pub mod pad_latch_reg;
#[doc = "POWER_AON_CTRL_REG register accessor: an alias for `Reg<POWER_AON_CTRL_REG_SPEC>`"]
pub type POWER_AON_CTRL_REG = crate::Reg<power_aon_ctrl_reg::POWER_AON_CTRL_REG_SPEC>;
#[doc = ""]
pub mod power_aon_ctrl_reg;
#[doc = "RAM_LPMX_REG register accessor: an alias for `Reg<RAM_LPMX_REG_SPEC>`"]
pub type RAM_LPMX_REG = crate::Reg<ram_lpmx_reg::RAM_LPMX_REG_SPEC>;
#[doc = ""]
pub mod ram_lpmx_reg;
#[doc = "RESET_STAT_REG register accessor: an alias for `Reg<RESET_STAT_REG_SPEC>`"]
pub type RESET_STAT_REG = crate::Reg<reset_stat_reg::RESET_STAT_REG_SPEC>;
#[doc = "Reset status register"]
pub mod reset_stat_reg;
#[doc = "TEST_VDD_REG register accessor: an alias for `Reg<TEST_VDD_REG_SPEC>`"]
pub type TEST_VDD_REG = crate::Reg<test_vdd_reg::TEST_VDD_REG_SPEC>;
#[doc = ""]
pub mod test_vdd_reg;
