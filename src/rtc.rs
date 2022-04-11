#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register"]
    pub rtc_control_reg: crate::Reg<rtc_control_reg::RTC_CONTROL_REG_SPEC>,
    #[doc = "0x04 - RTC Hour Mode Register"]
    pub rtc_hour_mode_reg: crate::Reg<rtc_hour_mode_reg::RTC_HOUR_MODE_REG_SPEC>,
    #[doc = "0x08 - RTC Time Register"]
    pub rtc_time_reg: crate::Reg<rtc_time_reg::RTC_TIME_REG_SPEC>,
    #[doc = "0x0c - RTC Calendar Register"]
    pub rtc_calendar_reg: crate::Reg<rtc_calendar_reg::RTC_CALENDAR_REG_SPEC>,
    #[doc = "0x10 - RTC Time Alarm Register"]
    pub rtc_time_alarm_reg: crate::Reg<rtc_time_alarm_reg::RTC_TIME_ALARM_REG_SPEC>,
    #[doc = "0x14 - RTC Calendar Alram Register"]
    pub rtc_calendar_alarm_reg: crate::Reg<rtc_calendar_alarm_reg::RTC_CALENDAR_ALARM_REG_SPEC>,
    #[doc = "0x18 - RTC Alarm Enable Register"]
    pub rtc_alarm_enable_reg: crate::Reg<rtc_alarm_enable_reg::RTC_ALARM_ENABLE_REG_SPEC>,
    #[doc = "0x1c - RTC Event Flags Register"]
    pub rtc_event_flags_reg: crate::Reg<rtc_event_flags_reg::RTC_EVENT_FLAGS_REG_SPEC>,
    #[doc = "0x20 - RTC Interrupt Enable Register"]
    pub rtc_interrupt_enable_reg:
        crate::Reg<rtc_interrupt_enable_reg::RTC_INTERRUPT_ENABLE_REG_SPEC>,
    #[doc = "0x24 - RTC Interrupt Disable Register"]
    pub rtc_interrupt_disable_reg:
        crate::Reg<rtc_interrupt_disable_reg::RTC_INTERRUPT_DISABLE_REG_SPEC>,
    #[doc = "0x28 - RTC Interrupt Mask Register"]
    pub rtc_interrupt_mask_reg: crate::Reg<rtc_interrupt_mask_reg::RTC_INTERRUPT_MASK_REG_SPEC>,
    #[doc = "0x2c - RTC Status Register"]
    pub rtc_status_reg: crate::Reg<rtc_status_reg::RTC_STATUS_REG_SPEC>,
    #[doc = "0x30 - RTC Keep RTC Register"]
    pub rtc_keep_rtc_reg: crate::Reg<rtc_keep_rtc_reg::RTC_KEEP_RTC_REG_SPEC>,
}
#[doc = "RTC_ALARM_ENABLE_REG register accessor: an alias for `Reg<RTC_ALARM_ENABLE_REG_SPEC>`"]
pub type RTC_ALARM_ENABLE_REG = crate::Reg<rtc_alarm_enable_reg::RTC_ALARM_ENABLE_REG_SPEC>;
#[doc = "RTC Alarm Enable Register"]
pub mod rtc_alarm_enable_reg;
#[doc = "RTC_CALENDAR_ALARM_REG register accessor: an alias for `Reg<RTC_CALENDAR_ALARM_REG_SPEC>`"]
pub type RTC_CALENDAR_ALARM_REG = crate::Reg<rtc_calendar_alarm_reg::RTC_CALENDAR_ALARM_REG_SPEC>;
#[doc = "RTC Calendar Alram Register"]
pub mod rtc_calendar_alarm_reg;
#[doc = "RTC_CALENDAR_REG register accessor: an alias for `Reg<RTC_CALENDAR_REG_SPEC>`"]
pub type RTC_CALENDAR_REG = crate::Reg<rtc_calendar_reg::RTC_CALENDAR_REG_SPEC>;
#[doc = "RTC Calendar Register"]
pub mod rtc_calendar_reg;
#[doc = "RTC_CONTROL_REG register accessor: an alias for `Reg<RTC_CONTROL_REG_SPEC>`"]
pub type RTC_CONTROL_REG = crate::Reg<rtc_control_reg::RTC_CONTROL_REG_SPEC>;
#[doc = "RTC Control Register"]
pub mod rtc_control_reg;
#[doc = "RTC_EVENT_FLAGS_REG register accessor: an alias for `Reg<RTC_EVENT_FLAGS_REG_SPEC>`"]
pub type RTC_EVENT_FLAGS_REG = crate::Reg<rtc_event_flags_reg::RTC_EVENT_FLAGS_REG_SPEC>;
#[doc = "RTC Event Flags Register"]
pub mod rtc_event_flags_reg;
#[doc = "RTC_HOUR_MODE_REG register accessor: an alias for `Reg<RTC_HOUR_MODE_REG_SPEC>`"]
pub type RTC_HOUR_MODE_REG = crate::Reg<rtc_hour_mode_reg::RTC_HOUR_MODE_REG_SPEC>;
#[doc = "RTC Hour Mode Register"]
pub mod rtc_hour_mode_reg;
#[doc = "RTC_INTERRUPT_DISABLE_REG register accessor: an alias for `Reg<RTC_INTERRUPT_DISABLE_REG_SPEC>`"]
pub type RTC_INTERRUPT_DISABLE_REG =
    crate::Reg<rtc_interrupt_disable_reg::RTC_INTERRUPT_DISABLE_REG_SPEC>;
