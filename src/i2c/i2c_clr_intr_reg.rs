#[doc = "Register `I2C_CLR_INTR_REG` reader"]
pub struct R(crate::R<I2C_CLR_INTR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CLR_INTR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CLR_INTR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CLR_INTR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_CLR_INTR_REG` writer"]
pub struct W(crate::W<I2C_CLR_INTR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CLR_INTR_REG_SPEC>;
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
impl From<crate::W<I2C_CLR_INTR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CLR_INTR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_INTR` reader - Read this register to clear the combined interrupt, all individual interrupts, and the I2C_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the I2C_TX_ABRT_SOURCE register for an exception to clearing I2C_TX_ABRT_SOURCE"]
pub struct CLR_INTR_R(crate::FieldReader<bool, bool>);
impl CLR_INTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read this register to clear the combined interrupt, all individual interrupts, and the I2C_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the I2C_TX_ABRT_SOURCE register for an exception to clearing I2C_TX_ABRT_SOURCE"]
    #[inline(always)]
    pub fn clr_intr(&self) -> CLR_INTR_R {
        CLR_INTR_R::new((self.bits & 1) != 0)
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
#[doc = "Clear Combined and Individual Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clr_intr_reg](index.html) module"]
pub struct I2C_CLR_INTR_REG_SPEC;
impl crate::RegisterSpec for I2C_CLR_INTR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_clr_intr_reg::R](R) reader structure"]
impl crate::Readable for I2C_CLR_INTR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_clr_intr_reg::W](W) writer structure"]
impl crate::Writable for I2C_CLR_INTR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_CLR_INTR_REG to value 0"]
impl crate::Resettable for I2C_CLR_INTR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}