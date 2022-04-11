#[doc = "Register `TEST_CTRL2_REG` reader"]
pub struct R(crate::R<TEST_CTRL2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTRL2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTRL2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTRL2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTRL2_REG` writer"]
pub struct W(crate::W<TEST_CTRL2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTRL2_REG_SPEC>;
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
impl From<crate::W<TEST_CTRL2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTRL2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANA_TEST_OUT_PARAM` reader - "]
pub struct ANA_TEST_OUT_PARAM_R(crate::FieldReader<u8, u8>);
impl ANA_TEST_OUT_PARAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ANA_TEST_OUT_PARAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA_TEST_OUT_PARAM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_TEST_OUT_PARAM` writer - "]
pub struct ANA_TEST_OUT_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_TEST_OUT_PARAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u16 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `ANA_TEST_OUT_TO_PIN` reader - "]
pub struct ANA_TEST_OUT_TO_PIN_R(crate::FieldReader<bool, bool>);
impl ANA_TEST_OUT_TO_PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ANA_TEST_OUT_TO_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA_TEST_OUT_TO_PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_TEST_OUT_TO_PIN` writer - "]
pub struct ANA_TEST_OUT_TO_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_TEST_OUT_TO_PIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u16 & 1) << 11);
        self.w
    }
}
#[doc = "Field `ANA_TEST_OUT_SEL` reader - "]
pub struct ANA_TEST_OUT_SEL_R(crate::FieldReader<u16, u16>);
impl ANA_TEST_OUT_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ANA_TEST_OUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANA_TEST_OUT_SEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANA_TEST_OUT_SEL` writer - "]
pub struct ANA_TEST_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ANA_TEST_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn ana_test_out_param(&self) -> ANA_TEST_OUT_PARAM_R {
        ANA_TEST_OUT_PARAM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ana_test_out_to_pin(&self) -> ANA_TEST_OUT_TO_PIN_R {
        ANA_TEST_OUT_TO_PIN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ana_test_out_sel(&self) -> ANA_TEST_OUT_SEL_R {
        ANA_TEST_OUT_SEL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn ana_test_out_param(&mut self) -> ANA_TEST_OUT_PARAM_W {
        ANA_TEST_OUT_PARAM_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ana_test_out_to_pin(&mut self) -> ANA_TEST_OUT_TO_PIN_W {
        ANA_TEST_OUT_TO_PIN_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn ana_test_out_sel(&mut self) -> ANA_TEST_OUT_SEL_W {
        ANA_TEST_OUT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctrl2_reg](index.html) module"]
pub struct TEST_CTRL2_REG_SPEC;
impl crate::RegisterSpec for TEST_CTRL2_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [test_ctrl2_reg::R](R) reader structure"]
impl crate::Readable for TEST_CTRL2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctrl2_reg::W](W) writer structure"]
impl crate::Writable for TEST_CTRL2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_CTRL2_REG to value 0"]
impl crate::Resettable for TEST_CTRL2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
