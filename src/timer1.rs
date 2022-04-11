#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer1 control register"]
    pub timer1_ctrl_reg: crate::Reg<timer1_ctrl_reg::TIMER1_CTRL_REG_SPEC>,
    #[doc = "0x04 - Timer1 Capture control register"]
    pub timer1_capture_reg: crate::Reg<timer1_capture_reg::TIMER1_CAPTURE_REG_SPEC>,
    #[doc = "0x08 - Timer1 counter value"]
    pub timer1_status_reg: crate::Reg<timer1_status_reg::TIMER1_STATUS_REG_SPEC>,
    #[doc = "0x0c - Timer1 value for event on GPIO1"]
    pub timer1_capcnt1_value_reg:
        crate::Reg<timer1_capcnt1_value_reg::TIMER1_CAPCNT1_VALUE_REG_SPEC>,
    #[doc = "0x10 - Timer1 value for event on GPIO2"]
    pub timer1_capcnt2_value_reg:
        crate::Reg<timer1_capcnt2_value_reg::TIMER1_CAPCNT2_VALUE_REG_SPEC>,
    #[doc = "0x14 - Clear event register"]
    pub timer1_clr_event_reg: crate::Reg<timer1_clr_event_reg::TIMER1_CLR_EVENT_REG_SPEC>,
}
#[doc = "TIMER1_CAPCNT1_VALUE_REG register accessor: an alias for `Reg<TIMER1_CAPCNT1_VALUE_REG_SPEC>`"]
pub type TIMER1_CAPCNT1_VALUE_REG =
    crate::Reg<timer1_capcnt1_value_reg::TIMER1_CAPCNT1_VALUE_REG_SPEC>;
#[doc = "Timer1 value for event on GPIO1"]
pub mod timer1_capcnt1_value_reg;
#[doc = "TIMER1_CAPCNT2_VALUE_REG register accessor: an alias for `Reg<TIMER1_CAPCNT2_VALUE_REG_SPEC>`"]
pub type TIMER1_CAPCNT2_VALUE_REG =
    crate::Reg<timer1_capcnt2_value_reg::TIMER1_CAPCNT2_VALUE_REG_SPEC>;
#[doc = "Timer1 value for event on GPIO2"]
pub mod timer1_capcnt2_value_reg;
#[doc = "TIMER1_CAPTURE_REG register accessor: an alias for `Reg<TIMER1_CAPTURE_REG_SPEC>`"]
pub type TIMER1_CAPTURE_REG = crate::Reg<timer1_capture_reg::TIMER1_CAPTURE_REG_SPEC>;
#[doc = "Timer1 Capture control register"]
pub mod timer1_capture_reg;
#[doc = "TIMER1_CLR_EVENT_REG register accessor: an alias for `Reg<TIMER1_CLR_EVENT_REG_SPEC>`"]
pub type TIMER1_CLR_EVENT_REG = crate::Reg<timer1_clr_event_reg::TIMER1_CLR_EVENT_REG_SPEC>;
#[doc = "Clear event register"]
pub mod timer1_clr_event_reg;
#[doc = "TIMER1_CTRL_REG register accessor: an alias for `Reg<TIMER1_CTRL_REG_SPEC>`"]
pub type TIMER1_CTRL_REG = crate::Reg<timer1_ctrl_reg::TIMER1_CTRL_REG_SPEC>;
#[doc = "Timer1 control register"]
pub mod timer1_ctrl_reg;
#[doc = "TIMER1_STATUS_REG register accessor: an alias for `Reg<TIMER1_STATUS_REG_SPEC>`"]
pub type TIMER1_STATUS_REG = crate::Reg<timer1_status_reg::TIMER1_STATUS_REG_SPEC>;
#[doc = "Timer1 counter value"]
pub mod timer1_status_reg;
