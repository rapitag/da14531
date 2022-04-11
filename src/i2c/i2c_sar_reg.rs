#[doc = "Register `I2C_SAR_REG` reader"]
pub struct R(crate::R<I2C_SAR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SAR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SAR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SAR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_SAR_REG` writer"]
pub struct W(crate::W<I2C_SAR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_SAR_REG_SPEC>;
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
impl From<crate::W<I2C_SAR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_SAR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_SAR` reader - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub struct IC_SAR_R(crate::FieldReader<u16, u16>);
impl IC_SAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_SAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_SAR` writer - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
pub struct IC_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_sar(&self) -> IC_SAR_R {
        IC_SAR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR\\[6:0\\]
is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect."]
    #[inline(always)]
    pub fn ic_sar(&mut self) -> IC_SAR_W {
        IC_SAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sar_reg](index.html) module"]
pub struct I2C_SAR_REG_SPEC;
impl crate::RegisterSpec for I2C_SAR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_sar_reg::R](R) reader structure"]
impl crate::Readable for I2C_SAR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_sar_reg::W](W) writer structure"]
impl crate::Writable for I2C_SAR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_SAR_REG to value 0x55"]
impl crate::Resettable for I2C_SAR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
