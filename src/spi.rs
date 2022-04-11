#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Spi control register"]
    pub spi_ctrl_reg: crate::Reg<spi_ctrl_reg::SPI_CTRL_REG_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Spi control register"]
    pub spi_config_reg: crate::Reg<spi_config_reg::SPI_CONFIG_REG_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - Spi clock register"]
    pub spi_clock_reg: crate::Reg<spi_clock_reg::SPI_CLOCK_REG_SPEC>,
    _reserved3: [u8; 0x02],
    #[doc = "0x0c - Spi fifo configuration register"]
    pub spi_fifo_config_reg: crate::Reg<spi_fifo_config_reg::SPI_FIFO_CONFIG_REG_SPEC>,
    _reserved4: [u8; 0x02],
    #[doc = "0x10 - Spi interrupt mask register"]
    pub spi_irq_mask_reg: crate::Reg<spi_irq_mask_reg::SPI_IRQ_MASK_REG_SPEC>,
    _reserved5: [u8; 0x02],
    #[doc = "0x14 - Spi status register"]
    pub spi_status_reg: crate::Reg<spi_status_reg::SPI_STATUS_REG_SPEC>,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - SPI RX/TX fifo status register"]
    pub spi_fifo_status_reg: crate::Reg<spi_fifo_status_reg::SPI_FIFO_STATUS_REG_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - Spi RX fifo read register"]
    pub spi_fifo_read_reg: crate::Reg<spi_fifo_read_reg::SPI_FIFO_READ_REG_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x20 - Spi TX fifo wtite register"]
    pub spi_fifo_write_reg: crate::Reg<spi_fifo_write_reg::SPI_FIFO_WRITE_REG_SPEC>,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - Spi cs configuration register"]
    pub spi_cs_config_reg: crate::Reg<spi_cs_config_reg::SPI_CS_CONFIG_REG_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - Spi TX/RX High 16bit word"]
    pub spi_fifo_high_reg: crate::Reg<spi_fifo_high_reg::SPI_FIFO_HIGH_REG_SPEC>,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - SPI TX buffer force low value"]
    pub spi_txbuffer_force_l_reg:
        crate::Reg<spi_txbuffer_force_l_reg::SPI_TXBUFFER_FORCE_L_REG_SPEC>,
    _reserved12: [u8; 0x02],
    #[doc = "0x30 - SPI TX buffer force high value"]
    pub spi_txbuffer_force_h_reg:
        crate::Reg<spi_txbuffer_force_h_reg::SPI_TXBUFFER_FORCE_H_REG_SPEC>,
}
#[doc = "SPI_CLOCK_REG register accessor: an alias for `Reg<SPI_CLOCK_REG_SPEC>`"]
pub type SPI_CLOCK_REG = crate::Reg<spi_clock_reg::SPI_CLOCK_REG_SPEC>;
#[doc = "Spi clock register"]
pub mod spi_clock_reg;
#[doc = "SPI_CONFIG_REG register accessor: an alias for `Reg<SPI_CONFIG_REG_SPEC>`"]
pub type SPI_CONFIG_REG = crate::Reg<spi_config_reg::SPI_CONFIG_REG_SPEC>;
#[doc = "Spi control register"]
pub mod spi_config_reg;
#[doc = "SPI_CS_CONFIG_REG register accessor: an alias for `Reg<SPI_CS_CONFIG_REG_SPEC>`"]
pub type SPI_CS_CONFIG_REG = crate::Reg<spi_cs_config_reg::SPI_CS_CONFIG_REG_SPEC>;
#[doc = "Spi cs configuration register"]
pub mod spi_cs_config_reg;
#[doc = "SPI_CTRL_REG register accessor: an alias for `Reg<SPI_CTRL_REG_SPEC>`"]
pub type SPI_CTRL_REG = crate::Reg<spi_ctrl_reg::SPI_CTRL_REG_SPEC>;
#[doc = "Spi control register"]
pub mod spi_ctrl_reg;
#[doc = "SPI_FIFO_CONFIG_REG register accessor: an alias for `Reg<SPI_FIFO_CONFIG_REG_SPEC>`"]
pub type SPI_FIFO_CONFIG_REG = crate::Reg<spi_fifo_config_reg::SPI_FIFO_CONFIG_REG_SPEC>;
#[doc = "Spi fifo configuration register"]
pub mod spi_fifo_config_reg;
#[doc = "SPI_FIFO_HIGH_REG register accessor: an alias for `Reg<SPI_FIFO_HIGH_REG_SPEC>`"]
pub type SPI_FIFO_HIGH_REG = crate::Reg<spi_fifo_high_reg::SPI_FIFO_HIGH_REG_SPEC>;
#[doc = "Spi TX/RX High 16bit word"]
pub mod spi_fifo_high_reg;
#[doc = "SPI_FIFO_READ_REG register accessor: an alias for `Reg<SPI_FIFO_READ_REG_SPEC>`"]
pub type SPI_FIFO_READ_REG = crate::Reg<spi_fifo_read_reg::SPI_FIFO_READ_REG_SPEC>;
#[doc = "Spi RX fifo read register"]
pub mod spi_fifo_read_reg;
#[doc = "SPI_FIFO_STATUS_REG register accessor: an alias for `Reg<SPI_FIFO_STATUS_REG_SPEC>`"]
pub type SPI_FIFO_STATUS_REG = crate::Reg<spi_fifo_status_reg::SPI_FIFO_STATUS_REG_SPEC>;
#[doc = "SPI RX/TX fifo status register"]
pub mod spi_fifo_status_reg;
#[doc = "SPI_FIFO_WRITE_REG register accessor: an alias for `Reg<SPI_FIFO_WRITE_REG_SPEC>`"]
pub type SPI_FIFO_WRITE_REG = crate::Reg<spi_fifo_write_reg::SPI_FIFO_WRITE_REG_SPEC>;
#[doc = "Spi TX fifo wtite register"]
pub mod spi_fifo_write_reg;
#[doc = "SPI_IRQ_MASK_REG register accessor: an alias for `Reg<SPI_IRQ_MASK_REG_SPEC>`"]
pub type SPI_IRQ_MASK_REG = crate::Reg<spi_irq_mask_reg::SPI_IRQ_MASK_REG_SPEC>;
#[doc = "Spi interrupt mask register"]
pub mod spi_irq_mask_reg;
#[doc = "SPI_STATUS_REG register accessor: an alias for `Reg<SPI_STATUS_REG_SPEC>`"]
pub type SPI_STATUS_REG = crate::Reg<spi_status_reg::SPI_STATUS_REG_SPEC>;
#[doc = "Spi status register"]
pub mod spi_status_reg;
#[doc = "SPI_TXBUFFER_FORCE_H_REG register accessor: an alias for `Reg<SPI_TXBUFFER_FORCE_H_REG_SPEC>`"]
pub type SPI_TXBUFFER_FORCE_H_REG =
    crate::Reg<spi_txbuffer_force_h_reg::SPI_TXBUFFER_FORCE_H_REG_SPEC>;
#[doc = "SPI TX buffer force high value"]
pub mod spi_txbuffer_force_h_reg;
#[doc = "SPI_TXBUFFER_FORCE_L_REG register accessor: an alias for `Reg<SPI_TXBUFFER_FORCE_L_REG_SPEC>`"]
pub type SPI_TXBUFFER_FORCE_L_REG =
    crate::Reg<spi_txbuffer_force_l_reg::SPI_TXBUFFER_FORCE_L_REG_SPEC>;
#[doc = "SPI TX buffer force low value"]
pub mod spi_txbuffer_force_l_reg;
