#[doc = "Register `TRIM_CTRL_REG` reader"]
pub struct R(crate::R<TRIM_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_CTRL_REG` writer"]
pub struct W(crate::W<TRIM_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_CTRL_REG_SPEC>;
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
impl From<crate::W<TRIM_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_SETTLE_N` reader - Designates that the XTAL can be safely used as the CPU clock. When XTAL_CLK_CNT reases this value, the signal XTAL32M_SETTLED bit in the SYS_STAT_REG will be set. Counts in steps of 64 xtal clock-cycles."]
pub struct XTAL_SETTLE_N_R(crate::FieldReader<u8, u8>);
impl XTAL_SETTLE_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL_SETTLE_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_SETTLE_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_SETTLE_N` writer - Designates that the XTAL can be safely used as the CPU clock. When XTAL_CLK_CNT reases this value, the signal XTAL32M_SETTLED bit in the SYS_STAT_REG will be set. Counts in steps of 64 xtal clock-cycles."]
pub struct XTAL_SETTLE_N_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_SETTLE_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u16 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `XTAL_TRIM_SELECT` reader - Select which source controls the XTAL trimming 0b00: xtal counter. Starts XTAL32M_START_REG\\[XTAL32M_START\\]
after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b01: xtal OK filter. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\], when xtal amplitude is ramping is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b11: xtal OK filter, 2 stage. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\]
switches to CLK_FREQ_TRIM_REG\\[XTAL32M_RAMP\\]
after timeout (32us), and switches to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]
when xtal amplitude is ramping up."]
pub struct XTAL_TRIM_SELECT_R(crate::FieldReader<u8, u8>);
impl XTAL_TRIM_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL_TRIM_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_TRIM_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_TRIM_SELECT` writer - Select which source controls the XTAL trimming 0b00: xtal counter. Starts XTAL32M_START_REG\\[XTAL32M_START\\]
after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b01: xtal OK filter. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\], when xtal amplitude is ramping is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b11: xtal OK filter, 2 stage. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\]
switches to CLK_FREQ_TRIM_REG\\[XTAL32M_RAMP\\]
after timeout (32us), and switches to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]
when xtal amplitude is ramping up."]
pub struct XTAL_TRIM_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_TRIM_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u16 & 3) << 6);
        self.w
    }
}
#[doc = "Field `XTAL_COUNT_N` reader - Defines the number of XTAL cycles to be counted, before the xtal trimming is applied, in steps of 64 cycles. 0x01: 64 0x02: 128 0x3f: 4032"]
pub struct XTAL_COUNT_N_R(crate::FieldReader<u8, u8>);
impl XTAL_COUNT_N_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        XTAL_COUNT_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL_COUNT_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL_COUNT_N` writer - Defines the number of XTAL cycles to be counted, before the xtal trimming is applied, in steps of 64 cycles. 0x01: 64 0x02: 128 0x3f: 4032"]
pub struct XTAL_COUNT_N_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL_COUNT_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:13 - Designates that the XTAL can be safely used as the CPU clock. When XTAL_CLK_CNT reases this value, the signal XTAL32M_SETTLED bit in the SYS_STAT_REG will be set. Counts in steps of 64 xtal clock-cycles."]
    #[inline(always)]
    pub fn xtal_settle_n(&self) -> XTAL_SETTLE_N_R {
        XTAL_SETTLE_N_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Select which source controls the XTAL trimming 0b00: xtal counter. Starts XTAL32M_START_REG\\[XTAL32M_START\\]
after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b01: xtal OK filter. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\], when xtal amplitude is ramping is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b11: xtal OK filter, 2 stage. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\]
switches to CLK_FREQ_TRIM_REG\\[XTAL32M_RAMP\\]
after timeout (32us), and switches to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]
when xtal amplitude is ramping up."]
    #[inline(always)]
    pub fn xtal_trim_select(&self) -> XTAL_TRIM_SELECT_R {
        XTAL_TRIM_SELECT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 0:5 - Defines the number of XTAL cycles to be counted, before the xtal trimming is applied, in steps of 64 cycles. 0x01: 64 0x02: 128 0x3f: 4032"]
    #[inline(always)]
    pub fn xtal_count_n(&self) -> XTAL_COUNT_N_R {
        XTAL_COUNT_N_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:13 - Designates that the XTAL can be safely used as the CPU clock. When XTAL_CLK_CNT reases this value, the signal XTAL32M_SETTLED bit in the SYS_STAT_REG will be set. Counts in steps of 64 xtal clock-cycles."]
    #[inline(always)]
    pub fn xtal_settle_n(&mut self) -> XTAL_SETTLE_N_W {
        XTAL_SETTLE_N_W { w: self }
    }
    #[doc = "Bits 6:7 - Select which source controls the XTAL trimming 0b00: xtal counter. Starts XTAL32M_START_REG\\[XTAL32M_START\\]
after COUNT_N * 32 xtal pulses trim is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b01: xtal OK filter. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\], when xtal amplitude is ramping is changed to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b10: statically forced off. Only uses CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]. 0b11: xtal OK filter, 2 stage. Starts with CLK_FREQ_TRIM_REG\\[XTAL32M_START\\]
switches to CLK_FREQ_TRIM_REG\\[XTAL32M_RAMP\\]
after timeout (32us), and switches to CLK_FREQ_TRIM_REG\\[XTAL32M_TRIM\\]
when xtal amplitude is ramping up."]
    #[inline(always)]
    pub fn xtal_trim_select(&mut self) -> XTAL_TRIM_SELECT_W {
        XTAL_TRIM_SELECT_W { w: self }
    }
    #[doc = "Bits 0:5 - Defines the number of XTAL cycles to be counted, before the xtal trimming is applied, in steps of 64 cycles. 0x01: 64 0x02: 128 0x3f: 4032"]
    #[inline(always)]
    pub fn xtal_count_n(&mut self) -> XTAL_COUNT_N_W {
        XTAL_COUNT_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control trimming of the XTAL32M\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ctrl_reg](index.html) module"]
pub struct TRIM_CTRL_REG_SPEC;
impl crate::RegisterSpec for TRIM_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [trim_ctrl_reg::R](R) reader structure"]
impl crate::Readable for TRIM_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ctrl_reg::W](W) writer structure"]
impl crate::Writable for TRIM_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_CTRL_REG to value 0x3f22"]
impl crate::Resettable for TRIM_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f22
    }
}
