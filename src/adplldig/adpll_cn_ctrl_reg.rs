#[doc = "Register `ADPLL_CN_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_CN_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_CN_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_CN_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_CN_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_CN_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_CN_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_CN_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_CN_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_CN_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - "]
pub struct CH0_R(crate::FieldReader<u16, u16>);
impl CH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0` writer - "]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `SGN` reader - "]
pub struct SGN_R(crate::FieldReader<bool, bool>);
impl SGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SGN` writer - "]
pub struct SGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `CS` reader - "]
pub struct CS_R(crate::FieldReader<bool, bool>);
impl CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` writer - "]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
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
#[doc = "Field `CN` reader - "]
pub struct CN_R(crate::FieldReader<u8, u8>);
impl CN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CN` writer - "]
pub struct CN_W<'a> {
    w: &'a mut W,
}
impl<'a> CN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sgn(&self) -> SGN_R {
        SGN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn cn(&self) -> CN_R {
        CN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sgn(&mut self) -> SGN_W {
        SGN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn cn(&mut self) -> CN_W {
        CN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_cn_ctrl_reg](index.html) module"]
pub struct ADPLL_CN_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_CN_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_cn_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_CN_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_cn_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_CN_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_CN_CTRL_REG to value 0x0962_0100"]
impl crate::Resettable for ADPLL_CN_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0962_0100
    }
}
