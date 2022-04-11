#[doc = "Register `TEST_VDD_REG` reader"]
pub struct R(crate::R<TEST_VDD_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_VDD_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_VDD_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_VDD_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_VDD_REG` writer"]
pub struct W(crate::W<TEST_VDD_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_VDD_REG_SPEC>;
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
impl From<crate::W<TEST_VDD_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_VDD_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOS_DISABLE` reader - "]
pub struct LDOS_DISABLE_R(crate::FieldReader<bool, bool>);
impl LDOS_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LDOS_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDOS_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LDOS_DISABLE` writer - "]
pub struct LDOS_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOS_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u16 & 1) << 1);
        self.w
    }
}
#[doc = "Field `TEST_VDD` reader - "]
pub struct TEST_VDD_R(crate::FieldReader<bool, bool>);
impl TEST_VDD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEST_VDD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_VDD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_VDD` writer - "]
pub struct TEST_VDD_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_VDD_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u16 & 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldos_disable(&self) -> LDOS_DISABLE_R {
        LDOS_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_vdd(&self) -> TEST_VDD_R {
        TEST_VDD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldos_disable(&mut self) -> LDOS_DISABLE_W {
        LDOS_DISABLE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn test_vdd(&mut self) -> TEST_VDD_W {
        TEST_VDD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_vdd_reg](index.html) module"]
pub struct TEST_VDD_REG_SPEC;
impl crate::RegisterSpec for TEST_VDD_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [test_vdd_reg::R](R) reader structure"]
impl crate::Readable for TEST_VDD_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_vdd_reg::W](W) writer structure"]
impl crate::Writable for TEST_VDD_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_VDD_REG to value 0"]
impl crate::Resettable for TEST_VDD_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
