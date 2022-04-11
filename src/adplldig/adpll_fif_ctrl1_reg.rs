#[doc = "Register `ADPLL_FIF_CTRL1_REG` reader"]
pub struct R(crate::R<ADPLL_FIF_CTRL1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_FIF_CTRL1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_FIF_CTRL1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_FIF_CTRL1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_FIF_CTRL1_REG` writer"]
pub struct W(crate::W<ADPLL_FIF_CTRL1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_FIF_CTRL1_REG_SPEC>;
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
impl From<crate::W<ADPLL_FIF_CTRL1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_FIF_CTRL1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFRX_1M` reader - "]
pub struct FIFRX_1M_R(crate::FieldReader<u16, u16>);
impl FIFRX_1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FIFRX_1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFRX_1M_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFRX_1M` writer - "]
pub struct FIFRX_1M_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFRX_1M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn fifrx_1m(&self) -> FIFRX_1M_R {
        FIFRX_1M_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn fifrx_1m(&mut self) -> FIFRX_1M_W {
        FIFRX_1M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_fif_ctrl1_reg](index.html) module"]
pub struct ADPLL_FIF_CTRL1_REG_SPEC;
impl crate::RegisterSpec for ADPLL_FIF_CTRL1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_fif_ctrl1_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_FIF_CTRL1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_fif_ctrl1_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_FIF_CTRL1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_FIF_CTRL1_REG to value 0x0400"]
impl crate::Resettable for ADPLL_FIF_CTRL1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
