#[doc = "Register `OTPC_TIM2_REG` reader"]
pub struct R(crate::R<OTPC_TIM2_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_TIM2_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_TIM2_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_TIM2_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_TIM2_REG` writer"]
pub struct W(crate::W<OTPC_TIM2_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_TIM2_REG_SPEC>;
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
impl From<crate::W<OTPC_TIM2_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_TIM2_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_TIM2_US_ADD_CC_EN` reader - Adds an additional hclk_c clock cycle at all the time intervals that count in microseconds. 0 : The extra hclk_c clock cycle is not applied 1 : The extra hclk_c clock cycle is applied"]
pub struct OTPC_TIM2_US_ADD_CC_EN_R(crate::FieldReader<bool, bool>);
impl OTPC_TIM2_US_ADD_CC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTPC_TIM2_US_ADD_CC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_ADD_CC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_ADD_CC_EN` writer - Adds an additional hclk_c clock cycle at all the time intervals that count in microseconds. 0 : The extra hclk_c clock cycle is not applied 1 : The extra hclk_c clock cycle is applied"]
pub struct OTPC_TIM2_US_ADD_CC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_ADD_CC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_SAS` reader - The number of microseconds (minus one) that are required after the exit from the deep sleep standby mode and before to become ready to enter in an active mode (reading or programming). It must be at least 2us."]
pub struct OTPC_TIM2_US_T_SAS_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_SAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_SAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_SAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_SAS` writer - The number of microseconds (minus one) that are required after the exit from the deep sleep standby mode and before to become ready to enter in an active mode (reading or programming). It must be at least 2us."]
pub struct OTPC_TIM2_US_T_SAS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_SAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 29)) | ((value as u32 & 3) << 29);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PPH` reader - The number of microseconds (minus one) that are required after the last programming pulse and before to be disabled the programming mode in the OTP memory. It must be: - at least 5us - no more than 20us"]
pub struct OTPC_TIM2_US_T_PPH_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_PPH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_PPH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_PPH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PPH` writer - The number of microseconds (minus one) that are required after the last programming pulse and before to be disabled the programming mode in the OTP memory. It must be: - at least 5us - no more than 20us"]
pub struct OTPC_TIM2_US_T_PPH_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_PPH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_VDS` reader - The number of microseconds (minus one) that are required after the enabling of the power supply of the OTP memory and before to become ready for the enabling of the internal LDO. It must be at least 1us."]
pub struct OTPC_TIM2_US_T_VDS_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_VDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_VDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_VDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_VDS` writer - The number of microseconds (minus one) that are required after the enabling of the power supply of the OTP memory and before to become ready for the enabling of the internal LDO. It must be at least 1us."]
pub struct OTPC_TIM2_US_T_VDS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_VDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 21)) | ((value as u32 & 7) << 21);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PPS` reader - The number of microseconds (minus one) that are required after the enabling of the programming in the OTP memory and before to be applied the first programming pulse. It must be : - at least 5us - no more than 20us"]
pub struct OTPC_TIM2_US_T_PPS_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_PPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_PPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_PPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PPS` writer - The number of microseconds (minus one) that are required after the enabling of the programming in the OTP memory and before to be applied the first programming pulse. It must be : - at least 5us - no more than 20us"]
pub struct OTPC_TIM2_US_T_PPS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_PPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PPR` reader - The number of microseconds (minus one) for recovery after a programming sequence. It must be : - at least 5us - no more than 100us"]
pub struct OTPC_TIM2_US_T_PPR_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_PPR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_PPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_PPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PPR` writer - The number of microseconds (minus one) for recovery after a programming sequence. It must be : - at least 5us - no more than 100us"]
pub struct OTPC_TIM2_US_T_PPR_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_PPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PWI` reader - The number of microseconds (minus one) between two consecutive programming pulses. It must be : - at least 1us - no more than 5us"]
pub struct OTPC_TIM2_US_T_PWI_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_PWI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_PWI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_PWI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PWI` writer - The number of microseconds (minus one) between two consecutive programming pulses. It must be : - at least 1us - no more than 5us"]
pub struct OTPC_TIM2_US_T_PWI_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_PWI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 5)) | ((value as u32 & 7) << 5);
        self.w
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PW` reader - The number of microseconds (minus one) that lasts the programming of each bit. It must be : - at least 10us - no more than 20us"]
pub struct OTPC_TIM2_US_T_PW_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM2_US_T_PW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM2_US_T_PW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM2_US_T_PW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM2_US_T_PW` writer - The number of microseconds (minus one) that lasts the programming of each bit. It must be : - at least 10us - no more than 20us"]
pub struct OTPC_TIM2_US_T_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM2_US_T_PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Adds an additional hclk_c clock cycle at all the time intervals that count in microseconds. 0 : The extra hclk_c clock cycle is not applied 1 : The extra hclk_c clock cycle is applied"]
    #[inline(always)]
    pub fn otpc_tim2_us_add_cc_en(&self) -> OTPC_TIM2_US_ADD_CC_EN_R {
        OTPC_TIM2_US_ADD_CC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 29:30 - The number of microseconds (minus one) that are required after the exit from the deep sleep standby mode and before to become ready to enter in an active mode (reading or programming). It must be at least 2us."]
    #[inline(always)]
    pub fn otpc_tim2_us_t_sas(&self) -> OTPC_TIM2_US_T_SAS_R {
        OTPC_TIM2_US_T_SAS_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bits 24:28 - The number of microseconds (minus one) that are required after the last programming pulse and before to be disabled the programming mode in the OTP memory. It must be: - at least 5us - no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pph(&self) -> OTPC_TIM2_US_T_PPH_R {
        OTPC_TIM2_US_T_PPH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - The number of microseconds (minus one) that are required after the enabling of the power supply of the OTP memory and before to become ready for the enabling of the internal LDO. It must be at least 1us."]
    #[inline(always)]
    pub fn otpc_tim2_us_t_vds(&self) -> OTPC_TIM2_US_T_VDS_R {
        OTPC_TIM2_US_T_VDS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 16:20 - The number of microseconds (minus one) that are required after the enabling of the programming in the OTP memory and before to be applied the first programming pulse. It must be : - at least 5us - no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pps(&self) -> OTPC_TIM2_US_T_PPS_R {
        OTPC_TIM2_US_T_PPS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - The number of microseconds (minus one) for recovery after a programming sequence. It must be : - at least 5us - no more than 100us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_ppr(&self) -> OTPC_TIM2_US_T_PPR_R {
        OTPC_TIM2_US_T_PPR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 5:7 - The number of microseconds (minus one) between two consecutive programming pulses. It must be : - at least 1us - no more than 5us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pwi(&self) -> OTPC_TIM2_US_T_PWI_R {
        OTPC_TIM2_US_T_PWI_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 0:4 - The number of microseconds (minus one) that lasts the programming of each bit. It must be : - at least 10us - no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pw(&self) -> OTPC_TIM2_US_T_PW_R {
        OTPC_TIM2_US_T_PW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Adds an additional hclk_c clock cycle at all the time intervals that count in microseconds. 0 : The extra hclk_c clock cycle is not applied 1 : The extra hclk_c clock cycle is applied"]
    #[inline(always)]
    pub fn otpc_tim2_us_add_cc_en(&mut self) -> OTPC_TIM2_US_ADD_CC_EN_W {
        OTPC_TIM2_US_ADD_CC_EN_W { w: self }
    }
    #[doc = "Bits 29:30 - The number of microseconds (minus one) that are required after the exit from the deep sleep standby mode and before to become ready to enter in an active mode (reading or programming). It must be at least 2us."]
    #[inline(always)]
    pub fn otpc_tim2_us_t_sas(&mut self) -> OTPC_TIM2_US_T_SAS_W {
        OTPC_TIM2_US_T_SAS_W { w: self }
    }
    #[doc = "Bits 24:28 - The number of microseconds (minus one) that are required after the last programming pulse and before to be disabled the programming mode in the OTP memory. It must be: - at least 5us - no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pph(&mut self) -> OTPC_TIM2_US_T_PPH_W {
        OTPC_TIM2_US_T_PPH_W { w: self }
    }
    #[doc = "Bits 21:23 - The number of microseconds (minus one) that are required after the enabling of the power supply of the OTP memory and before to become ready for the enabling of the internal LDO. It must be at least 1us."]
    #[inline(always)]
    pub fn otpc_tim2_us_t_vds(&mut self) -> OTPC_TIM2_US_T_VDS_W {
        OTPC_TIM2_US_T_VDS_W { w: self }
    }
    #[doc = "Bits 16:20 - The number of microseconds (minus one) that are required after the enabling of the programming in the OTP memory and before to be applied the first programming pulse. It must be : - at least 5us - no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pps(&mut self) -> OTPC_TIM2_US_T_PPS_W {
        OTPC_TIM2_US_T_PPS_W { w: self }
    }
    #[doc = "Bits 8:14 - The number of microseconds (minus one) for recovery after a programming sequence. It must be : - at least 5us - no more than 100us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_ppr(&mut self) -> OTPC_TIM2_US_T_PPR_W {
        OTPC_TIM2_US_T_PPR_W { w: self }
    }
    #[doc = "Bits 5:7 - The number of microseconds (minus one) between two consecutive programming pulses. It must be : - at least 1us - no more than 5us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pwi(&mut self) -> OTPC_TIM2_US_T_PWI_W {
        OTPC_TIM2_US_T_PWI_W { w: self }
    }
    #[doc = "Bits 0:4 - The number of microseconds (minus one) that lasts the programming of each bit. It must be : - at least 10us - no more than 20us"]
    #[inline(always)]
    pub fn otpc_tim2_us_t_pw(&mut self) -> OTPC_TIM2_US_T_PW_W {
        OTPC_TIM2_US_T_PW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various timing parameters of the OTP cell.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_tim2_reg](index.html) module"]
pub struct OTPC_TIM2_REG_SPEC;
impl crate::RegisterSpec for OTPC_TIM2_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_tim2_reg::R](R) reader structure"]
impl crate::Readable for OTPC_TIM2_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_tim2_reg::W](W) writer structure"]
impl crate::Writable for OTPC_TIM2_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_TIM2_REG to value 0x7fff_ffff"]
impl crate::Resettable for OTPC_TIM2_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7fff_ffff
    }
}
