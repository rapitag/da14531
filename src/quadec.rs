#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Quad Decoder control register"]
    pub qdec_ctrl_reg: crate::Reg<qdec_ctrl_reg::QDEC_CTRL_REG_SPEC>,
    #[doc = "0x02 - Counter value of the X Axis"]
    pub qdec_xcnt_reg: crate::Reg<qdec_xcnt_reg::QDEC_XCNT_REG_SPEC>,
    #[doc = "0x04 - Counter value of the Y Axis"]
    pub qdec_ycnt_reg: crate::Reg<qdec_ycnt_reg::QDEC_YCNT_REG_SPEC>,
    #[doc = "0x06 - Clock divider register"]
    pub qdec_clockdiv_reg: crate::Reg<qdec_clockdiv_reg::QDEC_CLOCKDIV_REG_SPEC>,
    #[doc = "0x08 - Quad Decoder port selection register"]
    pub qdec_ctrl2_reg: crate::Reg<qdec_ctrl2_reg::QDEC_CTRL2_REG_SPEC>,
    #[doc = "0x0a - Counter value of the Z Axis"]
    pub qdec_zcnt_reg: crate::Reg<qdec_zcnt_reg::QDEC_ZCNT_REG_SPEC>,
    #[doc = "0x0c - Event counter register"]
    pub qdec_event_cnt_reg: crate::Reg<qdec_event_cnt_reg::QDEC_EVENT_CNT_REG_SPEC>,
}
#[doc = "QDEC_CLOCKDIV_REG register accessor: an alias for `Reg<QDEC_CLOCKDIV_REG_SPEC>`"]
pub type QDEC_CLOCKDIV_REG = crate::Reg<qdec_clockdiv_reg::QDEC_CLOCKDIV_REG_SPEC>;
#[doc = "Clock divider register"]
pub mod qdec_clockdiv_reg;
#[doc = "QDEC_CTRL2_REG register accessor: an alias for `Reg<QDEC_CTRL2_REG_SPEC>`"]
pub type QDEC_CTRL2_REG = crate::Reg<qdec_ctrl2_reg::QDEC_CTRL2_REG_SPEC>;
#[doc = "Quad Decoder port selection register"]
pub mod qdec_ctrl2_reg;
#[doc = "QDEC_CTRL_REG register accessor: an alias for `Reg<QDEC_CTRL_REG_SPEC>`"]
pub type QDEC_CTRL_REG = crate::Reg<qdec_ctrl_reg::QDEC_CTRL_REG_SPEC>;
#[doc = "Quad Decoder control register"]
pub mod qdec_ctrl_reg;
#[doc = "QDEC_EVENT_CNT_REG register accessor: an alias for `Reg<QDEC_EVENT_CNT_REG_SPEC>`"]
pub type QDEC_EVENT_CNT_REG = crate::Reg<qdec_event_cnt_reg::QDEC_EVENT_CNT_REG_SPEC>;
#[doc = "Event counter register"]
pub mod qdec_event_cnt_reg;
#[doc = "QDEC_XCNT_REG register accessor: an alias for `Reg<QDEC_XCNT_REG_SPEC>`"]
pub type QDEC_XCNT_REG = crate::Reg<qdec_xcnt_reg::QDEC_XCNT_REG_SPEC>;
#[doc = "Counter value of the X Axis"]
pub mod qdec_xcnt_reg;
#[doc = "QDEC_YCNT_REG register accessor: an alias for `Reg<QDEC_YCNT_REG_SPEC>`"]
pub type QDEC_YCNT_REG = crate::Reg<qdec_ycnt_reg::QDEC_YCNT_REG_SPEC>;
#[doc = "Counter value of the Y Axis"]
pub mod qdec_ycnt_reg;
#[doc = "QDEC_ZCNT_REG register accessor: an alias for `Reg<QDEC_ZCNT_REG_SPEC>`"]
pub type QDEC_ZCNT_REG = crate::Reg<qdec_zcnt_reg::QDEC_ZCNT_REG_SPEC>;
#[doc = "Counter value of the Z Axis"]
pub mod qdec_zcnt_reg;
