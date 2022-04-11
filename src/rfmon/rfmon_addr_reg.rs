#[doc = "Register `RFMON_ADDR_REG` reader"]
pub struct R(crate::R<RFMON_ADDR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFMON_ADDR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFMON_ADDR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFMON_ADDR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFMON_ADDR_REG` writer"]
pub struct W(crate::W<RFMON_ADDR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFMON_ADDR_REG_SPEC>;
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
impl From<crate::W<RFMON_ADDR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFMON_ADDR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFMON_ADDR` reader - "]
pub struct RFMON_ADDR_R(crate::FieldReader<u16, u16>);
impl RFMON_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RFMON_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFMON_ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFMON_ADDR` writer - "]
pub struct RFMON_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> RFMON_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u16 & 0x3fff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn rfmon_addr(&self) -> RFMON_ADDR_R {
        RFMON_ADDR_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn rfmon_addr(&mut self) -> RFMON_ADDR_W {
        RFMON_ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfmon_addr_reg](index.html) module"]
pub struct RFMON_ADDR_REG_SPEC;
impl crate::RegisterSpec for RFMON_ADDR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rfmon_addr_reg::R](R) reader structure"]
impl crate::Readable for RFMON_ADDR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfmon_addr_reg::W](W) writer structure"]
impl crate::Writable for RFMON_ADDR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFMON_ADDR_REG to value 0"]
impl crate::Resettable for RFMON_ADDR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
