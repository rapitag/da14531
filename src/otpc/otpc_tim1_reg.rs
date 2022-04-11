#[doc = "Register `OTPC_TIM1_REG` reader"]
pub struct R(crate::R<OTPC_TIM1_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTPC_TIM1_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTPC_TIM1_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTPC_TIM1_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTPC_TIM1_REG` writer"]
pub struct W(crate::W<OTPC_TIM1_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTPC_TIM1_REG_SPEC>;
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
impl From<crate::W<OTPC_TIM1_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTPC_TIM1_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTPC_TIM1_US_T_CSP` reader - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for programming. It must be : - at least 10us - no more than 100us"]
pub struct OTPC_TIM1_US_T_CSP_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM1_US_T_CSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM1_US_T_CSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM1_US_T_CSP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM1_US_T_CSP` writer - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for programming. It must be : - at least 10us - no more than 100us"]
pub struct OTPC_TIM1_US_T_CSP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM1_US_T_CSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `OTPC_TIM1_US_T_CS` reader - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for any kind of read. It must be at least 10us."]
pub struct OTPC_TIM1_US_T_CS_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM1_US_T_CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM1_US_T_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM1_US_T_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM1_US_T_CS` writer - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for any kind of read. It must be at least 10us."]
pub struct OTPC_TIM1_US_T_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM1_US_T_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `OTPC_TIM1_US_T_PL` reader - The number of microseconds (minus one) that are required until to be enabled the LDO of the OTP. It must be at least 10us."]
pub struct OTPC_TIM1_US_T_PL_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM1_US_T_PL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM1_US_T_PL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM1_US_T_PL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM1_US_T_PL` writer - The number of microseconds (minus one) that are required until to be enabled the LDO of the OTP. It must be at least 10us."]
pub struct OTPC_TIM1_US_T_PL_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM1_US_T_PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `OTPC_TIM1_CC_T_RD` reader - The number of hclk_c clock periods (minus one) that give a time interval at least higher than 60ns. This timing parameter refers to the access time of the OTP memory."]
pub struct OTPC_TIM1_CC_T_RD_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM1_CC_T_RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM1_CC_T_RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM1_CC_T_RD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM1_CC_T_RD` writer - The number of hclk_c clock periods (minus one) that give a time interval at least higher than 60ns. This timing parameter refers to the access time of the OTP memory."]
pub struct OTPC_TIM1_CC_T_RD_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM1_CC_T_RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `OTPC_TIM1_CC_T_20NS` reader - The number of hclk_c clock periods (minus one) that give a time interval that is at least higher than 20 ns."]
pub struct OTPC_TIM1_CC_T_20NS_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM1_CC_T_20NS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM1_CC_T_20NS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM1_CC_T_20NS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM1_CC_T_20NS` writer - The number of hclk_c clock periods (minus one) that give a time interval that is at least higher than 20 ns."]
pub struct OTPC_TIM1_CC_T_20NS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM1_CC_T_20NS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `OTPC_TIM1_CC_T_1US` reader - The number of hclk_c clock periods (minus one) that give a time interval equal to 1us. This setting affects all the timing parameters that refer to microseconds, due to that defines the correspondence of a microsecond to a number of hclk_c clock cycles."]
pub struct OTPC_TIM1_CC_T_1US_R(crate::FieldReader<u8, u8>);
impl OTPC_TIM1_CC_T_1US_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OTPC_TIM1_CC_T_1US_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPC_TIM1_CC_T_1US_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPC_TIM1_CC_T_1US` writer - The number of hclk_c clock periods (minus one) that give a time interval equal to 1us. This setting affects all the timing parameters that refer to microseconds, due to that defines the correspondence of a microsecond to a number of hclk_c clock cycles."]
pub struct OTPC_TIM1_CC_T_1US_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPC_TIM1_CC_T_1US_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30 - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for programming. It must be : - at least 10us - no more than 100us"]
    #[inline(always)]
    pub fn otpc_tim1_us_t_csp(&self) -> OTPC_TIM1_US_T_CSP_R {
        OTPC_TIM1_US_T_CSP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 20:23 - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for any kind of read. It must be at least 10us."]
    #[inline(always)]
    pub fn otpc_tim1_us_t_cs(&self) -> OTPC_TIM1_US_T_CS_R {
        OTPC_TIM1_US_T_CS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - The number of microseconds (minus one) that are required until to be enabled the LDO of the OTP. It must be at least 10us."]
    #[inline(always)]
    pub fn otpc_tim1_us_t_pl(&self) -> OTPC_TIM1_US_T_PL_R {
        OTPC_TIM1_US_T_PL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - The number of hclk_c clock periods (minus one) that give a time interval at least higher than 60ns. This timing parameter refers to the access time of the OTP memory."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_rd(&self) -> OTPC_TIM1_CC_T_RD_R {
        OTPC_TIM1_CC_T_RD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 8:9 - The number of hclk_c clock periods (minus one) that give a time interval that is at least higher than 20 ns."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_20ns(&self) -> OTPC_TIM1_CC_T_20NS_R {
        OTPC_TIM1_CC_T_20NS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 0:6 - The number of hclk_c clock periods (minus one) that give a time interval equal to 1us. This setting affects all the timing parameters that refer to microseconds, due to that defines the correspondence of a microsecond to a number of hclk_c clock cycles."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_1us(&self) -> OTPC_TIM1_CC_T_1US_R {
        OTPC_TIM1_CC_T_1US_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for programming. It must be : - at least 10us - no more than 100us"]
    #[inline(always)]
    pub fn otpc_tim1_us_t_csp(&mut self) -> OTPC_TIM1_US_T_CSP_W {
        OTPC_TIM1_US_T_CSP_W { w: self }
    }
    #[doc = "Bits 20:23 - The number of microseconds (minus one) that are required after the selection of the OTP memory, until to be ready for any kind of read. It must be at least 10us."]
    #[inline(always)]
    pub fn otpc_tim1_us_t_cs(&mut self) -> OTPC_TIM1_US_T_CS_W {
        OTPC_TIM1_US_T_CS_W { w: self }
    }
    #[doc = "Bits 16:19 - The number of microseconds (minus one) that are required until to be enabled the LDO of the OTP. It must be at least 10us."]
    #[inline(always)]
    pub fn otpc_tim1_us_t_pl(&mut self) -> OTPC_TIM1_US_T_PL_W {
        OTPC_TIM1_US_T_PL_W { w: self }
    }
    #[doc = "Bits 12:14 - The number of hclk_c clock periods (minus one) that give a time interval at least higher than 60ns. This timing parameter refers to the access time of the OTP memory."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_rd(&mut self) -> OTPC_TIM1_CC_T_RD_W {
        OTPC_TIM1_CC_T_RD_W { w: self }
    }
    #[doc = "Bits 8:9 - The number of hclk_c clock periods (minus one) that give a time interval that is at least higher than 20 ns."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_20ns(&mut self) -> OTPC_TIM1_CC_T_20NS_W {
        OTPC_TIM1_CC_T_20NS_W { w: self }
    }
    #[doc = "Bits 0:6 - The number of hclk_c clock periods (minus one) that give a time interval equal to 1us. This setting affects all the timing parameters that refer to microseconds, due to that defines the correspondence of a microsecond to a number of hclk_c clock cycles."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_1us(&mut self) -> OTPC_TIM1_CC_T_1US_W {
        OTPC_TIM1_CC_T_1US_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Various timing parameters of the OTP cell.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otpc_tim1_reg](index.html) module"]
pub struct OTPC_TIM1_REG_SPEC;
impl crate::RegisterSpec for OTPC_TIM1_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otpc_tim1_reg::R](R) reader structure"]
impl crate::Readable for OTPC_TIM1_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otpc_tim1_reg::W](W) writer structure"]
impl crate::Writable for OTPC_TIM1_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTPC_TIM1_REG to value 0x0999_000f"]
impl crate::Resettable for OTPC_TIM1_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0999_000f
    }
}
