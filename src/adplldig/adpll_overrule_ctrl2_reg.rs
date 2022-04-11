#[doc = "Register `ADPLL_OVERRULE_CTRL2_REG` reader"]
pub struct R(crate::R<ADPLL_OVERRULE_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_OVERRULE_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_OVERRULE_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_OVERRULE_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_OVERRULE_CTRL2_REG` writer"]
pub struct W(crate::W<ADPLL_OVERRULE_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_OVERRULE_CTRL2_REG_SPEC>;
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
impl From<crate::W<ADPLL_OVERRULE_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_OVERRULE_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR_DCOMOD_WR` reader - "]
pub struct OVR_DCOMOD_WR_R(crate::FieldReader<u8, u8>);
impl OVR_DCOMOD_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_DCOMOD_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOMOD_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOMOD_WR` writer - "]
pub struct OVR_DCOMOD_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOMOD_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `OVR_DCOMOD_SEL` reader - "]
pub struct OVR_DCOMOD_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DCOMOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOMOD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOMOD_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOMOD_SEL` writer - "]
pub struct OVR_DCOMOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOMOD_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `OVR_DCOFINE_WR` reader - "]
pub struct OVR_DCOFINE_WR_R(crate::FieldReader<u8, u8>);
impl OVR_DCOFINE_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_DCOFINE_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOFINE_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOFINE_WR` writer - "]
pub struct OVR_DCOFINE_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOFINE_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 17)) | ((value as u32 & 0x3f) << 17);
        self.w
    }
}
#[doc = "Field `OVR_DCOFINE_SEL` reader - "]
pub struct OVR_DCOFINE_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DCOFINE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOFINE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOFINE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOFINE_SEL` writer - "]
pub struct OVR_DCOFINE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOFINE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `OVR_DCOMEDIUM_WR` reader - "]
pub struct OVR_DCOMEDIUM_WR_R(crate::FieldReader<u8, u8>);
impl OVR_DCOMEDIUM_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_DCOMEDIUM_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOMEDIUM_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOMEDIUM_WR` writer - "]
pub struct OVR_DCOMEDIUM_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOMEDIUM_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 9)) | ((value as u32 & 7) << 9);
        self.w
    }
}
#[doc = "Field `OVR_DCOMEDIUM_SEL` reader - "]
pub struct OVR_DCOMEDIUM_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DCOMEDIUM_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOMEDIUM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOMEDIUM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOMEDIUM_SEL` writer - "]
pub struct OVR_DCOMEDIUM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOMEDIUM_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `OVR_DCOCOARSE_WR` reader - "]
pub struct OVR_DCOCOARSE_WR_R(crate::FieldReader<u8, u8>);
impl OVR_DCOCOARSE_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OVR_DCOCOARSE_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOCOARSE_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOCOARSE_WR` writer - "]
pub struct OVR_DCOCOARSE_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOCOARSE_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `OVR_DCOCOARSE_SEL` reader - "]
pub struct OVR_DCOCOARSE_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_DCOCOARSE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_DCOCOARSE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_DCOCOARSE_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_DCOCOARSE_SEL` writer - "]
pub struct OVR_DCOCOARSE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_DCOCOARSE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ovr_dcomod_wr(&self) -> OVR_DCOMOD_WR_R {
        OVR_DCOMOD_WR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ovr_dcomod_sel(&self) -> OVR_DCOMOD_SEL_R {
        OVR_DCOMOD_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 17:22"]
    #[inline(always)]
    pub fn ovr_dcofine_wr(&self) -> OVR_DCOFINE_WR_R {
        OVR_DCOFINE_WR_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ovr_dcofine_sel(&self) -> OVR_DCOFINE_SEL_R {
        OVR_DCOFINE_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn ovr_dcomedium_wr(&self) -> OVR_DCOMEDIUM_WR_R {
        OVR_DCOMEDIUM_WR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ovr_dcomedium_sel(&self) -> OVR_DCOMEDIUM_SEL_R {
        OVR_DCOMEDIUM_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn ovr_dcocoarse_wr(&self) -> OVR_DCOCOARSE_WR_R {
        OVR_DCOCOARSE_WR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ovr_dcocoarse_sel(&self) -> OVR_DCOCOARSE_SEL_R {
        OVR_DCOCOARSE_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ovr_dcomod_wr(&mut self) -> OVR_DCOMOD_WR_W {
        OVR_DCOMOD_WR_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ovr_dcomod_sel(&mut self) -> OVR_DCOMOD_SEL_W {
        OVR_DCOMOD_SEL_W { w: self }
    }
    #[doc = "Bits 17:22"]
    #[inline(always)]
    pub fn ovr_dcofine_wr(&mut self) -> OVR_DCOFINE_WR_W {
        OVR_DCOFINE_WR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ovr_dcofine_sel(&mut self) -> OVR_DCOFINE_SEL_W {
        OVR_DCOFINE_SEL_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn ovr_dcomedium_wr(&mut self) -> OVR_DCOMEDIUM_WR_W {
        OVR_DCOMEDIUM_WR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ovr_dcomedium_sel(&mut self) -> OVR_DCOMEDIUM_SEL_W {
        OVR_DCOMEDIUM_SEL_W { w: self }
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn ovr_dcocoarse_wr(&mut self) -> OVR_DCOCOARSE_WR_W {
        OVR_DCOCOARSE_WR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ovr_dcocoarse_sel(&mut self) -> OVR_DCOCOARSE_SEL_W {
        OVR_DCOCOARSE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_overrule_ctrl2_reg](index.html) module"]
pub struct ADPLL_OVERRULE_CTRL2_REG_SPEC;
impl crate::RegisterSpec for ADPLL_OVERRULE_CTRL2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_overrule_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_OVERRULE_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_overrule_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_OVERRULE_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_OVERRULE_CTRL2_REG to value 0"]
impl crate::Resettable for ADPLL_OVERRULE_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
