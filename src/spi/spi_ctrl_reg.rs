#[doc = "Register `SPI_CTRL_REG` reader"]
pub struct R(crate::R<SPI_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL_REG` writer"]
pub struct W(crate::W<SPI_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL_REG_SPEC>;
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
impl From<crate::W<SPI_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SWAP_BYTES` reader - 0 = normal operation 1 = LSB and MSB are swaped in APB interface In case of 8bit spi interface, DMA/SPI can be configured in 16bit mode to off load the bus. Enabling SPI_SWAP_BYTES bytes will read/wrte correctly"]
pub struct SPI_SWAP_BYTES_R(crate::FieldReader<bool, bool>);
impl SPI_SWAP_BYTES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SWAP_BYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SWAP_BYTES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SWAP_BYTES` writer - 0 = normal operation 1 = LSB and MSB are swaped in APB interface In case of 8bit spi interface, DMA/SPI can be configured in 16bit mode to off load the bus. Enabling SPI_SWAP_BYTES bytes will read/wrte correctly"]
pub struct SPI_SWAP_BYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SWAP_BYTES_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `SPI_CAPTURE_AT_NEXT_EDGE` reader - 0 = SPI captures data at correct clock edge 1 = SPI captures data at next clock edge. (only for Master mode and high clock)"]
pub struct SPI_CAPTURE_AT_NEXT_EDGE_R(crate::FieldReader<bool, bool>);
impl SPI_CAPTURE_AT_NEXT_EDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_CAPTURE_AT_NEXT_EDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CAPTURE_AT_NEXT_EDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_CAPTURE_AT_NEXT_EDGE` writer - 0 = SPI captures data at correct clock edge 1 = SPI captures data at next clock edge. (only for Master mode and high clock)"]
pub struct SPI_CAPTURE_AT_NEXT_EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CAPTURE_AT_NEXT_EDGE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `SPI_FIFO_RESET` reader - 0 = Fifo normal operation 1 = Fifo in reset state"]
pub struct SPI_FIFO_RESET_R(crate::FieldReader<bool, bool>);
impl SPI_FIFO_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_FIFO_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_FIFO_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_FIFO_RESET` writer - 0 = Fifo normal operation 1 = Fifo in reset state"]
pub struct SPI_FIFO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FIFO_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u16 & 1) << 5);
        self.w
    }
}
#[doc = "Field `SPI_DMA_RX_EN` reader - applicable only when SPI_RX_EN=1 0 = No DMA request for RX 1 = DMA request when SPI_STATUS_RX_FULL='1'"]
pub struct SPI_DMA_RX_EN_R(crate::FieldReader<bool, bool>);
impl SPI_DMA_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_DMA_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_DMA_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_DMA_RX_EN` writer - applicable only when SPI_RX_EN=1 0 = No DMA request for RX 1 = DMA request when SPI_STATUS_RX_FULL='1'"]
pub struct SPI_DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_RX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u16 & 1) << 4);
        self.w
    }
}
#[doc = "Field `SPI_DMA_TX_EN` reader - applicable only when SPI_TX_EN=1 0 = No DMA request for TX 1 = DMA request when SPI_STATUS_TX_EMPTY='1'"]
pub struct SPI_DMA_TX_EN_R(crate::FieldReader<bool, bool>);
impl SPI_DMA_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_DMA_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_DMA_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_DMA_TX_EN` writer - applicable only when SPI_TX_EN=1 0 = No DMA request for TX 1 = DMA request when SPI_STATUS_TX_EMPTY='1'"]
pub struct SPI_DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DMA_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u16 & 1) << 3);
        self.w
    }
}
#[doc = "Field `SPI_RX_EN` reader - 0 = RX path is disabled 1 = RX path is enabled Note: if master clk async or spi mode=1 or spi mode=3 readonly is not supported"]
pub struct SPI_RX_EN_R(crate::FieldReader<bool, bool>);
impl SPI_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_RX_EN` writer - 0 = RX path is disabled 1 = RX path is enabled Note: if master clk async or spi mode=1 or spi mode=3 readonly is not supported"]
pub struct SPI_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `SPI_TX_EN` reader - 0 = TX path is disabled 1 = TX path is enabled"]
pub struct SPI_TX_EN_R(crate::FieldReader<bool, bool>);
impl SPI_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_TX_EN` writer - 0 = TX path is disabled 1 = TX path is enabled"]
pub struct SPI_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TX_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `SPI_EN` reader - 0 = SPI module is disable 1 = SPI module is enable"]
pub struct SPI_EN_R(crate::FieldReader<bool, bool>);
impl SPI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_EN` writer - 0 = SPI module is disable 1 = SPI module is enable"]
pub struct SPI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 0 = normal operation 1 = LSB and MSB are swaped in APB interface In case of 8bit spi interface, DMA/SPI can be configured in 16bit mode to off load the bus. Enabling SPI_SWAP_BYTES bytes will read/wrte correctly"]
    #[inline(always)]
    pub fn spi_swap_bytes(&self) -> SPI_SWAP_BYTES_R {
        SPI_SWAP_BYTES_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = SPI captures data at correct clock edge 1 = SPI captures data at next clock edge. (only for Master mode and high clock)"]
    #[inline(always)]
    pub fn spi_capture_at_next_edge(&self) -> SPI_CAPTURE_AT_NEXT_EDGE_R {
        SPI_CAPTURE_AT_NEXT_EDGE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = Fifo normal operation 1 = Fifo in reset state"]
    #[inline(always)]
    pub fn spi_fifo_reset(&self) -> SPI_FIFO_RESET_R {
        SPI_FIFO_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - applicable only when SPI_RX_EN=1 0 = No DMA request for RX 1 = DMA request when SPI_STATUS_RX_FULL='1'"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&self) -> SPI_DMA_RX_EN_R {
        SPI_DMA_RX_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - applicable only when SPI_TX_EN=1 0 = No DMA request for TX 1 = DMA request when SPI_STATUS_TX_EMPTY='1'"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&self) -> SPI_DMA_TX_EN_R {
        SPI_DMA_TX_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - 0 = RX path is disabled 1 = RX path is enabled Note: if master clk async or spi mode=1 or spi mode=3 readonly is not supported"]
    #[inline(always)]
    pub fn spi_rx_en(&self) -> SPI_RX_EN_R {
        SPI_RX_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - 0 = TX path is disabled 1 = TX path is enabled"]
    #[inline(always)]
    pub fn spi_tx_en(&self) -> SPI_TX_EN_R {
        SPI_TX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - 0 = SPI module is disable 1 = SPI module is enable"]
    #[inline(always)]
    pub fn spi_en(&self) -> SPI_EN_R {
        SPI_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - 0 = normal operation 1 = LSB and MSB are swaped in APB interface In case of 8bit spi interface, DMA/SPI can be configured in 16bit mode to off load the bus. Enabling SPI_SWAP_BYTES bytes will read/wrte correctly"]
    #[inline(always)]
    pub fn spi_swap_bytes(&mut self) -> SPI_SWAP_BYTES_W {
        SPI_SWAP_BYTES_W { w: self }
    }
    #[doc = "Bit 6 - 0 = SPI captures data at correct clock edge 1 = SPI captures data at next clock edge. (only for Master mode and high clock)"]
    #[inline(always)]
    pub fn spi_capture_at_next_edge(&mut self) -> SPI_CAPTURE_AT_NEXT_EDGE_W {
        SPI_CAPTURE_AT_NEXT_EDGE_W { w: self }
    }
    #[doc = "Bit 5 - 0 = Fifo normal operation 1 = Fifo in reset state"]
    #[inline(always)]
    pub fn spi_fifo_reset(&mut self) -> SPI_FIFO_RESET_W {
        SPI_FIFO_RESET_W { w: self }
    }
    #[doc = "Bit 4 - applicable only when SPI_RX_EN=1 0 = No DMA request for RX 1 = DMA request when SPI_STATUS_RX_FULL='1'"]
    #[inline(always)]
    pub fn spi_dma_rx_en(&mut self) -> SPI_DMA_RX_EN_W {
        SPI_DMA_RX_EN_W { w: self }
    }
    #[doc = "Bit 3 - applicable only when SPI_TX_EN=1 0 = No DMA request for TX 1 = DMA request when SPI_STATUS_TX_EMPTY='1'"]
    #[inline(always)]
    pub fn spi_dma_tx_en(&mut self) -> SPI_DMA_TX_EN_W {
        SPI_DMA_TX_EN_W { w: self }
    }
    #[doc = "Bit 2 - 0 = RX path is disabled 1 = RX path is enabled Note: if master clk async or spi mode=1 or spi mode=3 readonly is not supported"]
    #[inline(always)]
    pub fn spi_rx_en(&mut self) -> SPI_RX_EN_W {
        SPI_RX_EN_W { w: self }
    }
    #[doc = "Bit 1 - 0 = TX path is disabled 1 = TX path is enabled"]
    #[inline(always)]
    pub fn spi_tx_en(&mut self) -> SPI_TX_EN_W {
        SPI_TX_EN_W { w: self }
    }
    #[doc = "Bit 0 - 0 = SPI module is disable 1 = SPI module is enable"]
    #[inline(always)]
    pub fn spi_en(&mut self) -> SPI_EN_W {
        SPI_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spi control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl_reg](index.html) module"]
pub struct SPI_CTRL_REG_SPEC;
impl crate::RegisterSpec for SPI_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_ctrl_reg::R](R) reader structure"]
impl crate::Readable for SPI_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl_reg::W](W) writer structure"]
impl crate::Writable for SPI_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CTRL_REG to value 0"]
impl crate::Resettable for SPI_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
