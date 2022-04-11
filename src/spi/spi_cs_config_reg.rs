#[doc = "Register `SPI_CS_CONFIG_REG` reader"]
pub struct R(crate::R<SPI_CS_CONFIG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CS_CONFIG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CS_CONFIG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CS_CONFIG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CS_CONFIG_REG` writer"]
pub struct W(crate::W<SPI_CS_CONFIG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CS_CONFIG_REG_SPEC>;
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
impl From<crate::W<SPI_CS_CONFIG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CS_CONFIG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_CS_SELECT` reader - Control the cs output in master mode 0 = none slave device selected 1 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS0 2 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS1 4 = selected slave device connected to GPIO with FUNC_MODE=GPIO"]
pub struct SPI_CS_SELECT_R(crate::FieldReader<u8, u8>);
impl SPI_CS_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_CS_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CS_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_CS_SELECT` writer - Control the cs output in master mode 0 = none slave device selected 1 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS0 2 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS1 4 = selected slave device connected to GPIO with FUNC_MODE=GPIO"]
pub struct SPI_CS_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u16 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Control the cs output in master mode 0 = none slave device selected 1 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS0 2 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS1 4 = selected slave device connected to GPIO with FUNC_MODE=GPIO"]
    #[inline(always)]
    pub fn spi_cs_select(&self) -> SPI_CS_SELECT_R {
        SPI_CS_SELECT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Control the cs output in master mode 0 = none slave device selected 1 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS0 2 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS1 4 = selected slave device connected to GPIO with FUNC_MODE=GPIO"]
    #[inline(always)]
    pub fn spi_cs_select(&mut self) -> SPI_CS_SELECT_W {
        SPI_CS_SELECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spi cs configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cs_config_reg](index.html) module"]
pub struct SPI_CS_CONFIG_REG_SPEC;
impl crate::RegisterSpec for SPI_CS_CONFIG_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_cs_config_reg::R](R) reader structure"]
impl crate::Readable for SPI_CS_CONFIG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cs_config_reg::W](W) writer structure"]
impl crate::Writable for SPI_CS_CONFIG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CS_CONFIG_REG to value 0"]
impl crate::Resettable for SPI_CS_CONFIG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
