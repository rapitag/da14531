#[doc = "Register `ADPLL_LF_CTRL2_REG` reader"]
pub struct R(crate::R<ADPLL_LF_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LF_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LF_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LF_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_LF_CTRL2_REG` writer"]
pub struct W(crate::W<ADPLL_LF_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LF_CTRL2_REG_SPEC>;
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
impl From<crate::W<ADPLL_LF_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LF_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST_TAU_EN` reader - "]
pub struct RST_TAU_EN_R(crate::FieldReader<bool, bool>);
impl RST_TAU_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_TAU_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_TAU_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST_TAU_EN` writer - "]
pub struct RST_TAU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_TAU_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `MODKZ` reader - "]
pub struct MODKZ_R(crate::FieldReader<u8, u8>);
impl MODKZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODKZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODKZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODKZ` writer - "]
pub struct MODKZ_W<'a> {
    w: &'a mut W,
}
impl<'a> MODKZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `MODK` reader - "]
pub struct MODK_R(crate::FieldReader<u8, u8>);
impl MODK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODK` writer - "]
pub struct MODK_W<'a> {
    w: &'a mut W,
}
impl<'a> MODK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `MODTAU` reader - "]
pub struct MODTAU_R(crate::FieldReader<u8, u8>);
impl MODTAU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODTAU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODTAU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODTAU` writer - "]
pub struct MODTAU_W<'a> {
    w: &'a mut W,
}
impl<'a> MODTAU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `MODK_TUNE` reader - "]
pub struct MODK_TUNE_R(crate::FieldReader<u8, u8>);
impl MODK_TUNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODK_TUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODK_TUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODK_TUNE` writer - "]
pub struct MODK_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODK_TUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `MODTAU_TUNE` reader - "]
pub struct MODTAU_TUNE_R(crate::FieldReader<u8, u8>);
impl MODTAU_TUNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODTAU_TUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODTAU_TUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODTAU_TUNE` writer - "]
pub struct MODTAU_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODTAU_TUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rst_tau_en(&self) -> RST_TAU_EN_R {
        RST_TAU_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn modkz(&self) -> MODKZ_R {
        MODKZ_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn modk(&self) -> MODK_R {
        MODK_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn modtau(&self) -> MODTAU_R {
        MODTAU_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn modk_tune(&self) -> MODK_TUNE_R {
        MODK_TUNE_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn modtau_tune(&self) -> MODTAU_TUNE_R {
        MODTAU_TUNE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rst_tau_en(&mut self) -> RST_TAU_EN_W {
        RST_TAU_EN_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn modkz(&mut self) -> MODKZ_W {
        MODKZ_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn modk(&mut self) -> MODK_W {
        MODK_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn modtau(&mut self) -> MODTAU_W {
        MODTAU_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn modk_tune(&mut self) -> MODK_TUNE_W {
        MODK_TUNE_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn modtau_tune(&mut self) -> MODTAU_TUNE_W {
        MODTAU_TUNE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lf_ctrl2_reg](index.html) module"]
pub struct ADPLL_LF_CTRL2_REG_SPEC;
impl crate::RegisterSpec for ADPLL_LF_CTRL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lf_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_LF_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lf_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_LF_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_LF_CTRL2_REG to value 0x1ca2_879e"]
impl crate::Resettable for ADPLL_LF_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1ca2_879e
    }
}
