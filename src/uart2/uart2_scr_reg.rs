#[doc = "Register `UART2_SCR_REG` reader"]
pub struct R(crate::R<UART2_SCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART2_SCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART2_SCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART2_SCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART2_SCR_REG` writer"]
pub struct W(crate::W<UART2_SCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART2_SCR_REG_SPEC>;
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
impl From<crate::W<UART2_SCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART2_SCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART_SCRATCH_PAD` reader - This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
pub struct UART_SCRATCH_PAD_R(crate::FieldReader<u8, u8>);
impl UART_SCRATCH_PAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_SCRATCH_PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SCRATCH_PAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART_SCRATCH_PAD` writer - This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
pub struct UART_SCRATCH_PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SCRATCH_PAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
    #[inline(always)]
    pub fn uart_scratch_pad(&self) -> UART_SCRATCH_PAD_R {
        UART_SCRATCH_PAD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
    #[inline(always)]
    pub fn uart_scratch_pad(&mut self) -> UART_SCRATCH_PAD_W {
        UART_SCRATCH_PAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scratchpad Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart2_scr_reg](index.html) module"]
pub struct UART2_SCR_REG_SPEC;
impl crate::RegisterSpec for UART2_SCR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uart2_scr_reg::R](R) reader structure"]
impl crate::Readable for UART2_SCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart2_scr_reg::W](W) writer structure"]
impl crate::Writable for UART2_SCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART2_SCR_REG to value 0"]
impl crate::Resettable for UART2_SCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
