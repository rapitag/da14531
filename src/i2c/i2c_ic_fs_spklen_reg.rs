#[doc = "Register `I2C_IC_FS_SPKLEN_REG` reader"]
pub struct R(crate::R<I2C_IC_FS_SPKLEN_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_IC_FS_SPKLEN_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_IC_FS_SPKLEN_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_IC_FS_SPKLEN_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_IC_FS_SPKLEN_REG` writer"]
pub struct W(crate::W<I2C_IC_FS_SPKLEN_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_IC_FS_SPKLEN_REG_SPEC>;
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
impl From<crate::W<I2C_IC_FS_SPKLEN_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_IC_FS_SPKLEN_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_FS_SPKLEN` reader - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set."]
pub struct IC_FS_SPKLEN_R(crate::FieldReader<u8, u8>);
impl IC_FS_SPKLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IC_FS_SPKLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_FS_SPKLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_FS_SPKLEN` writer - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set."]
pub struct IC_FS_SPKLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_FS_SPKLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set."]
    #[inline(always)]
    pub fn ic_fs_spklen(&self) -> IC_FS_SPKLEN_R {
        IC_FS_SPKLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set."]
    #[inline(always)]
    pub fn ic_fs_spklen(&mut self) -> IC_FS_SPKLEN_W {
        IC_FS_SPKLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SS and FS spike suppression limit Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ic_fs_spklen_reg](index.html) module"]
pub struct I2C_IC_FS_SPKLEN_REG_SPEC;
impl crate::RegisterSpec for I2C_IC_FS_SPKLEN_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_ic_fs_spklen_reg::R](R) reader structure"]
impl crate::Readable for I2C_IC_FS_SPKLEN_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ic_fs_spklen_reg::W](W) writer structure"]
impl crate::Writable for I2C_IC_FS_SPKLEN_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_IC_FS_SPKLEN_REG to value 0x01"]
impl crate::Resettable for I2C_IC_FS_SPKLEN_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
