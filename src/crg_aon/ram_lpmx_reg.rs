#[doc = "Register `RAM_LPMX_REG` reader"]
pub struct R(crate::R<RAM_LPMX_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_LPMX_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_LPMX_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_LPMX_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAM_LPMX_REG` writer"]
pub struct W(crate::W<RAM_LPMX_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_LPMX_REG_SPEC>;
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
impl From<crate::W<RAM_LPMX_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_LPMX_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMx_LPMX` reader - RAM\\[3:1\\]
Transparent Light Sleep (TLS) Core Enable for System RAMs. Assert low to enable the TLS core feature, which will result in lower leakage current. In case VDD is below 0.81V, it is necessary to hold this pin high to maintain data retention."]
pub struct RAMX_LPMX_R(crate::FieldReader<u8, u8>);
impl RAMX_LPMX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RAMX_LPMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMX_LPMX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMx_LPMX` writer - RAM\\[3:1\\]
Transparent Light Sleep (TLS) Core Enable for System RAMs. Assert low to enable the TLS core feature, which will result in lower leakage current. In case VDD is below 0.81V, it is necessary to hold this pin high to maintain data retention."]
pub struct RAMX_LPMX_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMX_LPMX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u16 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RAM\\[3:1\\]
Transparent Light Sleep (TLS) Core Enable for System RAMs. Assert low to enable the TLS core feature, which will result in lower leakage current. In case VDD is below 0.81V, it is necessary to hold this pin high to maintain data retention."]
    #[inline(always)]
    pub fn ramx_lpmx(&self) -> RAMX_LPMX_R {
        RAMX_LPMX_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RAM\\[3:1\\]
Transparent Light Sleep (TLS) Core Enable for System RAMs. Assert low to enable the TLS core feature, which will result in lower leakage current. In case VDD is below 0.81V, it is necessary to hold this pin high to maintain data retention."]
    #[inline(always)]
    pub fn ramx_lpmx(&mut self) -> RAMX_LPMX_W {
        RAMX_LPMX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram_lpmx_reg](index.html) module"]
pub struct RAM_LPMX_REG_SPEC;
impl crate::RegisterSpec for RAM_LPMX_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ram_lpmx_reg::R](R) reader structure"]
impl crate::Readable for RAM_LPMX_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ram_lpmx_reg::W](W) writer structure"]
impl crate::Writable for RAM_LPMX_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAM_LPMX_REG to value 0x07"]
impl crate::Resettable for RAM_LPMX_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
