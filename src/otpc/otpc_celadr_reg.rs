#[doc = "Register `OTPC_CELADR_REG` reader"]
pub struct R(crate::R<OTPC_CELADR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_CELADR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_CELADR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_CELADR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_CELADR_REG` writer"]
pub struct W(crate::W<OTPC_CELADR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_CELADR_REG_SPEC>;
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
impl From<crate::W<OTPC_CELADR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_CELADR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_CELADR` reader - Defines a word address inside the OTP cell that will be used during the AREAD mode and the OTP mirroring."]
pub struct OTPC_CELADR_R(crate::FieldReader<u16, u16>);
impl OTPC_CELADR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        OTPC_CELADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_CELADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_CELADR` writer - Defines a word address inside the OTP cell that will be used during the AREAD mode and the OTP mirroring."]
pub struct OTPC_CELADR_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_CELADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Defines a word address inside the OTP cell that will be used during the AREAD mode and the OTP mirroring."]
    #[inline(always)]
    pub fn otpc_celadr(&self) -> OTPC_CELADR_R {
        OTPC_CELADR_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Defines a word address inside the OTP cell that will be used during the AREAD mode and the OTP mirroring."]
    #[inline(always)]
    pub fn otpc_celadr(&mut self) -> OTPC_CELADR_W {
        OTPC_CELADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP cell start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_celadr_reg](index.html) module"]
pub struct OTPC_CELADR_REG_SPEC;
impl crate::RegisterSpec for OTPC_CELADR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_celadr_reg::R](R) reader structure"]
impl crate::Readable for OTPC_CELADR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_celadr_reg::W](W) writer structure"]
impl crate::Writable for OTPC_CELADR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_CELADR_REG to value 0"]
impl crate::Resettable for OTPC_CELADR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
