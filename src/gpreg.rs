#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Controls freezing of various timers/counters."]
    pub set_freeze_reg: crate::Reg<set_freeze_reg::SET_FREEZE_REG_SPEC>,
    #[doc = "0x02 - Controls unfreezing of various timers/counters."]
    pub reset_freeze_reg: crate::Reg<reset_freeze_reg::RESET_FREEZE_REG_SPEC>,
    #[doc = "0x04 - Various debug information register."]
    pub debug_reg: crate::Reg<debug_reg::DEBUG_REG_SPEC>,
    #[doc = "0x06 - General purpose system status register."]
    pub gp_status_reg: crate::Reg<gp_status_reg::GP_STATUS_REG_SPEC>,
    #[doc = "0x08 - General purpose system control register."]
    pub gp_control_reg: crate::Reg<gp_control_reg::GP_CONTROL_REG_SPEC>,
    #[doc = "0x0a - BLE FINECNT sampled value while in deep sleep state."]
    pub ble_timer_reg: crate::Reg<ble_timer_reg::BLE_TIMER_REG_SPEC>,
    #[doc = "0x0c - "]
    pub mem_ctrl_reg: crate::Reg<mem_ctrl_reg::MEM_CTRL_REG_SPEC>,
}
#[doc = "BLE_TIMER_REG register accessor: an alias for `Reg<BLE_TIMER_REG_SPEC>`"]
pub type BLE_TIMER_REG = crate::Reg<ble_timer_reg::BLE_TIMER_REG_SPEC>;
#[doc = "BLE FINECNT sampled value while in deep sleep state."]
pub mod ble_timer_reg;
#[doc = "DEBUG_REG register accessor: an alias for `Reg<DEBUG_REG_SPEC>`"]
pub type DEBUG_REG = crate::Reg<debug_reg::DEBUG_REG_SPEC>;
#[doc = "Various debug information register."]
pub mod debug_reg;
#[doc = "GP_CONTROL_REG register accessor: an alias for `Reg<GP_CONTROL_REG_SPEC>`"]
pub type GP_CONTROL_REG = crate::Reg<gp_control_reg::GP_CONTROL_REG_SPEC>;
#[doc = "General purpose system control register."]
pub mod gp_control_reg;
#[doc = "GP_STATUS_REG register accessor: an alias for `Reg<GP_STATUS_REG_SPEC>`"]
pub type GP_STATUS_REG = crate::Reg<gp_status_reg::GP_STATUS_REG_SPEC>;
#[doc = "General purpose system status register."]
pub mod gp_status_reg;
#[doc = "MEM_CTRL_REG register accessor: an alias for `Reg<MEM_CTRL_REG_SPEC>`"]
pub type MEM_CTRL_REG = crate::Reg<mem_ctrl_reg::MEM_CTRL_REG_SPEC>;
#[doc = ""]
pub mod mem_ctrl_reg;
#[doc = "RESET_FREEZE_REG register accessor: an alias for `Reg<RESET_FREEZE_REG_SPEC>`"]
pub type RESET_FREEZE_REG = crate::Reg<reset_freeze_reg::RESET_FREEZE_REG_SPEC>;
#[doc = "Controls unfreezing of various timers/counters."]
pub mod reset_freeze_reg;
#[doc = "SET_FREEZE_REG register accessor: an alias for `Reg<SET_FREEZE_REG_SPEC>`"]
pub type SET_FREEZE_REG = crate::Reg<set_freeze_reg::SET_FREEZE_REG_SPEC>;
#[doc = "Controls freezing of various timers/counters."]
pub mod set_freeze_reg;
