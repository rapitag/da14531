#[doc = "Register `SPI_TXBUFFER_FORCE_H_REG` reader"]
pub struct R(crate::R<SPI_TXBUFFER_FORCE_H_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_TXBUFFER_FORCE_H_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_TXBUFFER_FORCE_H_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_TXBUFFER_FORCE_H_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_TXBUFFER_FORCE_H_REG` writer"]
pub struct W(crate::W<SPI_TXBUFFER_FORCE_H_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_TXBUFFER_FORCE_H_REG_SPEC>;
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
impl From<crate::W<SPI_TXBUFFER_FORCE_H_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_TXBUFFER_FORCE_H_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_TXBUFFER_FORCE_H` writer - Write directly the tx buffer (2 MSB). It must to be used only in slave mode. This register has to be written before the SPI_FIFO_WRITE_REG"]
pub struct SPI_TXBUFFER_FORCE_H_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TXBUFFER_FORCE_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Write directly the tx buffer (2 MSB). It must to be used only in slave mode. This register has to be written before the SPI_FIFO_WRITE_REG"]
    #[inline(always)]
    pub fn spi_txbuffer_force_h(&mut self) -> SPI_TXBUFFER_FORCE_H_W {
        SPI_TXBUFFER_FORCE_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI TX buffer force high value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_txbuffer_force_h_reg](index.html) module"]
pub struct SPI_TXBUFFER_FORCE_H_REG_SPEC;
impl crate::RegisterSpec for SPI_TXBUFFER_FORCE_H_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_txbuffer_force_h_reg::R](R) reader structure"]
impl crate::Readable for SPI_TXBUFFER_FORCE_H_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_txbuffer_force_h_reg::W](W) writer structure"]
impl crate::Writable for SPI_TXBUFFER_FORCE_H_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_TXBUFFER_FORCE_H_REG to value 0"]
impl crate::Resettable for SPI_TXBUFFER_FORCE_H_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
