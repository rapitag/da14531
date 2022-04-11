#[doc = "Register `SPI_FIFO_CONFIG_REG` reader"]
pub struct R(crate::R<SPI_FIFO_CONFIG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_CONFIG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FIFO_CONFIG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FIFO_CONFIG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FIFO_CONFIG_REG` writer"]
pub struct W(crate::W<SPI_FIFO_CONFIG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FIFO_CONFIG_REG_SPEC>;
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
impl From<crate::W<SPI_FIFO_CONFIG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FIFO_CONFIG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_RX_TL` reader - Receive FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the RX_FULL interrupt. IRQ is occurred when fifo level is more or equal to SPI_RX_TL+1. Fifo level is from 0 to 4"]
pub struct SPI_RX_TL_R(crate::FieldReader<u8, u8>);
impl SPI_RX_TL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_RX_TL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_RX_TL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_RX_TL` writer - Receive FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the RX_FULL interrupt. IRQ is occurred when fifo level is more or equal to SPI_RX_TL+1. Fifo level is from 0 to 4"]
pub struct SPI_RX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u16 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SPI_TX_TL` reader - Transmit FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the TX_EMPTY interrupt. IRQ is occurred when fifo level is less or equal to SPI_TX_TL. Fifo level is from 0 to 4"]
pub struct SPI_TX_TL_R(crate::FieldReader<u8, u8>);
impl SPI_TX_TL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_TX_TL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_TX_TL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_TX_TL` writer - Transmit FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the TX_EMPTY interrupt. IRQ is occurred when fifo level is less or equal to SPI_TX_TL. Fifo level is from 0 to 4"]
pub struct SPI_TX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - Receive FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the RX_FULL interrupt. IRQ is occurred when fifo level is more or equal to SPI_RX_TL+1. Fifo level is from 0 to 4"]
    #[inline(always)]
    pub fn spi_rx_tl(&self) -> SPI_RX_TL_R {
        SPI_RX_TL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Transmit FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the TX_EMPTY interrupt. IRQ is occurred when fifo level is less or equal to SPI_TX_TL. Fifo level is from 0 to 4"]
    #[inline(always)]
    pub fn spi_tx_tl(&self) -> SPI_TX_TL_R {
        SPI_TX_TL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Receive FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the RX_FULL interrupt. IRQ is occurred when fifo level is more or equal to SPI_RX_TL+1. Fifo level is from 0 to 4"]
    #[inline(always)]
    pub fn spi_rx_tl(&mut self) -> SPI_RX_TL_W {
        SPI_RX_TL_W { w: self }
    }
    #[doc = "Bits 0:3 - Transmit FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the TX_EMPTY interrupt. IRQ is occurred when fifo level is less or equal to SPI_TX_TL. Fifo level is from 0 to 4"]
    #[inline(always)]
    pub fn spi_tx_tl(&mut self) -> SPI_TX_TL_W {
        SPI_TX_TL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spi fifo configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_config_reg](index.html) module"]
pub struct SPI_FIFO_CONFIG_REG_SPEC;
impl crate::RegisterSpec for SPI_FIFO_CONFIG_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_fifo_config_reg::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_CONFIG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fifo_config_reg::W](W) writer structure"]
impl crate::Writable for SPI_FIFO_CONFIG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FIFO_CONFIG_REG to value 0"]
impl crate::Resettable for SPI_FIFO_CONFIG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
