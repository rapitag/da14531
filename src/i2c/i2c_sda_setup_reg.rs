#[doc = "Register `I2C_SDA_SETUP_REG` reader"]
pub struct R(crate::R<I2C_SDA_SETUP_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SDA_SETUP_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SDA_SETUP_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SDA_SETUP_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_SDA_SETUP_REG` writer"]
pub struct W(crate::W<I2C_SDA_SETUP_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_SDA_SETUP_REG_SPEC>;
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
impl From<crate::W<I2C_SDA_SETUP_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_SDA_SETUP_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_SETUP` reader - SDA Setup. This register controls the amount of time delay (number of I2C clock periods) between the rising edge of SCL and SDA changing by holding SCL low when I2C block services a read request while operating as a slave-transmitter. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. It is recommended that if the required delay is 1000ns, then for an I2C frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11.Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0."]
pub struct SDA_SETUP_R(crate::FieldReader<u8, u8>);
impl SDA_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDA_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDA_SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDA_SETUP` writer - SDA Setup. This register controls the amount of time delay (number of I2C clock periods) between the rising edge of SCL and SDA changing by holding SCL low when I2C block services a read request while operating as a slave-transmitter. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. It is recommended that if the required delay is 1000ns, then for an I2C frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11.Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0."]
pub struct SDA_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SDA Setup. This register controls the amount of time delay (number of I2C clock periods) between the rising edge of SCL and SDA changing by holding SCL low when I2C block services a read request while operating as a slave-transmitter. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. It is recommended that if the required delay is 1000ns, then for an I2C frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11.Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0."]
    #[inline(always)]
    pub fn sda_setup(&self) -> SDA_SETUP_R {
        SDA_SETUP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA Setup. This register controls the amount of time delay (number of I2C clock periods) between the rising edge of SCL and SDA changing by holding SCL low when I2C block services a read request while operating as a slave-transmitter. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. It is recommended that if the required delay is 1000ns, then for an I2C frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11.Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0."]
    #[inline(always)]
    pub fn sda_setup(&mut self) -> SDA_SETUP_W {
        SDA_SETUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SDA Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sda_setup_reg](index.html) module"]
pub struct I2C_SDA_SETUP_REG_SPEC;
impl crate::RegisterSpec for I2C_SDA_SETUP_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_sda_setup_reg::R](R) reader structure"]
impl crate::Readable for I2C_SDA_SETUP_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_sda_setup_reg::W](W) writer structure"]
impl crate::Writable for I2C_SDA_SETUP_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_SDA_SETUP_REG to value 0x64"]
impl crate::Resettable for I2C_SDA_SETUP_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x64
    }
}
