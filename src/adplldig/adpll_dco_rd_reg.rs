#[doc = "Register `ADPLL_DCO_RD_REG` reader"]
pub struct R(crate::R<ADPLL_DCO_RD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_DCO_RD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_DCO_RD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_DCO_RD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_DCO_RD_REG` writer"]
pub struct W(crate::W<ADPLL_DCO_RD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_DCO_RD_REG_SPEC>;
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
impl From<crate::W<ADPLL_DCO_RD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_DCO_RD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOAMP` reader - "]
pub struct DCOAMP_R(crate::FieldReader<u8, u8>);
impl DCOAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOAMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOMOD` reader - "]
pub struct DCOMOD_R(crate::FieldReader<u16, u16>);
impl DCOMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DCOMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOMOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOFINE` reader - "]
pub struct DCOFINE_R(crate::FieldReader<u8, u8>);
impl DCOFINE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOFINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOFINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOMEDIUM` reader - "]
pub struct DCOMEDIUM_R(crate::FieldReader<u8, u8>);
impl DCOMEDIUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOMEDIUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOMEDIUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOCOARSE` reader - "]
pub struct DCOCOARSE_R(crate::FieldReader<u8, u8>);
impl DCOCOARSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOCOARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOCOARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn dcoamp(&self) -> DCOAMP_R {
        DCOAMP_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 13:25"]
    #[inline(always)]
    pub fn dcomod(&self) -> DCOMOD_R {
        DCOMOD_R::new(((self.bits >> 13) & 0x1fff) as u16)
    }
    #[doc = "Bits 7:12"]
    #[inline(always)]
    pub fn dcofine(&self) -> DCOFINE_R {
        DCOFINE_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn dcomedium(&self) -> DCOMEDIUM_R {
        DCOMEDIUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dcocoarse(&self) -> DCOCOARSE_R {
        DCOCOARSE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_dco_rd_reg](index.html) module"]
pub struct ADPLL_DCO_RD_REG_SPEC;
impl crate::RegisterSpec for ADPLL_DCO_RD_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_dco_rd_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_DCO_RD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_dco_rd_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_DCO_RD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_DCO_RD_REG to value 0x1c00_0007"]
impl crate::Resettable for ADPLL_DCO_RD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1c00_0007
    }
}
