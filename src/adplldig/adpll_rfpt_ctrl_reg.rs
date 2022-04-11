#[doc = "Register `ADPLL_RFPT_CTRL_REG` reader"]
pub struct R(crate::R<ADPLL_RFPT_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_RFPT_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_RFPT_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_RFPT_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPLL_RFPT_CTRL_REG` writer"]
pub struct W(crate::W<ADPLL_RFPT_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_RFPT_CTRL_REG_SPEC>;
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
impl From<crate::W<ADPLL_RFPT_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_RFPT_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFPT_RATE` reader - "]
pub struct RFPT_RATE_R(crate::FieldReader<bool, bool>);
impl RFPT_RATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFPT_RATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFPT_RATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFPT_RATE` writer - "]
pub struct RFPT_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFPT_RATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `INV_CKRFPT` reader - "]
pub struct INV_CKRFPT_R(crate::FieldReader<bool, bool>);
impl INV_CKRFPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INV_CKRFPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_CKRFPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV_CKRFPT` writer - "]
pub struct INV_CKRFPT_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_CKRFPT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `RFPT_MUX` reader - "]
pub struct RFPT_MUX_R(crate::FieldReader<u8, u8>);
impl RFPT_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RFPT_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFPT_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFPT_MUX` writer - "]
pub struct RFPT_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> RFPT_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rfpt_rate(&self) -> RFPT_RATE_R {
        RFPT_RATE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv_ckrfpt(&self) -> INV_CKRFPT_R {
        INV_CKRFPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rfpt_mux(&self) -> RFPT_MUX_R {
        RFPT_MUX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rfpt_rate(&mut self) -> RFPT_RATE_W {
        RFPT_RATE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv_ckrfpt(&mut self) -> INV_CKRFPT_W {
        INV_CKRFPT_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rfpt_mux(&mut self) -> RFPT_MUX_W {
        RFPT_MUX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_rfpt_ctrl_reg](index.html) module"]
pub struct ADPLL_RFPT_CTRL_REG_SPEC;
impl crate::RegisterSpec for ADPLL_RFPT_CTRL_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_rfpt_ctrl_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_RFPT_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_rfpt_ctrl_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_RFPT_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADPLL_RFPT_CTRL_REG to value 0"]
impl crate::Resettable for ADPLL_RFPT_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
