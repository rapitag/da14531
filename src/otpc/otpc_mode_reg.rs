#[doc = "Register `OTPC_MODE_REG` reader"]
pub struct R(crate::R<OTPC_MODE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_MODE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_MODE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_MODE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_MODE_REG` writer"]
pub struct W(crate::W<OTPC_MODE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_MODE_REG_SPEC>;
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
impl From<crate::W<OTPC_MODE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_MODE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_MODE_PRG_SEL` reader - Defines the part of the OTP cell that is programmed by the controller during the PROG mode, for each program request that is applied. 0x0 : Both normal and redundancy arrays are programmed. This is the normal way of programming. 0x1 : Only the normal array is programmed. 0x2 : Only the redundancy array is programmed. 0x3 : Reserved The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The setting will take effect when will be enabled again the PROG mode."]
pub struct OTPC_MODE_PRG_SEL_R(crate::FieldReader<u8, u8>);
impl OTPC_MODE_PRG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_MODE_PRG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_MODE_PRG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_MODE_PRG_SEL` writer - Defines the part of the OTP cell that is programmed by the controller during the PROG mode, for each program request that is applied. 0x0 : Both normal and redundancy arrays are programmed. This is the normal way of programming. 0x1 : Only the normal array is programmed. 0x2 : Only the redundancy array is programmed. 0x3 : Reserved The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The setting will take effect when will be enabled again the PROG mode."]
pub struct OTPC_MODE_PRG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_MODE_PRG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `OTPC_MODE_HT_MARG_EN` reader - Defines the temperature condition under which is performed a margin read. It affects only the initial margin read (RINI mode) and the programming verification margin read (PVFY). 0 : Regular temperature condition (less than 85°C) 1 : High temperature condition (85°C or more) The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next PVFY or RINI mode that will be enabled. The READ mode is not affected by the setting of this configuration bit."]
pub struct OTPC_MODE_HT_MARG_EN_R(crate::FieldReader<bool, bool>);
impl OTPC_MODE_HT_MARG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_MODE_HT_MARG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_MODE_HT_MARG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_MODE_HT_MARG_EN` writer - Defines the temperature condition under which is performed a margin read. It affects only the initial margin read (RINI mode) and the programming verification margin read (PVFY). 0 : Regular temperature condition (less than 85°C) 1 : High temperature condition (85°C or more) The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next PVFY or RINI mode that will be enabled. The READ mode is not affected by the setting of this configuration bit."]
pub struct OTPC_MODE_HT_MARG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_MODE_HT_MARG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `OTPC_MODE_USE_TST_ROW` reader - Selects the memory area of the OTP cell that will be used. 0 - Uses the main memory area of the OTP cell 1 - Uses the test row of the OTP cell The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next programming or reading mode that will be enabled."]
pub struct OTPC_MODE_USE_TST_ROW_R(crate::FieldReader<bool, bool>);
impl OTPC_MODE_USE_TST_ROW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_MODE_USE_TST_ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_MODE_USE_TST_ROW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_MODE_USE_TST_ROW` writer - Selects the memory area of the OTP cell that will be used. 0 - Uses the main memory area of the OTP cell 1 - Uses the test row of the OTP cell The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next programming or reading mode that will be enabled."]
pub struct OTPC_MODE_USE_TST_ROW_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_MODE_USE_TST_ROW_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `OTPC_MODE_MODE` reader - Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows: 0x0: DSTBY. The OTP memory is in deep standby mode (power supply ON and internal LDO OFF). 0x1: STBY. The OTP memory is powered (power supply ON and internal LDO ON, but is not selected). 0x2: READ. The OTP memory is in the normal read mode. 0x3: PROG. The OTP memory is in programming mode. 0x4: PVFY. The OTP memory is in programming verification mode (margin read after programming). 0x5: RINI. The OTP memory is in initial read mode (initial margin read). 0x6: AREAD. Copying of data from the OTP memory to a system RAM by using the internal DMA. See also the registers OTPC_AHBADR_REG, OTPC_CELADR_REG and OTPC_NWORDS_REG. Whenever the OTPC_MODE_REG\\[MODE\\]
is changing, the status bit OTPC_STAT_REG\\[OTPC_STAT_MRDY\\]
gets the value zero. The new mode will be ready for use when the OTPC_STAT_MRDY become again 1. During the mode transition the OTPC_MODE_REG\\[MODE\\]
become read only. Do not try to use or change any function of the controller until the OTPC_STAT_MRDY bit to become equal to 1. The data transferring that is performed by using the AREAD mode is completed when OTPC_STAT_MRDY becomes again 1. The mode change automatically to DSTBY with the completion of the transfer."]
pub struct OTPC_MODE_MODE_R(crate::FieldReader<u8, u8>);
impl OTPC_MODE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_MODE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_MODE_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_MODE_MODE` writer - Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows: 0x0: DSTBY. The OTP memory is in deep standby mode (power supply ON and internal LDO OFF). 0x1: STBY. The OTP memory is powered (power supply ON and internal LDO ON, but is not selected). 0x2: READ. The OTP memory is in the normal read mode. 0x3: PROG. The OTP memory is in programming mode. 0x4: PVFY. The OTP memory is in programming verification mode (margin read after programming). 0x5: RINI. The OTP memory is in initial read mode (initial margin read). 0x6: AREAD. Copying of data from the OTP memory to a system RAM by using the internal DMA. See also the registers OTPC_AHBADR_REG, OTPC_CELADR_REG and OTPC_NWORDS_REG. Whenever the OTPC_MODE_REG\\[MODE\\]
is changing, the status bit OTPC_STAT_REG\\[OTPC_STAT_MRDY\\]
gets the value zero. The new mode will be ready for use when the OTPC_STAT_MRDY become again 1. During the mode transition the OTPC_MODE_REG\\[MODE\\]
become read only. Do not try to use or change any function of the controller until the OTPC_STAT_MRDY bit to become equal to 1. The data transferring that is performed by using the AREAD mode is completed when OTPC_STAT_MRDY becomes again 1. The mode change automatically to DSTBY with the completion of the transfer."]
pub struct OTPC_MODE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_MODE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Defines the part of the OTP cell that is programmed by the controller during the PROG mode, for each program request that is applied. 0x0 : Both normal and redundancy arrays are programmed. This is the normal way of programming. 0x1 : Only the normal array is programmed. 0x2 : Only the redundancy array is programmed. 0x3 : Reserved The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The setting will take effect when will be enabled again the PROG mode."]
    #[inline(always)]
    pub fn otpc_mode_prg_sel(&self) -> OTPC_MODE_PRG_SEL_R {
        OTPC_MODE_PRG_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 5 - Defines the temperature condition under which is performed a margin read. It affects only the initial margin read (RINI mode) and the programming verification margin read (PVFY). 0 : Regular temperature condition (less than 85°C) 1 : High temperature condition (85°C or more) The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next PVFY or RINI mode that will be enabled. The READ mode is not affected by the setting of this configuration bit."]
    #[inline(always)]
    pub fn otpc_mode_ht_marg_en(&self) -> OTPC_MODE_HT_MARG_EN_R {
        OTPC_MODE_HT_MARG_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the memory area of the OTP cell that will be used. 0 - Uses the main memory area of the OTP cell 1 - Uses the test row of the OTP cell The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next programming or reading mode that will be enabled."]
    #[inline(always)]
    pub fn otpc_mode_use_tst_row(&self) -> OTPC_MODE_USE_TST_ROW_R {
        OTPC_MODE_USE_TST_ROW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:2 - Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows: 0x0: DSTBY. The OTP memory is in deep standby mode (power supply ON and internal LDO OFF). 0x1: STBY. The OTP memory is powered (power supply ON and internal LDO ON, but is not selected). 0x2: READ. The OTP memory is in the normal read mode. 0x3: PROG. The OTP memory is in programming mode. 0x4: PVFY. The OTP memory is in programming verification mode (margin read after programming). 0x5: RINI. The OTP memory is in initial read mode (initial margin read). 0x6: AREAD. Copying of data from the OTP memory to a system RAM by using the internal DMA. See also the registers OTPC_AHBADR_REG, OTPC_CELADR_REG and OTPC_NWORDS_REG. Whenever the OTPC_MODE_REG\\[MODE\\]
is changing, the status bit OTPC_STAT_REG\\[OTPC_STAT_MRDY\\]
gets the value zero. The new mode will be ready for use when the OTPC_STAT_MRDY become again 1. During the mode transition the OTPC_MODE_REG\\[MODE\\]
become read only. Do not try to use or change any function of the controller until the OTPC_STAT_MRDY bit to become equal to 1. The data transferring that is performed by using the AREAD mode is completed when OTPC_STAT_MRDY becomes again 1. The mode change automatically to DSTBY with the completion of the transfer."]
    #[inline(always)]
    pub fn otpc_mode_mode(&self) -> OTPC_MODE_MODE_R {
        OTPC_MODE_MODE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Defines the part of the OTP cell that is programmed by the controller during the PROG mode, for each program request that is applied. 0x0 : Both normal and redundancy arrays are programmed. This is the normal way of programming. 0x1 : Only the normal array is programmed. 0x2 : Only the redundancy array is programmed. 0x3 : Reserved The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The setting will take effect when will be enabled again the PROG mode."]
    #[inline(always)]
    pub fn otpc_mode_prg_sel(&mut self) -> OTPC_MODE_PRG_SEL_W {
        OTPC_MODE_PRG_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Defines the temperature condition under which is performed a margin read. It affects only the initial margin read (RINI mode) and the programming verification margin read (PVFY). 0 : Regular temperature condition (less than 85°C) 1 : High temperature condition (85°C or more) The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next PVFY or RINI mode that will be enabled. The READ mode is not affected by the setting of this configuration bit."]
    #[inline(always)]
    pub fn otpc_mode_ht_marg_en(&mut self) -> OTPC_MODE_HT_MARG_EN_W {
        OTPC_MODE_HT_MARG_EN_W { w: self }
    }
    #[doc = "Bit 4 - Selects the memory area of the OTP cell that will be used. 0 - Uses the main memory area of the OTP cell 1 - Uses the test row of the OTP cell The value of this configuration field can be modified only when the controller is in an inactive mode (DSTBY or STBY). The selection will take effect at the next programming or reading mode that will be enabled."]
    #[inline(always)]
    pub fn otpc_mode_use_tst_row(&mut self) -> OTPC_MODE_USE_TST_ROW_W {
        OTPC_MODE_USE_TST_ROW_W { w: self }
    }
    #[doc = "Bits 0:2 - Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows: 0x0: DSTBY. The OTP memory is in deep standby mode (power supply ON and internal LDO OFF). 0x1: STBY. The OTP memory is powered (power supply ON and internal LDO ON, but is not selected). 0x2: READ. The OTP memory is in the normal read mode. 0x3: PROG. The OTP memory is in programming mode. 0x4: PVFY. The OTP memory is in programming verification mode (margin read after programming). 0x5: RINI. The OTP memory is in initial read mode (initial margin read). 0x6: AREAD. Copying of data from the OTP memory to a system RAM by using the internal DMA. See also the registers OTPC_AHBADR_REG, OTPC_CELADR_REG and OTPC_NWORDS_REG. Whenever the OTPC_MODE_REG\\[MODE\\]
is changing, the status bit OTPC_STAT_REG\\[OTPC_STAT_MRDY\\]
gets the value zero. The new mode will be ready for use when the OTPC_STAT_MRDY become again 1. During the mode transition the OTPC_MODE_REG\\[MODE\\]
become read only. Do not try to use or change any function of the controller until the OTPC_STAT_MRDY bit to become equal to 1. The data transferring that is performed by using the AREAD mode is completed when OTPC_STAT_MRDY becomes again 1. The mode change automatically to DSTBY with the completion of the transfer."]
    #[inline(always)]
    pub fn otpc_mode_mode(&mut self) -> OTPC_MODE_MODE_W {
        OTPC_MODE_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_mode_reg](index.html) module"]
pub struct OTPC_MODE_REG_SPEC;
impl crate::RegisterSpec for OTPC_MODE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_mode_reg::R](R) reader structure"]
impl crate::Readable for OTPC_MODE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_mode_reg::W](W) writer structure"]
impl crate::Writable for OTPC_MODE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_MODE_REG to value 0"]
impl crate::Resettable for OTPC_MODE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
