#[doc = "Register `SPI_CLOCK_REG` reader"]
pub struct R(crate::R<SPI_CLOCK_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CLOCK_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CLOCK_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CLOCK_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CLOCK_REG` writer"]
pub struct W(crate::W<SPI_CLOCK_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CLOCK_REG_SPEC>;
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
impl From<crate::W<SPI_CLOCK_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CLOCK_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MASTER_CLK_MODE` reader - Should be always 1"]
pub struct SPI_MASTER_CLK_MODE_R(crate::FieldReader<bool, bool>);
impl SPI_MASTER_CLK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_MASTER_CLK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MASTER_CLK_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_MASTER_CLK_MODE` writer - Should be always 1"]
pub struct SPI_MASTER_CLK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MASTER_CLK_MODE_W<'a> {
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
#[doc = "Field `SPI_CLK_DIV` reader - Applicable only in master mode Defines the spi clock frequency in master only mode SPI_CLK = module_clk / 2*(SPI_CLK_DIV+1) when SPI_CLK_DIV not 0x7F if SPI_CLK_DIV=0x7F then SPI_CLK=module_clk"]
pub struct SPI_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SPI_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_CLK_DIV` writer - Applicable only in master mode Defines the spi clock frequency in master only mode SPI_CLK = module_clk / 2*(SPI_CLK_DIV+1) when SPI_CLK_DIV not 0x7F if SPI_CLK_DIV=0x7F then SPI_CLK=module_clk"]
pub struct SPI_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u16 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Should be always 1"]
    #[inline(always)]
    pub fn spi_master_clk_mode(&self) -> SPI_MASTER_CLK_MODE_R {
        SPI_MASTER_CLK_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 0:6 - Applicable only in master mode Defines the spi clock frequency in master only mode SPI_CLK = module_clk / 2*(SPI_CLK_DIV+1) when SPI_CLK_DIV not 0x7F if SPI_CLK_DIV=0x7F then SPI_CLK=module_clk"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Should be always 1"]
    #[inline(always)]
    pub fn spi_master_clk_mode(&mut self) -> SPI_MASTER_CLK_MODE_W {
        SPI_MASTER_CLK_MODE_W { w: self }
    }
    #[doc = "Bits 0:6 - Applicable only in master mode Defines the spi clock frequency in master only mode SPI_CLK = module_clk / 2*(SPI_CLK_DIV+1) when SPI_CLK_DIV not 0x7F if SPI_CLK_DIV=0x7F then SPI_CLK=module_clk"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W {
        SPI_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spi clock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_clock_reg](index.html) module"]
pub struct SPI_CLOCK_REG_SPEC;
impl crate::RegisterSpec for SPI_CLOCK_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_clock_reg::R](R) reader structure"]
impl crate::Readable for SPI_CLOCK_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_clock_reg::W](W) writer structure"]
impl crate::Writable for SPI_CLOCK_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CLOCK_REG to value 0"]
impl crate::Resettable for SPI_CLOCK_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
