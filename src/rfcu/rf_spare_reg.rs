#[doc = "Register `RF_SPARE_REG` reader"]
pub struct R(crate::R<RF_SPARE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SPARE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SPARE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SPARE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_SPARE_REG` writer"]
pub struct W(crate::W<RF_SPARE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SPARE_REG_SPEC>;
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
impl From<crate::W<RF_SPARE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SPARE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF_SPARE_IN_EN` reader - "]
pub struct RF_SPARE_IN_EN_R(crate::FieldReader<bool, bool>);
impl RF_SPARE_IN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_SPARE_IN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SPARE_IN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_SPARE_IN_EN` writer - "]
pub struct RF_SPARE_IN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SPARE_IN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `RF_SPARE_IN` reader - "]
pub struct RF_SPARE_IN_R(crate::FieldReader<u8, u8>);
impl RF_SPARE_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_SPARE_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SPARE_IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_SPARE_BITS_HV` reader - "]
pub struct RF_SPARE_BITS_HV_R(crate::FieldReader<u8, u8>);
impl RF_SPARE_BITS_HV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_SPARE_BITS_HV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SPARE_BITS_HV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_SPARE_BITS_HV` writer - "]
pub struct RF_SPARE_BITS_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SPARE_BITS_HV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RF_SPARE_BITS` reader - "]
pub struct RF_SPARE_BITS_R(crate::FieldReader<u16, u16>);
impl RF_SPARE_BITS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RF_SPARE_BITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_SPARE_BITS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_SPARE_BITS` writer - "]
pub struct RF_SPARE_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_SPARE_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rf_spare_in_en(&self) -> RF_SPARE_IN_EN_R {
        RF_SPARE_IN_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rf_spare_in(&self) -> RF_SPARE_IN_R {
        RF_SPARE_IN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rf_spare_bits_hv(&self) -> RF_SPARE_BITS_HV_R {
        RF_SPARE_BITS_HV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_spare_bits(&self) -> RF_SPARE_BITS_R {
        RF_SPARE_BITS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rf_spare_in_en(&mut self) -> RF_SPARE_IN_EN_W {
        RF_SPARE_IN_EN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rf_spare_bits_hv(&mut self) -> RF_SPARE_BITS_HV_W {
        RF_SPARE_BITS_HV_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_spare_bits(&mut self) -> RF_SPARE_BITS_W {
        RF_SPARE_BITS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_spare_reg](index.html) module"]
pub struct RF_SPARE_REG_SPEC;
impl crate::RegisterSpec for RF_SPARE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_spare_reg::R](R) reader structure"]
impl crate::Readable for RF_SPARE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_spare_reg::W](W) writer structure"]
impl crate::Writable for RF_SPARE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_SPARE_REG to value 0"]
impl crate::Resettable for RF_SPARE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
