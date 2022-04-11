#[doc = "Register `SPI_FIFO_READ_REG` reader"]
pub struct R(crate::R<SPI_FIFO_READ_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_READ_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FIFO_READ_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FIFO_READ_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FIFO_READ_REG` writer"]
pub struct W(crate::W<SPI_FIFO_READ_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FIFO_READ_REG_SPEC>;
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
impl From<crate::W<SPI_FIFO_READ_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FIFO_READ_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_FIFO_READ` reader - Read from RX fifo. Read access is permit only if SPI_STATUS_RX_EMPTY=0. Returns the 16 LSb"]
pub struct SPI_FIFO_READ_R(crate::FieldReader<u16, u16>);
impl SPI_FIFO_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SPI_FIFO_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_FIFO_READ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Read from RX fifo. Read access is permit only if SPI_STATUS_RX_EMPTY=0. Returns the 16 LSb"]
    #[inline(always)]
    pub fn spi_fifo_read(&self) -> SPI_FIFO_READ_R {
        SPI_FIFO_READ_R::new(self.bits)
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
#[doc = "Spi RX fifo read register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_read_reg](index.html) module"]
pub struct SPI_FIFO_READ_REG_SPEC;
impl crate::RegisterSpec for SPI_FIFO_READ_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_fifo_read_reg::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_READ_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fifo_read_reg::W](W) writer structure"]
impl crate::Writable for SPI_FIFO_READ_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_FIFO_READ_REG to value 0"]
impl crate::Resettable for SPI_FIFO_READ_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
