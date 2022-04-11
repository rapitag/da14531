#[doc = "Register `I2C_SS_SCL_HCNT_REG` reader"]
pub struct R(crate::R<I2C_SS_SCL_HCNT_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SS_SCL_HCNT_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SS_SCL_HCNT_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SS_SCL_HCNT_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_SS_SCL_HCNT_REG` writer"]
pub struct W(crate::W<I2C_SS_SCL_HCNT_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_SS_SCL_HCNT_REG_SPEC>;
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
impl From<crate::W<I2C_SS_SCL_HCNT_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_SS_SCL_HCNT_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_SS_SCL_HCNT` reader - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. NOTE: This register must not be programmed to a value higher than 65525, because the controller uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
pub struct IC_SS_SCL_HCNT_R(crate::FieldReader<u16, u16>);
impl IC_SS_SCL_HCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_SS_SCL_HCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_SS_SCL_HCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_SS_SCL_HCNT` writer - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. NOTE: This register must not be programmed to a value higher than 65525, because the controller uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
pub struct IC_SS_SCL_HCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_SS_SCL_HCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. NOTE: This register must not be programmed to a value higher than 65525, because the controller uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&self) -> IC_SS_SCL_HCNT_R {
        IC_SS_SCL_HCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. NOTE: This register must not be programmed to a value higher than 65525, because the controller uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    #[inline(always)]
    pub fn ic_ss_scl_hcnt(&mut self) -> IC_SS_SCL_HCNT_W {
        IC_SS_SCL_HCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard Speed I2C Clock SCL High Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ss_scl_hcnt_reg](index.html) module"]
pub struct I2C_SS_SCL_HCNT_REG_SPEC;
impl crate::RegisterSpec for I2C_SS_SCL_HCNT_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_ss_scl_hcnt_reg::R](R) reader structure"]
impl crate::Readable for I2C_SS_SCL_HCNT_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ss_scl_hcnt_reg::W](W) writer structure"]
impl crate::Writable for I2C_SS_SCL_HCNT_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_SS_SCL_HCNT_REG to value 0x48"]
impl crate::Resettable for I2C_SS_SCL_HCNT_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x48
    }
}
