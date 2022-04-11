#[doc = "Register `TEST_CTRL4_REG` reader"]
pub struct R(crate::R<TEST_CTRL4_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTRL4_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTRL4_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTRL4_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTRL4_REG` writer"]
pub struct W(crate::W<TEST_CTRL4_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTRL4_REG_SPEC>;
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
impl From<crate::W<TEST_CTRL4_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTRL4_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF_TEST_IN_TO_PIN` reader - "]
pub struct RF_TEST_IN_TO_PIN_R(crate::FieldReader<bool, bool>);
impl RF_TEST_IN_TO_PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_TEST_IN_TO_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TEST_IN_TO_PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_TEST_IN_TO_PIN` writer - "]
pub struct RF_TEST_IN_TO_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TEST_IN_TO_PIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u16 & 1) << 13);
        self.w
    }
}
#[doc = "Field `RF_TEST_IN_PARAM` reader - "]
pub struct RF_TEST_IN_PARAM_R(crate::FieldReader<u8, u8>);
impl RF_TEST_IN_PARAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_TEST_IN_PARAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TEST_IN_PARAM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_TEST_IN_PARAM` writer - "]
pub struct RF_TEST_IN_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TEST_IN_PARAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u16 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `RF_TEST_IN_SEL` reader - "]
pub struct RF_TEST_IN_SEL_R(crate::FieldReader<u8, u8>);
impl RF_TEST_IN_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_TEST_IN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TEST_IN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF_TEST_IN_SEL` writer - "]
pub struct RF_TEST_IN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TEST_IN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u16 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rf_test_in_to_pin(&self) -> RF_TEST_IN_TO_PIN_R {
        RF_TEST_IN_TO_PIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rf_test_in_param(&self) -> RF_TEST_IN_PARAM_R {
        RF_TEST_IN_PARAM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rf_test_in_sel(&self) -> RF_TEST_IN_SEL_R {
        RF_TEST_IN_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rf_test_in_to_pin(&mut self) -> RF_TEST_IN_TO_PIN_W {
        RF_TEST_IN_TO_PIN_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rf_test_in_param(&mut self) -> RF_TEST_IN_PARAM_W {
        RF_TEST_IN_PARAM_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rf_test_in_sel(&mut self) -> RF_TEST_IN_SEL_W {
        RF_TEST_IN_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctrl4_reg](index.html) module"]
pub struct TEST_CTRL4_REG_SPEC;
impl crate::RegisterSpec for TEST_CTRL4_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [test_ctrl4_reg::R](R) reader structure"]
impl crate::Readable for TEST_CTRL4_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctrl4_reg::W](W) writer structure"]
impl crate::Writable for TEST_CTRL4_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_CTRL4_REG to value 0"]
impl crate::Resettable for TEST_CTRL4_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
