#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO interrupt selection for GPIO_IRQ0"]
    pub gpio_irq0_in_sel_reg: crate::Reg<gpio_irq0_in_sel_reg::GPIO_IRQ0_IN_SEL_REG_SPEC>,
    #[doc = "0x02 - GPIO interrupt selection for GPIO_IRQ1"]
    pub gpio_irq1_in_sel_reg: crate::Reg<gpio_irq1_in_sel_reg::GPIO_IRQ1_IN_SEL_REG_SPEC>,
    #[doc = "0x04 - GPIO interrupt selection for GPIO_IRQ2"]
    pub gpio_irq2_in_sel_reg: crate::Reg<gpio_irq2_in_sel_reg::GPIO_IRQ2_IN_SEL_REG_SPEC>,
    #[doc = "0x06 - GPIO interrupt selection for GPIO_IRQ3"]
    pub gpio_irq3_in_sel_reg: crate::Reg<gpio_irq3_in_sel_reg::GPIO_IRQ3_IN_SEL_REG_SPEC>,
    #[doc = "0x08 - GPIO interrupt selection for GPIO_IRQ4"]
    pub gpio_irq4_in_sel_reg: crate::Reg<gpio_irq4_in_sel_reg::GPIO_IRQ4_IN_SEL_REG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x0c - debounce counter value for GPIO inputs"]
    pub gpio_debounce_reg: crate::Reg<gpio_debounce_reg::GPIO_DEBOUNCE_REG_SPEC>,
    #[doc = "0x0e - GPIO interrupt reset register"]
    pub gpio_reset_irq_reg: crate::Reg<gpio_reset_irq_reg::GPIO_RESET_IRQ_REG_SPEC>,
    #[doc = "0x10 - high or low level select for GPIO interrupts"]
    pub gpio_int_level_ctrl_reg: crate::Reg<gpio_int_level_ctrl_reg::GPIO_INT_LEVEL_CTRL_REG_SPEC>,
    #[doc = "0x12 - GPIO interrupt selection for KBRD_IRQ for P0"]
    pub kbrd_irq_in_sel0_reg: crate::Reg<kbrd_irq_in_sel0_reg::KBRD_IRQ_IN_SEL0_REG_SPEC>,
    #[doc = "0x14 - GPIO Kbrd control register"]
    pub kbrd_ctrl_reg: crate::Reg<kbrd_ctrl_reg::KBRD_CTRL_REG_SPEC>,
}
#[doc = "GPIO_DEBOUNCE_REG register accessor: an alias for `Reg<GPIO_DEBOUNCE_REG_SPEC>`"]
pub type GPIO_DEBOUNCE_REG = crate::Reg<gpio_debounce_reg::GPIO_DEBOUNCE_REG_SPEC>;
#[doc = "debounce counter value for GPIO inputs"]
pub mod gpio_debounce_reg;
#[doc = "GPIO_INT_LEVEL_CTRL_REG register accessor: an alias for `Reg<GPIO_INT_LEVEL_CTRL_REG_SPEC>`"]
pub type GPIO_INT_LEVEL_CTRL_REG =
    crate::Reg<gpio_int_level_ctrl_reg::GPIO_INT_LEVEL_CTRL_REG_SPEC>;
#[doc = "high or low level select for GPIO interrupts"]
pub mod gpio_int_level_ctrl_reg;
#[doc = "GPIO_IRQ0_IN_SEL_REG register accessor: an alias for `Reg<GPIO_IRQ0_IN_SEL_REG_SPEC>`"]
pub type GPIO_IRQ0_IN_SEL_REG = crate::Reg<gpio_irq0_in_sel_reg::GPIO_IRQ0_IN_SEL_REG_SPEC>;
#[doc = "GPIO interrupt selection for GPIO_IRQ0"]
pub mod gpio_irq0_in_sel_reg;
#[doc = "GPIO_IRQ1_IN_SEL_REG register accessor: an alias for `Reg<GPIO_IRQ1_IN_SEL_REG_SPEC>`"]
pub type GPIO_IRQ1_IN_SEL_REG = crate::Reg<gpio_irq1_in_sel_reg::GPIO_IRQ1_IN_SEL_REG_SPEC>;
#[doc = "GPIO interrupt selection for GPIO_IRQ1"]
pub mod gpio_irq1_in_sel_reg;
#[doc = "GPIO_IRQ2_IN_SEL_REG register accessor: an alias for `Reg<GPIO_IRQ2_IN_SEL_REG_SPEC>`"]
pub type GPIO_IRQ2_IN_SEL_REG = crate::Reg<gpio_irq2_in_sel_reg::GPIO_IRQ2_IN_SEL_REG_SPEC>;
#[doc = "GPIO interrupt selection for GPIO_IRQ2"]
pub mod gpio_irq2_in_sel_reg;
#[doc = "GPIO_IRQ3_IN_SEL_REG register accessor: an alias for `Reg<GPIO_IRQ3_IN_SEL_REG_SPEC>`"]
pub type GPIO_IRQ3_IN_SEL_REG = crate::Reg<gpio_irq3_in_sel_reg::GPIO_IRQ3_IN_SEL_REG_SPEC>;
#[doc = "GPIO interrupt selection for GPIO_IRQ3"]
pub mod gpio_irq3_in_sel_reg;
#[doc = "GPIO_IRQ4_IN_SEL_REG register accessor: an alias for `Reg<GPIO_IRQ4_IN_SEL_REG_SPEC>`"]
pub type GPIO_IRQ4_IN_SEL_REG = crate::Reg<gpio_irq4_in_sel_reg::GPIO_IRQ4_IN_SEL_REG_SPEC>;
#[doc = "GPIO interrupt selection for GPIO_IRQ4"]
pub mod gpio_irq4_in_sel_reg;
#[doc = "GPIO_RESET_IRQ_REG register accessor: an alias for `Reg<GPIO_RESET_IRQ_REG_SPEC>`"]
pub type GPIO_RESET_IRQ_REG = crate::Reg<gpio_reset_irq_reg::GPIO_RESET_IRQ_REG_SPEC>;
#[doc = "GPIO interrupt reset register"]
pub mod gpio_reset_irq_reg;
#[doc = "KBRD_CTRL_REG register accessor: an alias for `Reg<KBRD_CTRL_REG_SPEC>`"]
pub type KBRD_CTRL_REG = crate::Reg<kbrd_ctrl_reg::KBRD_CTRL_REG_SPEC>;
#[doc = "GPIO Kbrd control register"]
pub mod kbrd_ctrl_reg;
#[doc = "KBRD_IRQ_IN_SEL0_REG register accessor: an alias for `Reg<KBRD_IRQ_IN_SEL0_REG_SPEC>`"]
pub type KBRD_IRQ_IN_SEL0_REG = crate::Reg<kbrd_irq_in_sel0_reg::KBRD_IRQ_IN_SEL0_REG_SPEC>;
#[doc = "GPIO interrupt selection for KBRD_IRQ for P0"]
pub mod kbrd_irq_in_sel0_reg;
