#[doc = "Register `XTALRDY_CTRL_REG` reader"]
pub struct R(crate::R<XTALRDY_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTALRDY_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTALRDY_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTALRDY_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTALRDY_CTRL_REG` writer"]
pub struct W(crate::W<XTALRDY_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTALRDY_CTRL_REG_SPEC>;
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
impl From<crate::W<XTALRDY_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTALRDY_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTALRDY_CNT` reader - Number of 32kHz cycles between the crystal is enabled, and the XTALRDY_IRQ is fired. 0x00: no interrupt"]
pub struct XTALRDY_CNT_R(crate::FieldReader<u8, u8>);
impl XTALRDY_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTALRDY_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALRDY_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALRDY_CNT` writer - Number of 32kHz cycles between the crystal is enabled, and the XTALRDY_IRQ is fired. 0x00: no interrupt"]
pub struct XTALRDY_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALRDY_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of 32kHz cycles between the crystal is enabled, and the XTALRDY_IRQ is fired. 0x00: no interrupt"]
    #[inline(always)]
    pub fn xtalrdy_cnt(&self) -> XTALRDY_CNT_R {
        XTALRDY_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of 32kHz cycles between the crystal is enabled, and the XTALRDY_IRQ is fired. 0x00: no interrupt"]
    #[inline(always)]
    pub fn xtalrdy_cnt(&mut self) -> XTALRDY_CNT_W {
        XTALRDY_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register for XTALRDY IRQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalrdy_ctrl_reg](index.html) module"]
pub struct XTALRDY_CTRL_REG_SPEC;
impl crate::RegisterSpec for XTALRDY_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtalrdy_ctrl_reg::R](R) reader structure"]
impl crate::Readable for XTALRDY_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtalrdy_ctrl_reg::W](W) writer structure"]
impl crate::Writable for XTALRDY_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTALRDY_CTRL_REG to value 0"]
impl crate::Resettable for XTALRDY_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
