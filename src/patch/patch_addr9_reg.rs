#[doc = "Register `PATCH_ADDR9_REG` reader"]
pub struct R(crate::R<PATCH_ADDR9_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PATCH_ADDR9_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PATCH_ADDR9_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PATCH_ADDR9_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PATCH_ADDR9_REG` writer"]
pub struct W(crate::W<PATCH_ADDR9_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PATCH_ADDR9_REG_SPEC>;
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
impl From<crate::W<PATCH_ADDR9_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PATCH_ADDR9_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PATCH_ADDR_19` reader - "]
pub struct PATCH_ADDR_19_R(crate::FieldReader<bool, bool>);
impl PATCH_ADDR_19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PATCH_ADDR_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH_ADDR_19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH_ADDR_19` writer - "]
pub struct PATCH_ADDR_19_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_ADDR_19_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `PATCH_ADDR_C` reader - "]
pub struct PATCH_ADDR_C_R(crate::FieldReader<u32, u32>);
impl PATCH_ADDR_C_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        PATCH_ADDR_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PATCH_ADDR_C_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PATCH_ADDR_C` writer - "]
pub struct PATCH_ADDR_C_W<'a> {
    w: &'a mut W,
}
impl<'a> PATCH_ADDR_C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 1)) | ((value as u32 & 0x0001_ffff) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn patch_addr_19(&self) -> PATCH_ADDR_19_R {
        PATCH_ADDR_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 1:17"]
    #[inline(always)]
    pub fn patch_addr_c(&self) -> PATCH_ADDR_C_R {
        PATCH_ADDR_C_R::new(((self.bits >> 1) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn patch_addr_19(&mut self) -> PATCH_ADDR_19_W {
        PATCH_ADDR_19_W { w: self }
    }
    #[doc = "Bits 1:17"]
    #[inline(always)]
    pub fn patch_addr_c(&mut self) -> PATCH_ADDR_C_W {
        PATCH_ADDR_C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [patch_addr9_reg](index.html) module"]
pub struct PATCH_ADDR9_REG_SPEC;
impl crate::RegisterSpec for PATCH_ADDR9_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [patch_addr9_reg::R](R) reader structure"]
impl crate::Readable for PATCH_ADDR9_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [patch_addr9_reg::W](W) writer structure"]
impl crate::Writable for PATCH_ADDR9_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PATCH_ADDR9_REG to value 0x07f0_0000"]
impl crate::Resettable for PATCH_ADDR9_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07f0_0000
    }
}
