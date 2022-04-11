#[doc = "Register `ADPLL_SDMOD_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_SDMOD_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_SDMOD_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_SDMOD_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_SDMOD_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_SDMOD_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_SDMOD_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_SDMOD_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_SDMOD_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_SDMOD_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMMODETX` reader - "]
pub struct SDMMODETX_R(crate::FieldReader<u8, u8>);
impl SDMMODETX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDMMODETX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMODETX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMODETX` writer - "]
pub struct SDMMODETX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMODETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `SDMMODERX` reader - "]
pub struct SDMMODERX_R(crate::FieldReader<u8, u8>);
impl SDMMODERX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDMMODERX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMODERX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMODERX` writer - "]
pub struct SDMMODERX_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMODERX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sdmmodetx(&self) -> SDMMODETX_R {
        SDMMODETX_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sdmmoderx(&self) -> SDMMODERX_R {
        SDMMODERX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn sdmmodetx(&mut self) -> SDMMODETX_W {
        SDMMODETX_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn sdmmoderx(&mut self) -> SDMMODERX_W {
        SDMMODERX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_sdmod_ctrl_reg](index.html) module"]
pub struct ADPLL_SDMOD_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_SDMOD_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_sdmod_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_SDMOD_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_sdmod_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_SDMOD_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_SDMOD_CTRL_REG to value 0x39"]
impl crate::Resettable for ADPLL_SDMOD_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x39
    }
}
