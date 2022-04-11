#[doc = "Register `UART2_DLF_REG` reader"]
pub struct R(crate::R<UART2_DLF_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_DLF_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_DLF_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_DLF_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_DLF_REG` writer"]
pub struct W(crate::W<UART2_DLF_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_DLF_REG_SPEC>;
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
impl From<crate::W<UART2_DLF_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_DLF_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_DLF` reader - The fractional value is added to integer value set by DLH, DLL. Fractional value is equal UART_DLF/16"]
pub struct UART_DLF_R(crate::FieldReader<u8, u8>);
impl UART_DLF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_DLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DLF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_DLF` writer - The fractional value is added to integer value set by DLH, DLL. Fractional value is equal UART_DLF/16"]
pub struct UART_DLF_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DLF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - The fractional value is added to integer value set by DLH, DLL. Fractional value is equal UART_DLF/16"]
    #[inline(always)]
    pub fn uart_dlf(&self) -> UART_DLF_R {
        UART_DLF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The fractional value is added to integer value set by DLH, DLL. Fractional value is equal UART_DLF/16"]
    #[inline(always)]
    pub fn uart_dlf(&mut self) -> UART_DLF_W {
        UART_DLF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divisor Latch Fraction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_dlf_reg](index.html) module"]
pub struct UART2_DLF_REG_SPEC;
impl crate::RegisterSpec for UART2_DLF_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_dlf_reg::R](R) reader structure"]
impl crate::Readable for UART2_DLF_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_dlf_reg::W](W) writer structure"]
impl crate::Writable for UART2_DLF_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_DLF_REG to value 0"]
impl crate::Resettable for UART2_DLF_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
