#[doc = "Register `SPI_CONFIG_REG` reader"]
pub struct R(crate::R<SPI_CONFIG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CONFIG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CONFIG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CONFIG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CONFIG_REG` writer"]
pub struct W(crate::W<SPI_CONFIG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CONFIG_REG_SPEC>;
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
impl From<crate::W<SPI_CONFIG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CONFIG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_SLAVE_EN` reader - 0 = SPI module master mode 1 = SPI module slave mode"]
pub struct SPI_SLAVE_EN_R(crate::FieldReader<bool, bool>);
impl SPI_SLAVE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SLAVE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_SLAVE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_SLAVE_EN` writer - 0 = SPI module master mode 1 = SPI module slave mode"]
pub struct SPI_SLAVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLAVE_EN_W<'a> {
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
#[doc = "Field `SPI_WORD_LENGTH` reader - Define the spi word length = 1+ SPI_WORD_LENGTH (range 4 to 32)"]
pub struct SPI_WORD_LENGTH_R(crate::FieldReader<u8, u8>);
impl SPI_WORD_LENGTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_WORD_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_WORD_LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_WORD_LENGTH` writer - Define the spi word length = 1+ SPI_WORD_LENGTH (range 4 to 32)"]
pub struct SPI_WORD_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_WORD_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | ((value as u16 & 0x1f) << 2);
        self.w
    }
}
#[doc = "Field `SPI_MODE` reader - Define the spi mode (CPOL, CPHA) 0 = new data on falling, capture on rising, clk low in idle state 1 = new data on rising, capture on falling, Clk low in idle state 2 = new data on rising, capture on falling, Clk high in idle state 3 = new data on falling, capture on rising Clk high in idle state"]
pub struct SPI_MODE_R(crate::FieldReader<u8, u8>);
impl SPI_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_MODE` writer - Define the spi mode (CPOL, CPHA) 0 = new data on falling, capture on rising, clk low in idle state 1 = new data on rising, capture on falling, Clk low in idle state 2 = new data on rising, capture on falling, Clk high in idle state 3 = new data on falling, capture on rising Clk high in idle state"]
pub struct SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 0 = SPI module master mode 1 = SPI module slave mode"]
    #[inline(always)]
    pub fn spi_slave_en(&self) -> SPI_SLAVE_EN_R {
        SPI_SLAVE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Define the spi word length = 1+ SPI_WORD_LENGTH (range 4 to 32)"]
    #[inline(always)]
    pub fn spi_word_length(&self) -> SPI_WORD_LENGTH_R {
        SPI_WORD_LENGTH_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 0:1 - Define the spi mode (CPOL, CPHA) 0 = new data on falling, capture on rising, clk low in idle state 1 = new data on rising, capture on falling, Clk low in idle state 2 = new data on rising, capture on falling, Clk high in idle state 3 = new data on falling, capture on rising Clk high in idle state"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - 0 = SPI module master mode 1 = SPI module slave mode"]
    #[inline(always)]
    pub fn spi_slave_en(&mut self) -> SPI_SLAVE_EN_W {
        SPI_SLAVE_EN_W { w: self }
    }
    #[doc = "Bits 2:6 - Define the spi word length = 1+ SPI_WORD_LENGTH (range 4 to 32)"]
    #[inline(always)]
    pub fn spi_word_length(&mut self) -> SPI_WORD_LENGTH_W {
        SPI_WORD_LENGTH_W { w: self }
    }
    #[doc = "Bits 0:1 - Define the spi mode (CPOL, CPHA) 0 = new data on falling, capture on rising, clk low in idle state 1 = new data on rising, capture on falling, Clk low in idle state 2 = new data on rising, capture on falling, Clk high in idle state 3 = new data on falling, capture on rising Clk high in idle state"]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SPI_MODE_W {
        SPI_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spi control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_config_reg](index.html) module"]
pub struct SPI_CONFIG_REG_SPEC;
impl crate::RegisterSpec for SPI_CONFIG_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [spi_config_reg::R](R) reader structure"]
impl crate::Readable for SPI_CONFIG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_config_reg::W](W) writer structure"]
impl crate::Writable for SPI_CONFIG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_CONFIG_REG to value 0"]
impl crate::Resettable for SPI_CONFIG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
