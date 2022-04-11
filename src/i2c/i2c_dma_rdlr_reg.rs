#[doc = "Register `I2C_DMA_RDLR_REG` reader"]
pub struct R(crate::R<I2C_DMA_RDLR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_DMA_RDLR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_DMA_RDLR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_DMA_RDLR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_DMA_RDLR_REG` writer"]
pub struct W(crate::W<I2C_DMA_RDLR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_DMA_RDLR_REG_SPEC>;
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
impl From<crate::W<I2C_DMA_RDLR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_DMA_RDLR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARDL` reader - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
pub struct DMARDL_R(crate::FieldReader<u8, u8>);
impl DMARDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMARDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMARDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMARDL` writer - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
pub struct DMARDL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
    #[inline(always)]
    pub fn dmardl(&self) -> DMARDL_R {
        DMARDL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO."]
    #[inline(always)]
    pub fn dmardl(&mut self) -> DMARDL_W {
        DMARDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Receive Data Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_dma_rdlr_reg](index.html) module"]
pub struct I2C_DMA_RDLR_REG_SPEC;
impl crate::RegisterSpec for I2C_DMA_RDLR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_dma_rdlr_reg::R](R) reader structure"]
impl crate::Readable for I2C_DMA_RDLR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_dma_rdlr_reg::W](W) writer structure"]
impl crate::Writable for I2C_DMA_RDLR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_DMA_RDLR_REG to value 0"]
impl crate::Resettable for I2C_DMA_RDLR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
