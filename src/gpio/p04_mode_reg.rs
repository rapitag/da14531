#[doc = "Register `P04_MODE_REG` reader"]
pub struct R(crate::R<P04_MODE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P04_MODE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P04_MODE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P04_MODE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P04_MODE_REG` writer"]
pub struct W(crate::W<P04_MODE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P04_MODE_REG_SPEC>;
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
impl From<crate::W<P04_MODE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P04_MODE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUPD` reader - 00 = Input, no resistors selected 01 = Input, pull-up selected 10 = Input, pull-down selected 11 = Output, no resistors selected"]
pub struct PUPD_R(crate::FieldReader<u8, u8>);
impl PUPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUPD` writer - 00 = Input, no resistors selected 01 = Input, pull-up selected 10 = Input, pull-down selected 11 = Output, no resistors selected"]
pub struct PUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> PUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u16 & 3) << 8);
        self.w
    }
}
#[doc = "Field `PID` reader - See P00_MODE_REG\\[PID\\]"]
pub struct PID_R(crate::FieldReader<u8, u8>);
impl PID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID` writer - See P00_MODE_REG\\[PID\\]"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - 00 = Input, no resistors selected 01 = Input, pull-up selected 10 = Input, pull-down selected 11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(&self) -> PUPD_R {
        PUPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:4 - See P00_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - 00 = Input, no resistors selected 01 = Input, pull-up selected 10 = Input, pull-down selected 11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(&mut self) -> PUPD_W {
        PUPD_W { w: self }
    }
    #[doc = "Bits 0:4 - See P00_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "P04 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p04_mode_reg](index.html) module"]
pub struct P04_MODE_REG_SPEC;
impl crate::RegisterSpec for P04_MODE_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p04_mode_reg::R](R) reader structure"]
impl crate::Readable for P04_MODE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p04_mode_reg::W](W) writer structure"]
impl crate::Writable for P04_MODE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P04_MODE_REG to value 0x0200"]
impl crate::Resettable for P04_MODE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
