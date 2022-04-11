#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog timer register."]
    pub watchdog_reg: crate::Reg<watchdog_reg::WATCHDOG_REG_SPEC>,
    #[doc = "0x02 - Watchdog control register."]
    pub watchdog_ctrl_reg: crate::Reg<watchdog_ctrl_reg::WATCHDOG_CTRL_REG_SPEC>,
}
#[doc = "WATCHDOG_CTRL_REG register accessor: an alias for `Reg<WATCHDOG_CTRL_REG_SPEC>`"]
pub type WATCHDOG_CTRL_REG = crate::Reg<watchdog_ctrl_reg::WATCHDOG_CTRL_REG_SPEC>;
#[doc = "Watchdog control register."]
pub mod watchdog_ctrl_reg;
#[doc = "WATCHDOG_REG register accessor: an alias for `Reg<WATCHDOG_REG_SPEC>`"]
pub type WATCHDOG_REG = crate::Reg<watchdog_reg::WATCHDOG_REG_SPEC>;
#[doc = "Watchdog timer register."]
pub mod watchdog_reg;