#[doc = "RTC Interrupt Disable Register"]
pub mod rtc_interrupt_disable_reg;
#[doc = "RTC_INTERRUPT_ENABLE_REG register accessor: an alias for `Reg<RTC_INTERRUPT_ENABLE_REG_SPEC>`"]
pub type RTC_INTERRUPT_ENABLE_REG =
    crate::Reg<rtc_interrupt_enable_reg::RTC_INTERRUPT_ENABLE_REG_SPEC>;
#[doc = "RTC Interrupt Enable Register"]
pub mod rtc_interrupt_enable_reg;
#[doc = "RTC_INTERRUPT_MASK_REG register accessor: an alias for `Reg<RTC_INTERRUPT_MASK_REG_SPEC>`"]
pub type RTC_INTERRUPT_MASK_REG = crate::Reg<rtc_interrupt_mask_reg::RTC_INTERRUPT_MASK_REG_SPEC>;
#[doc = "RTC Interrupt Mask Register"]
pub mod rtc_interrupt_mask_reg;
#[doc = "RTC_KEEP_RTC_REG register accessor: an alias for `Reg<RTC_KEEP_RTC_REG_SPEC>`"]
pub type RTC_KEEP_RTC_REG = crate::Reg<rtc_keep_rtc_reg::RTC_KEEP_RTC_REG_SPEC>;
#[doc = "RTC Keep RTC Register"]
pub mod rtc_keep_rtc_reg;
#[doc = "RTC_STATUS_REG register accessor: an alias for `Reg<RTC_STATUS_REG_SPEC>`"]
pub type RTC_STATUS_REG = crate::Reg<rtc_status_reg::RTC_STATUS_REG_SPEC>;
#[doc = "RTC Status Register"]
pub mod rtc_status_reg;
#[doc = "RTC_TIME_ALARM_REG register accessor: an alias for `Reg<RTC_TIME_ALARM_REG_SPEC>`"]
pub type RTC_TIME_ALARM_REG = crate::Reg<rtc_time_alarm_reg::RTC_TIME_ALARM_REG_SPEC>;
#[doc = "RTC Time Alarm Register"]
pub mod rtc_time_alarm_reg;
#[doc = "RTC_TIME_REG register accessor: an alias for `Reg<RTC_TIME_REG_SPEC>`"]
pub type RTC_TIME_REG = crate::Reg<rtc_time_reg::RTC_TIME_REG_SPEC>;
#[doc = "RTC Time Register"]
pub mod rtc_time_reg;
