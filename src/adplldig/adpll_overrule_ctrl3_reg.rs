#[doc = "Register `ADPLL_OVERRULE_CTRL3_REG` reader"]
pub struct R(crate::R<ADPLL_OVERRULE_CTRL3_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_OVERRULE_CTRL3_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_OVERRULE_CTRL3_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_OVERRULE_CTRL3_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_OVERRULE_CTRL3_REG` writer"]
pub struct W(crate::W<ADPLL_OVERRULE_CTRL3_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_OVERRULE_CTRL3_REG_SPEC>;
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
impl From<crate::W<ADPLL_OVERRULE_CTRL3_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_OVERRULE_CTRL3_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVR_RXDIV_FB_EN_WR` reader - "]
pub struct OVR_RXDIV_FB_EN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_RXDIV_FB_EN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RXDIV_FB_EN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RXDIV_FB_EN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RXDIV_FB_EN_WR` writer - "]
pub struct OVR_RXDIV_FB_EN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RXDIV_FB_EN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `OVR_RXDIV_FB_EN_SEL` reader - "]
pub struct OVR_RXDIV_FB_EN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_RXDIV_FB_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RXDIV_FB_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RXDIV_FB_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RXDIV_FB_EN_SEL` writer - "]
pub struct OVR_RXDIV_FB_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RXDIV_FB_EN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OVR_FBDIV_EN_WR` reader - "]
pub struct OVR_FBDIV_EN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_FBDIV_EN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_FBDIV_EN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_FBDIV_EN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_FBDIV_EN_WR` writer - "]
pub struct OVR_FBDIV_EN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_FBDIV_EN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `OVR_FBDIV_EN_SEL` reader - "]
pub struct OVR_FBDIV_EN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_FBDIV_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_FBDIV_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_FBDIV_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_FBDIV_EN_SEL` writer - "]
pub struct OVR_FBDIV_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_FBDIV_EN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `OVR_TXDIV_EN_WR` reader - "]
pub struct OVR_TXDIV_EN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_TXDIV_EN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_TXDIV_EN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_TXDIV_EN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_TXDIV_EN_WR` writer - "]
pub struct OVR_TXDIV_EN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_TXDIV_EN_WR_W<'a> {
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
#[doc = "Field `OVR_TXDIV_EN_SEL` reader - "]
pub struct OVR_TXDIV_EN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_TXDIV_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_TXDIV_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_TXDIV_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_TXDIV_EN_SEL` writer - "]
pub struct OVR_TXDIV_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_TXDIV_EN_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `OVR_RXDIV_EN_WR` reader - "]
pub struct OVR_RXDIV_EN_WR_R(crate::FieldReader<bool, bool>);
impl OVR_RXDIV_EN_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RXDIV_EN_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RXDIV_EN_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RXDIV_EN_WR` writer - "]
pub struct OVR_RXDIV_EN_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RXDIV_EN_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `OVR_RXDIV_EN_SEL` reader - "]
pub struct OVR_RXDIV_EN_SEL_R(crate::FieldReader<bool, bool>);
impl OVR_RXDIV_EN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVR_RXDIV_EN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_RXDIV_EN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_RXDIV_EN_SEL` writer - "]
pub struct OVR_RXDIV_EN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_RXDIV_EN_SEL_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ovr_rxdiv_fb_en_wr(&self) -> OVR_RXDIV_FB_EN_WR_R {
        OVR_RXDIV_FB_EN_WR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ovr_rxdiv_fb_en_sel(&self) -> OVR_RXDIV_FB_EN_SEL_R {
        OVR_RXDIV_FB_EN_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ovr_fbdiv_en_wr(&self) -> OVR_FBDIV_EN_WR_R {
        OVR_FBDIV_EN_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ovr_fbdiv_en_sel(&self) -> OVR_FBDIV_EN_SEL_R {
        OVR_FBDIV_EN_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ovr_txdiv_en_wr(&self) -> OVR_TXDIV_EN_WR_R {
        OVR_TXDIV_EN_WR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ovr_txdiv_en_sel(&self) -> OVR_TXDIV_EN_SEL_R {
        OVR_TXDIV_EN_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ovr_rxdiv_en_wr(&self) -> OVR_RXDIV_EN_WR_R {
        OVR_RXDIV_EN_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ovr_rxdiv_en_sel(&self) -> OVR_RXDIV_EN_SEL_R {
        OVR_RXDIV_EN_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ovr_rxdiv_fb_en_wr(&mut self) -> OVR_RXDIV_FB_EN_WR_W {
        OVR_RXDIV_FB_EN_WR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ovr_rxdiv_fb_en_sel(&mut self) -> OVR_RXDIV_FB_EN_SEL_W {
        OVR_RXDIV_FB_EN_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ovr_fbdiv_en_wr(&mut self) -> OVR_FBDIV_EN_WR_W {
        OVR_FBDIV_EN_WR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ovr_fbdiv_en_sel(&mut self) -> OVR_FBDIV_EN_SEL_W {
        OVR_FBDIV_EN_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ovr_txdiv_en_wr(&mut self) -> OVR_TXDIV_EN_WR_W {
        OVR_TXDIV_EN_WR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ovr_txdiv_en_sel(&mut self) -> OVR_TXDIV_EN_SEL_W {
        OVR_TXDIV_EN_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ovr_rxdiv_en_wr(&mut self) -> OVR_RXDIV_EN_WR_W {
        OVR_RXDIV_EN_WR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ovr_rxdiv_en_sel(&mut self) -> OVR_RXDIV_EN_SEL_W {
        OVR_RXDIV_EN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_overrule_ctrl3_reg](index.html) module"]
pub struct ADPLL_OVERRULE_CTRL3_REG_SPEC;
impl crate::RegisterSpec for ADPLL_OVERRULE_CTRL3_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_overrule_ctrl3_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_OVERRULE_CTRL3_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_overrule_ctrl3_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_OVERRULE_CTRL3_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_OVERRULE_CTRL3_REG to value 0"]
impl crate::Resettable for ADPLL_OVERRULE_CTRL3_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
