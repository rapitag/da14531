#[doc = "Register `ADPLL_LF_CTRL1_REG` reader"]
pub struct R(crate::R<ADPLL_LF_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LF_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LF_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LF_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_LF_CTRL1_REG` writer"]
pub struct W(crate::W<ADPLL_LF_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LF_CTRL1_REG_SPEC>;
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
impl From<crate::W<ADPLL_LF_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LF_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINEKZ` reader - "]
pub struct FINEKZ_R(crate::FieldReader<u8, u8>);
impl FINEKZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FINEKZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINEKZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINEKZ` writer - "]
pub struct FINEKZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FINEKZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `FINEK` reader - "]
pub struct FINEK_R(crate::FieldReader<u8, u8>);
impl FINEK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FINEK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINEK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINEK` writer - "]
pub struct FINEK_W<'a> {
    w: &'a mut W,
}
impl<'a> FINEK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `FINETAU` reader - "]
pub struct FINETAU_R(crate::FieldReader<u8, u8>);
impl FINETAU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FINETAU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINETAU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINETAU` writer - "]
pub struct FINETAU_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETAU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn finekz(&self) -> FINEKZ_R {
        FINEKZ_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn finek(&self) -> FINEK_R {
        FINEK_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn finetau(&self) -> FINETAU_R {
        FINETAU_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15"]
    #[inline(always)]
    pub fn finekz(&mut self) -> FINEKZ_W {
        FINEKZ_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn finek(&mut self) -> FINEK_W {
        FINEK_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn finetau(&mut self) -> FINETAU_W {
        FINETAU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lf_ctrl1_reg](index.html) module"]
pub struct ADPLL_LF_CTRL1_REG_SPEC;
impl crate::RegisterSpec for ADPLL_LF_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lf_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_LF_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lf_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_LF_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_LF_CTRL1_REG to value 0x818c"]
impl crate::Resettable for ADPLL_LF_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x818c
    }
}
