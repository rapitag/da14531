#[doc = "Register `RF_ATTR_REG` reader"]
pub struct R(crate::R<RF_ATTR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ATTR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ATTR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ATTR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_ATTR_REG` writer"]
pub struct W(crate::W<RF_ATTR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ATTR_REG_SPEC>;
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
impl From<crate::W<RF_ATTR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ATTR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_POWER_SETTING` reader - "]
pub struct PA_POWER_SETTING_R(crate::FieldReader<u8, u8>);
impl PA_POWER_SETTING_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_POWER_SETTING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_POWER_SETTING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PA_POWER_SETTING` writer - "]
pub struct PA_POWER_SETTING_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_POWER_SETTING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `TIA_BIAS` reader - "]
pub struct TIA_BIAS_R(crate::FieldReader<bool, bool>);
impl TIA_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIA_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIA_BIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIA_BIAS` writer - "]
pub struct TIA_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIA_BIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `RF_BIAS` reader - "]
pub struct RF_BIAS_R(crate::FieldReader<u8, u8>);
impl RF_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_BIAS` writer - "]
pub struct RF_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `IFF_POLARITY` reader - "]
pub struct IFF_POLARITY_R(crate::FieldReader<bool, bool>);
impl IFF_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IFF_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFF_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFF_POLARITY` writer - "]
pub struct IFF_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> IFF_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn pa_power_setting(&self) -> PA_POWER_SETTING_R {
        PA_POWER_SETTING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tia_bias(&self) -> TIA_BIAS_R {
        TIA_BIAS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rf_bias(&self) -> RF_BIAS_R {
        RF_BIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iff_polarity(&self) -> IFF_POLARITY_R {
        IFF_POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn pa_power_setting(&mut self) -> PA_POWER_SETTING_W {
        PA_POWER_SETTING_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tia_bias(&mut self) -> TIA_BIAS_W {
        TIA_BIAS_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn rf_bias(&mut self) -> RF_BIAS_W {
        RF_BIAS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn iff_polarity(&mut self) -> IFF_POLARITY_W {
        IFF_POLARITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_attr_reg](index.html) module"]
pub struct RF_ATTR_REG_SPEC;
impl crate::RegisterSpec for RF_ATTR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_attr_reg::R](R) reader structure"]
impl crate::Readable for RF_ATTR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_attr_reg::W](W) writer structure"]
impl crate::Writable for RF_ATTR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RF_ATTR_REG to value 0x0c00_0820"]
impl crate::Resettable for RF_ATTR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0820
    }
}
