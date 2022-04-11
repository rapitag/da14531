#[doc = "Register `SPI_STATUS_REG` reader"]
pub struct R(crate::R<SPI_STATUS_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STATUS_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_STATUS_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_STATUS_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_STATUS_REG` writer"]
pub struct W(crate::W<SPI_STATUS_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_STATUS_REG_SPEC>;
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
impl From<crate::W<SPI_STATUS_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_STATUS_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_STATUS_RX_FULL` reader - Auto clear 0 = RX fifo level is less than SPI_RX_TL+1 1 = RX fifo level is more or equal to SPI_RX_TL+1"]
pub struct SPI_STATUS_RX_FULL_R(crate::FieldReader<bool, bool>);
impl SPI_STATUS_RX_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_STATUS_RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_STATUS_RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_STATUS_TX_EMPTY` reader - Auto clear 0 = TX fifo level is larger than SPI_TX_TL 1 = TX fifo level is less or equal to SPI_TX_TL"]
pub struct SPI_STATUS_TX_EMPTY_R(crate::FieldReader<bool, bool>);
impl SPI_STATUS_TX_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_STATUS_TX_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_STATUS_TX_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Auto clear 0 = RX fifo level is less than SPI_RX_TL+1 1 = RX fifo level is more or equal to SPI_RX_TL+1"]
    #[inline(always)]
    pub fn spi_status_rx_full(&self) -> SPI_STATUS_RX_FULL_R {
        SPI_STATUS_RX_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Auto clear 0 = TX fifo level is larger than SPI_TX_TL 1 = TX fifo level is less or equal to SPI_TX_TL"]
    #[inline(always)]
    pub fn spi_status_tx_empty(&self) -> SPI_STATUS_TX_EMPTY_R {
        SPI_STATUS_TX_EMPTY_R::new((self.bits & 1) != 0)
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
#[doc = "Spi status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_status_reg](index.html) module"]
pub struct SPI_STATUS_REG_SPEC;
impl crate::RegisterSpec for SPI_STATUS_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_status_reg::R](R) reader structure"]
impl crate::Readable for SPI_STATUS_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_status_reg::W](W) writer structure"]
impl crate::Writable for SPI_STATUS_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_STATUS_REG to value 0x01"]
impl crate::Resettable for SPI_STATUS_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
