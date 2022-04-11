#[doc = "Register `PMU_CTRL_REG` reader"]
pub struct R(crate::R<PMU_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMU_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMU_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMU_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMU_CTRL_REG` writer"]
pub struct W(crate::W<PMU_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMU_CTRL_REG_SPEC>;
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
impl From<crate::W<PMU_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMU_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAP_BANDGAP_EN` reader - Enable wakeup diagnostics mapping. When set, these functions are mapped (please set direction to output) P0\\[2\\]: BANDGAP_ENABLE P0\\[1\\]: Power WOKENUP Note: P0\\[2\\]
assigned also to SWD_CLK, thus the debugger must be detached before entering into sleep mode with MAP_BANDGAP_EN=1. Refer also to SYS_STAT_REG->DBG_IS_UP."]
pub struct MAP_BANDGAP_EN_R(crate::FieldReader<bool, bool>);
impl MAP_BANDGAP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAP_BANDGAP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAP_BANDGAP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAP_BANDGAP_EN` writer - Enable wakeup diagnostics mapping. When set, these functions are mapped (please set direction to output) P0\\[2\\]: BANDGAP_ENABLE P0\\[1\\]: Power WOKENUP Note: P0\\[2\\]
assigned also to SWD_CLK, thus the debugger must be detached before entering into sleep mode with MAP_BANDGAP_EN=1. Refer also to SYS_STAT_REG->DBG_IS_UP."]
pub struct MAP_BANDGAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_BANDGAP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u16 & 1) << 6);
        self.w
    }
}
#[doc = "Field `OTP_COPY_DIV` reader - Sets the HCLK division during OTP mirroring"]
pub struct OTP_COPY_DIV_R(crate::FieldReader<u8, u8>);
impl OTP_COPY_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTP_COPY_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_COPY_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP_COPY_DIV` writer - Sets the HCLK division during OTP mirroring"]
pub struct OTP_COPY_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_COPY_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u16 & 3) << 4);
        self.w
    }
}
#[doc = "Field `RADIO_SLEEP` reader - Put the digital part of the radio in powerdown"]
pub struct RADIO_SLEEP_R(crate::FieldReader<bool, bool>);
impl RADIO_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RADIO_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADIO_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADIO_SLEEP` writer - Put the digital part of the radio in powerdown"]
pub struct RADIO_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> RADIO_SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u16 & 1) << 2);
        self.w
    }
}
#[doc = "Field `TIM_SLEEP` reader - Put PD_TIM in powerdown"]
pub struct TIM_SLEEP_R(crate::FieldReader<bool, bool>);
impl TIM_SLEEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIM_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM_SLEEP` writer - Put PD_TIM in powerdown"]
pub struct TIM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM_SLEEP_W<'a> {
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
#[doc = "Field `RESET_ON_WAKEUP` reader - Perform a Hardware Reset after waking up. Booter will be started."]
pub struct RESET_ON_WAKEUP_R(crate::FieldReader<bool, bool>);
impl RESET_ON_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_ON_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_ON_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_ON_WAKEUP` writer - Perform a Hardware Reset after waking up. Booter will be started."]
pub struct RESET_ON_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_ON_WAKEUP_W<'a> {
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
    #[doc = "Bit 6 - Enable wakeup diagnostics mapping. When set, these functions are mapped (please set direction to output) P0\\[2\\]: BANDGAP_ENABLE P0\\[1\\]: Power WOKENUP Note: P0\\[2\\]
assigned also to SWD_CLK, thus the debugger must be detached before entering into sleep mode with MAP_BANDGAP_EN=1. Refer also to SYS_STAT_REG->DBG_IS_UP."]
    #[inline(always)]
    pub fn map_bandgap_en(&self) -> MAP_BANDGAP_EN_R {
        MAP_BANDGAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sets the HCLK division during OTP mirroring"]
    #[inline(always)]
    pub fn otp_copy_div(&self) -> OTP_COPY_DIV_R {
        OTP_COPY_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 2 - Put the digital part of the radio in powerdown"]
    #[inline(always)]
    pub fn radio_sleep(&self) -> RADIO_SLEEP_R {
        RADIO_SLEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Put PD_TIM in powerdown"]
    #[inline(always)]
    pub fn tim_sleep(&self) -> TIM_SLEEP_R {
        TIM_SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Perform a Hardware Reset after waking up. Booter will be started."]
    #[inline(always)]
    pub fn reset_on_wakeup(&self) -> RESET_ON_WAKEUP_R {
        RESET_ON_WAKEUP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enable wakeup diagnostics mapping. When set, these functions are mapped (please set direction to output) P0\\[2\\]: BANDGAP_ENABLE P0\\[1\\]: Power WOKENUP Note: P0\\[2\\]
assigned also to SWD_CLK, thus the debugger must be detached before entering into sleep mode with MAP_BANDGAP_EN=1. Refer also to SYS_STAT_REG->DBG_IS_UP."]
    #[inline(always)]
    pub fn map_bandgap_en(&mut self) -> MAP_BANDGAP_EN_W {
        MAP_BANDGAP_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Sets the HCLK division during OTP mirroring"]
    #[inline(always)]
    pub fn otp_copy_div(&mut self) -> OTP_COPY_DIV_W {
        OTP_COPY_DIV_W { w: self }
    }
    #[doc = "Bit 2 - Put the digital part of the radio in powerdown"]
    #[inline(always)]
    pub fn radio_sleep(&mut self) -> RADIO_SLEEP_W {
        RADIO_SLEEP_W { w: self }
    }
    #[doc = "Bit 1 - Put PD_TIM in powerdown"]
    #[inline(always)]
    pub fn tim_sleep(&mut self) -> TIM_SLEEP_W {
        TIM_SLEEP_W { w: self }
    }
    #[doc = "Bit 0 - Perform a Hardware Reset after waking up. Booter will be started."]
    #[inline(always)]
    pub fn reset_on_wakeup(&mut self) -> RESET_ON_WAKEUP_W {
        RESET_ON_WAKEUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management Unit control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmu_ctrl_reg](index.html) module"]
pub struct PMU_CTRL_REG_SPEC;
impl crate::RegisterSpec for PMU_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmu_ctrl_reg::R](R) reader structure"]
impl crate::Readable for PMU_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmu_ctrl_reg::W](W) writer structure"]
impl crate::Writable for PMU_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMU_CTRL_REG to value 0x06"]
impl crate::Resettable for PMU_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
