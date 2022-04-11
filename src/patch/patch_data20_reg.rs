#[doc = "Register `PATCH_DATA20_REG` reader"]
pub struct R(crate::R<PATCH_DATA20_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATCH_DATA20_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATCH_DATA20_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATCH_DATA20_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATCH_DATA20_REG` writer"]
pub struct W(crate::W<PATCH_DATA20_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATCH_DATA20_REG_SPEC>;
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
impl From<crate::W<PATCH_DATA20_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATCH_DATA20_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PATCH_DATA` reader - "]
pub struct PATCH_DATA_R(crate::FieldReader<u32, u32>);
impl PATCH_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PATCH_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH_DATA` writer - "]
pub struct PATCH_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn patch_data(&self) -> PATCH_DATA_R {
        PATCH_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn patch_data(&mut self) -> PATCH_DATA_W {
        PATCH_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patch_data20_reg](index.html) module"]
pub struct PATCH_DATA20_REG_SPEC;
impl crate::RegisterSpec for PATCH_DATA20_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patch_data20_reg::R](R) reader structure"]
impl crate::Readable for PATCH_DATA20_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patch_data20_reg::W](W) writer structure"]
impl crate::Writable for PATCH_DATA20_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATCH_DATA20_REG to value 0"]
impl crate::Resettable for PATCH_DATA20_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
