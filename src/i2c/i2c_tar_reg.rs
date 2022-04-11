#[doc = "Register `I2C_TAR_REG` reader"]
pub struct R(crate::R<I2C_TAR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TAR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TAR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TAR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TAR_REG` writer"]
pub struct W(crate::W<I2C_TAR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TAR_REG_SPEC>;
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
impl From<crate::W<I2C_TAR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TAR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPECIAL` reader - This bit indicates whether software performs a General Call or START BYTE command. 0: ignore bit 10 GC_OR_START and use IC_TAR normally 1: perform special I2C command as specified in GC_OR_START bit"]
pub struct SPECIAL_R(crate::FieldReader<bool, bool>);
impl SPECIAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPECIAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPECIAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPECIAL` writer - This bit indicates whether software performs a General Call or START BYTE command. 0: ignore bit 10 GC_OR_START and use IC_TAR normally 1: perform special I2C command as specified in GC_OR_START bit"]
pub struct SPECIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPECIAL_W<'a> {
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
#[doc = "Field `GC_OR_START` reader - If bit 11 (SPECIAL) is set to 1, then this bit indicates whether a General Call or START byte command is to be performed by the controller. 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The controller remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. 1: START BYTE"]
pub struct GC_OR_START_R(crate::FieldReader<bool, bool>);
impl GC_OR_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GC_OR_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_OR_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GC_OR_START` writer - If bit 11 (SPECIAL) is set to 1, then this bit indicates whether a General Call or START byte command is to be performed by the controller. 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The controller remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. 1: START BYTE"]
pub struct GC_OR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_OR_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u16 & 1) << 10);
        self.w
    }
}
#[doc = "Field `IC_TAR` reader - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits. Note: If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave"]
pub struct IC_TAR_R(crate::FieldReader<u16, u16>);
impl IC_TAR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        IC_TAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_TAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC_TAR` writer - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits. Note: If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave"]
pub struct IC_TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE command. 0: ignore bit 10 GC_OR_START and use IC_TAR normally 1: perform special I2C command as specified in GC_OR_START bit"]
    #[inline(always)]
    pub fn special(&self) -> SPECIAL_R {
        SPECIAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - If bit 11 (SPECIAL) is set to 1, then this bit indicates whether a General Call or START byte command is to be performed by the controller. 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The controller remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. 1: START BYTE"]
    #[inline(always)]
    pub fn gc_or_start(&self) -> GC_OR_START_R {
        GC_OR_START_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits. Note: If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave"]
    #[inline(always)]
    pub fn ic_tar(&self) -> IC_TAR_R {
        IC_TAR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 11 - This bit indicates whether software performs a General Call or START BYTE command. 0: ignore bit 10 GC_OR_START and use IC_TAR normally 1: perform special I2C command as specified in GC_OR_START bit"]
    #[inline(always)]
    pub fn special(&mut self) -> SPECIAL_W {
        SPECIAL_W { w: self }
    }
    #[doc = "Bit 10 - If bit 11 (SPECIAL) is set to 1, then this bit indicates whether a General Call or START byte command is to be performed by the controller. 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The controller remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. 1: START BYTE"]
    #[inline(always)]
    pub fn gc_or_start(&mut self) -> GC_OR_START_W {
        GC_OR_START_W { w: self }
    }
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits. Note: If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave"]
    #[inline(always)]
    pub fn ic_tar(&mut self) -> IC_TAR_W {
        IC_TAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Target Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tar_reg](index.html) module"]
pub struct I2C_TAR_REG_SPEC;
impl crate::RegisterSpec for I2C_TAR_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [i2c_tar_reg::R](R) reader structure"]
impl crate::Readable for I2C_TAR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_tar_reg::W](W) writer structure"]
impl crate::Writable for I2C_TAR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TAR_REG to value 0x55"]
impl crate::Resettable for I2C_TAR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55
    }
}
