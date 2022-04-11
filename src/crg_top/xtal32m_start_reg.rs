#[doc = "Register `XTAL32M_START_REG` reader"]
pub struct R(crate::R<XTAL32M_START_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL32M_START_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL32M_START_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL32M_START_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL32M_START_REG` writer"]
pub struct W(crate::W<XTAL32M_START_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL32M_START_REG_SPEC>;
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
impl From<crate::W<XTAL32M_START_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL32M_START_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL32M_RAMP` reader - Xtal frequency trimming register. 0x00 : highest frequency 0xFF :lowest frequency"]
pub struct XTAL32M_RAMP_R(crate::FieldReader<u8, u8>);
impl XTAL32M_RAMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32M_RAMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_RAMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_RAMP` writer - Xtal frequency trimming register. 0x00 : highest frequency 0xFF :lowest frequency"]
pub struct XTAL32M_RAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32M_RAMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `XTAL32M_START` reader - Xtal frequency trimming register. 0x0 = highest frequency 0xF = lowest frequency."]
pub struct XTAL32M_START_R(crate::FieldReader<u8, u8>);
impl XTAL32M_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL32M_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_START_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_START` writer - Xtal frequency trimming register. 0x0 = highest frequency 0xF = lowest frequency."]
pub struct XTAL32M_START_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32M_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Xtal frequency trimming register. 0x00 : highest frequency 0xFF :lowest frequency"]
    #[inline(always)]
    pub fn xtal32m_ramp(&self) -> XTAL32M_RAMP_R {
        XTAL32M_RAMP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Xtal frequency trimming register. 0x0 = highest frequency 0xF = lowest frequency."]
    #[inline(always)]
    pub fn xtal32m_start(&self) -> XTAL32M_START_R {
        XTAL32M_START_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Xtal frequency trimming register. 0x00 : highest frequency 0xFF :lowest frequency"]
    #[inline(always)]
    pub fn xtal32m_ramp(&mut self) -> XTAL32M_RAMP_W {
        XTAL32M_RAMP_W { w: self }
    }
    #[doc = "Bits 0:7 - Xtal frequency trimming register. 0x0 = highest frequency 0xF = lowest frequency."]
    #[inline(always)]
    pub fn xtal32m_start(&mut self) -> XTAL32M_START_W {
        XTAL32M_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim values for XTAL32M\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal32m_start_reg](index.html) module"]
pub struct XTAL32M_START_REG_SPEC;
impl crate::RegisterSpec for XTAL32M_START_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xtal32m_start_reg::R](R) reader structure"]
impl crate::Readable for XTAL32M_START_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal32m_start_reg::W](W) writer structure"]
impl crate::Writable for XTAL32M_START_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XTAL32M_START_REG to value 0xaa"]
impl crate::Resettable for XTAL32M_START_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaa
    }
}
