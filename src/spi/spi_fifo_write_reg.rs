#[doc = "Register `SPI_FIFO_WRITE_REG` reader"]
pub struct R(crate::R<SPI_FIFO_WRITE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_WRITE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FIFO_WRITE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FIFO_WRITE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FIFO_WRITE_REG` writer"]
pub struct W(crate::W<SPI_FIFO_WRITE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FIFO_WRITE_REG_SPEC>;
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
impl From<crate::W<SPI_FIFO_WRITE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FIFO_WRITE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_FIFO_WRITE` writer - Write to TX fifo. Write access is permit only if SPI_STATUS_TX_FULL is 0"]
pub struct SPI_FIFO_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FIFO_WRITE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Write to TX fifo. Write access is permit only if SPI_STATUS_TX_FULL is 0"]
    #[inline(always)]
    pub fn spi_fifo_write(&mut self) -> SPI_FIFO_WRITE_W {
        SPI_FIFO_WRITE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spi TX fifo wtite register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_write_reg](index.html) module"]
pub struct SPI_FIFO_WRITE_REG_SPEC;
impl crate::RegisterSpec for SPI_FIFO_WRITE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_fifo_write_reg::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_WRITE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fifo_write_reg::W](W) writer structure"]
impl crate::Writable for SPI_FIFO_WRITE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FIFO_WRITE_REG to value 0"]
impl crate::Resettable for SPI_FIFO_WRITE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
