#[doc = "Register `SPI_FIFO_STATUS_REG` reader"]
pub struct R(crate::R<SPI_FIFO_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FIFO_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FIFO_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FIFO_STATUS_REG` writer"]
pub struct W(crate::W<SPI_FIFO_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FIFO_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_FIFO_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FIFO_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_TRANSACTION_ACTIVE` reader - In master mode 0 = spi transaction is inactive 1 = spi transaction is active"]
pub struct SPI_TRANSACTION_ACTIVE_R(crate::FieldReader<bool, bool>);
impl SPI_TRANSACTION_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_TRANSACTION_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_TRANSACTION_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_RX_FIFO_OVFL` reader - When 1, receive data is not written to fifo because fifo was full and interrupt is generated. It clears with SPI_CTRL_REG.SPI_FIFO_RESET"]
pub struct SPI_RX_FIFO_OVFL_R(crate::FieldReader<bool, bool>);
impl SPI_RX_FIFO_OVFL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_RX_FIFO_OVFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_RX_FIFO_OVFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_STATUS_TX_FULL` reader - 0 = TX fifo is not full 1 = TX fifo is full"]
pub struct SPI_STATUS_TX_FULL_R(crate::FieldReader<bool, bool>);
impl SPI_STATUS_TX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_STATUS_TX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_STATUS_TX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_STATUS_RX_EMPTY` reader - 0 = RX fifo is not empty 1 = RX fifo is empty"]
pub struct SPI_STATUS_RX_EMPTY_R(crate::FieldReader<bool, bool>);
impl SPI_STATUS_RX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_STATUS_RX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_STATUS_RX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_TX_FIFO_LEVEL` reader - Gives the number of bytes in TX fifo"]
pub struct SPI_TX_FIFO_LEVEL_R(crate::FieldReader<u8, u8>);
impl SPI_TX_FIFO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_TX_FIFO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_TX_FIFO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_RX_FIFO_LEVEL` reader - Gives the number of bytes in RX fifo"]
pub struct SPI_RX_FIFO_LEVEL_R(crate::FieldReader<u8, u8>);
impl SPI_RX_FIFO_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_RX_FIFO_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_RX_FIFO_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - In master mode 0 = spi transaction is inactive 1 = spi transaction is active"]
    #[inline(always)]
    pub fn spi_transaction_active(&self) -> SPI_TRANSACTION_ACTIVE_R {
        SPI_TRANSACTION_ACTIVE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, receive data is not written to fifo because fifo was full and interrupt is generated. It clears with SPI_CTRL_REG.SPI_FIFO_RESET"]
    #[inline(always)]
    pub fn spi_rx_fifo_ovfl(&self) -> SPI_RX_FIFO_OVFL_R {
        SPI_RX_FIFO_OVFL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - 0 = TX fifo is not full 1 = TX fifo is full"]
    #[inline(always)]
    pub fn spi_status_tx_full(&self) -> SPI_STATUS_TX_FULL_R {
        SPI_STATUS_TX_FULL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 = RX fifo is not empty 1 = RX fifo is empty"]
    #[inline(always)]
    pub fn spi_status_rx_empty(&self) -> SPI_STATUS_RX_EMPTY_R {
        SPI_STATUS_RX_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 6:11 - Gives the number of bytes in TX fifo"]
    #[inline(always)]
    pub fn spi_tx_fifo_level(&self) -> SPI_TX_FIFO_LEVEL_R {
        SPI_TX_FIFO_LEVEL_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - Gives the number of bytes in RX fifo"]
    #[inline(always)]
    pub fn spi_rx_fifo_level(&self) -> SPI_RX_FIFO_LEVEL_R {
        SPI_RX_FIFO_LEVEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI RX/TX fifo status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_status_reg](index.html) module"]
pub struct SPI_FIFO_STATUS_REG_SPEC;
impl crate::RegisterSpec for SPI_FIFO_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_fifo_status_reg::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fifo_status_reg::W](W) writer structure"]
impl crate::Writable for SPI_FIFO_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FIFO_STATUS_REG to value 0x1000"]
impl crate::Resettable for SPI_FIFO_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000
    }
}
