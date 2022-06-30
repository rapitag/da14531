#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - P0 Data input/output Register"]
    pub p0_data_reg: crate::Reg<p0_data_reg::P0_DATA_REG_SPEC>,
    #[doc = "0x02 - P0 Set port pins Register"]
    pub p0_set_data_reg: crate::Reg<p0_set_data_reg::P0_SET_DATA_REG_SPEC>,
    #[doc = "0x04 - P0 Reset port pins Register"]
    pub p0_reset_data_reg: crate::Reg<p0_reset_data_reg::P0_RESET_DATA_REG_SPEC>,
    #[doc = "0x06..0x1e - Configuration of GPIO pins"]
    pub p0_mode_reg: [crate::Reg<p0_mode_reg::P0_MODE_REG_SPEC>; 12],
    #[doc = "0x1e - Pad driving strength control Register"]
    pub pad_weak_ctrl_reg: crate::Reg<pad_weak_ctrl_reg::PAD_WEAK_CTRL_REG_SPEC>,
    #[doc = "0x20 - "]
    pub scan_observe_reg: crate::Reg<scan_observe_reg::SCAN_OBSERVE_REG_SPEC>,
    _reserved6: [u8; 0x0e],
    #[doc = "0x30 - "]
    pub test_ctrl_reg: crate::Reg<test_ctrl_reg::TEST_CTRL_REG_SPEC>,
    #[doc = "0x32 - "]
    pub test_ctrl2_reg: crate::Reg<test_ctrl2_reg::TEST_CTRL2_REG_SPEC>,
    #[doc = "0x34 - "]
    pub test_ctrl3_reg: crate::Reg<test_ctrl3_reg::TEST_CTRL3_REG_SPEC>,
    #[doc = "0x36 - "]
    pub test_ctrl4_reg: crate::Reg<test_ctrl4_reg::TEST_CTRL4_REG_SPEC>,
    #[doc = "0x38 - "]
    pub xtal32m_testctrl0_reg: crate::Reg<xtal32m_testctrl0_reg::XTAL32M_TESTCTRL0_REG_SPEC>,
    #[doc = "0x3a - "]
    pub xtal32m_testctrl1_reg: crate::Reg<xtal32m_testctrl1_reg::XTAL32M_TESTCTRL1_REG_SPEC>,
    #[doc = "0x3c - "]
    pub bist_ctrl_reg: crate::Reg<bist_ctrl_reg::BIST_CTRL_REG_SPEC>,
    #[doc = "0x3e - "]
    pub rombist_resultl_reg: crate::Reg<rombist_resultl_reg::ROMBIST_RESULTL_REG_SPEC>,
    #[doc = "0x40 - "]
    pub rombist_resulth_reg: crate::Reg<rombist_resulth_reg::ROMBIST_RESULTH_REG_SPEC>,
}
#[doc = "BIST_CTRL_REG register accessor: an alias for `Reg<BIST_CTRL_REG_SPEC>`"]
pub type BIST_CTRL_REG = crate::Reg<bist_ctrl_reg::BIST_CTRL_REG_SPEC>;
#[doc = ""]
pub mod bist_ctrl_reg;
#[doc = "P0_MODE_REG register accessor: an alias for `Reg<P0_MODE_REG_SPEC>`"]
pub type P0_MODE_REG = crate::Reg<p0_mode_reg::P0_MODE_REG_SPEC>;
#[doc = "Configuration of GPIO pins"]
pub mod p0_mode_reg;
#[doc = "P0_DATA_REG register accessor: an alias for `Reg<P0_DATA_REG_SPEC>`"]
pub type P0_DATA_REG = crate::Reg<p0_data_reg::P0_DATA_REG_SPEC>;
#[doc = "P0 Data input/output Register"]
pub mod p0_data_reg;
#[doc = "P0_RESET_DATA_REG register accessor: an alias for `Reg<P0_RESET_DATA_REG_SPEC>`"]
pub type P0_RESET_DATA_REG = crate::Reg<p0_reset_data_reg::P0_RESET_DATA_REG_SPEC>;
#[doc = "P0 Reset port pins Register"]
pub mod p0_reset_data_reg;
#[doc = "P0_SET_DATA_REG register accessor: an alias for `Reg<P0_SET_DATA_REG_SPEC>`"]
pub type P0_SET_DATA_REG = crate::Reg<p0_set_data_reg::P0_SET_DATA_REG_SPEC>;
#[doc = "P0 Set port pins Register"]
pub mod p0_set_data_reg;
#[doc = "PAD_WEAK_CTRL_REG register accessor: an alias for `Reg<PAD_WEAK_CTRL_REG_SPEC>`"]
pub type PAD_WEAK_CTRL_REG = crate::Reg<pad_weak_ctrl_reg::PAD_WEAK_CTRL_REG_SPEC>;
#[doc = "Pad driving strength control Register"]
pub mod pad_weak_ctrl_reg;
#[doc = "ROMBIST_RESULTH_REG register accessor: an alias for `Reg<ROMBIST_RESULTH_REG_SPEC>`"]
pub type ROMBIST_RESULTH_REG = crate::Reg<rombist_resulth_reg::ROMBIST_RESULTH_REG_SPEC>;
#[doc = ""]
pub mod rombist_resulth_reg;
#[doc = "ROMBIST_RESULTL_REG register accessor: an alias for `Reg<ROMBIST_RESULTL_REG_SPEC>`"]
pub type ROMBIST_RESULTL_REG = crate::Reg<rombist_resultl_reg::ROMBIST_RESULTL_REG_SPEC>;
#[doc = ""]
pub mod rombist_resultl_reg;
#[doc = "SCAN_OBSERVE_REG register accessor: an alias for `Reg<SCAN_OBSERVE_REG_SPEC>`"]
pub type SCAN_OBSERVE_REG = crate::Reg<scan_observe_reg::SCAN_OBSERVE_REG_SPEC>;
#[doc = ""]
pub mod scan_observe_reg;
#[doc = "TEST_CTRL2_REG register accessor: an alias for `Reg<TEST_CTRL2_REG_SPEC>`"]
pub type TEST_CTRL2_REG = crate::Reg<test_ctrl2_reg::TEST_CTRL2_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl2_reg;
#[doc = "TEST_CTRL3_REG register accessor: an alias for `Reg<TEST_CTRL3_REG_SPEC>`"]
pub type TEST_CTRL3_REG = crate::Reg<test_ctrl3_reg::TEST_CTRL3_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl3_reg;
#[doc = "TEST_CTRL4_REG register accessor: an alias for `Reg<TEST_CTRL4_REG_SPEC>`"]
pub type TEST_CTRL4_REG = crate::Reg<test_ctrl4_reg::TEST_CTRL4_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl4_reg;
#[doc = "TEST_CTRL_REG register accessor: an alias for `Reg<TEST_CTRL_REG_SPEC>`"]
pub type TEST_CTRL_REG = crate::Reg<test_ctrl_reg::TEST_CTRL_REG_SPEC>;
#[doc = ""]
pub mod test_ctrl_reg;
#[doc = "XTAL32M_TESTCTRL0_REG register accessor: an alias for `Reg<XTAL32M_TESTCTRL0_REG_SPEC>`"]
pub type XTAL32M_TESTCTRL0_REG = crate::Reg<xtal32m_testctrl0_reg::XTAL32M_TESTCTRL0_REG_SPEC>;
#[doc = ""]
pub mod xtal32m_testctrl0_reg;
#[doc = "XTAL32M_TESTCTRL1_REG register accessor: an alias for `Reg<XTAL32M_TESTCTRL1_REG_SPEC>`"]
pub type XTAL32M_TESTCTRL1_REG = crate::Reg<xtal32m_testctrl1_reg::XTAL32M_TESTCTRL1_REG_SPEC>;
#[doc = ""]
pub mod xtal32m_testctrl1_reg;
