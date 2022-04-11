#[doc = "Register `I2C_COMP2_VERSION` reader"]
pub struct R(crate::R<I2C_COMP2_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_COMP2_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_COMP2_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_COMP2_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_COMP2_VERSION` writer"]
pub struct W(crate::W<I2C_COMP2_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_COMP2_VERSION_SPEC>;
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
impl From<crate::W<I2C_COMP2_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_COMP2_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_COMP2_VERSION` reader - "]
pub struct IC_COMP2_VERSION_R(crate::FieldReader<u16, u16>);
impl IC_COMP2_VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_COMP2_VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_COMP2_VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ic_comp2_version(&self) -> IC_COMP2_VERSION_R {
        IC_COMP2_VERSION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_comp2_version](index.html) module"]
pub struct I2C_COMP2_VERSION_SPEC;
impl crate::RegisterSpec for I2C_COMP2_VERSION_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_comp2_version::R](R) reader structure"]
impl crate::Readable for I2C_COMP2_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_comp2_version::W](W) writer structure"]
impl crate::Writable for I2C_COMP2_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_COMP2_VERSION to value 0x3132"]
impl crate::Resettable for I2C_COMP2_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3132
    }
}
