#[doc = "Register `CLK_CTRL_REG` reader"]
pub struct R(crate::R<CLK_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL_REG` writer"]
pub struct W(crate::W<CLK_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL_REG_SPEC>;
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
impl From<crate::W<CLK_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUNNING_AT_XTAL32M` reader - Indicates that the XTAL32M clock is used as clock, and may not be switched off"]
pub struct RUNNING_AT_XTAL32M_R(crate::FieldReader<bool, bool>);
impl RUNNING_AT_XTAL32M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNNING_AT_XTAL32M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNNING_AT_XTAL32M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNNING_AT_RC32M` reader - Indicates that the RC32M clock is used as clock"]
pub struct RUNNING_AT_RC32M_R(crate::FieldReader<bool, bool>);
impl RUNNING_AT_RC32M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNNING_AT_RC32M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNNING_AT_RC32M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNNING_AT_LP_CLK` reader - Indicates that either the LP_CLK is being used as system clock"]
pub struct RUNNING_AT_LP_CLK_R(crate::FieldReader<bool, bool>);
impl RUNNING_AT_LP_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNNING_AT_LP_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNNING_AT_LP_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP_CLK_SEL` reader - Sets the clock source of the LowerPower clock 0x0: RC32K 0x1: RCX 0x2: XTAL32K through the oscillator with an external Crystal. 0x3: XTAL32K through an external square wave generator (set PID of P0\\[3\\]
to FUNC_GPIO) Change this setting before using this clock, and while RUNNING_AT_LP_CLK == 0."]
pub struct LP_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl LP_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LP_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LP_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LP_CLK_SEL` writer - Sets the clock source of the LowerPower clock 0x0: RC32K 0x1: RCX 0x2: XTAL32K through the oscillator with an external Crystal. 0x3: XTAL32K through an external square wave generator (set PID of P0\\[3\\]
to FUNC_GPIO) Change this setting before using this clock, and while RUNNING_AT_LP_CLK == 0."]
pub struct LP_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 3)) | ((value as u16 & 3) << 3);
        self.w
    }
}
#[doc = "Field `XTAL32M_DISABLE` reader - Setting this bit instantaneously disables the 32 MHz crystal oscillator. Also, after sleep/wakeup cycle, the oscillator will not be enabled. This bit may not be set to '1'when \"RUNNING_AT_XTAL32M is '1' to prevent deadlock. After resetting this bit, wait for XTAL32M_SETTLED or XTAL32M_TRIM_READY to become '1' before switching to XTAL32M clock source."]
pub struct XTAL32M_DISABLE_R(crate::FieldReader<bool, bool>);
impl XTAL32M_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTAL32M_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTAL32M_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTAL32M_DISABLE` writer - Setting this bit instantaneously disables the 32 MHz crystal oscillator. Also, after sleep/wakeup cycle, the oscillator will not be enabled. This bit may not be set to '1'when \"RUNNING_AT_XTAL32M is '1' to prevent deadlock. After resetting this bit, wait for XTAL32M_SETTLED or XTAL32M_TRIM_READY to become '1' before switching to XTAL32M clock source."]
pub struct XTAL32M_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> XTAL32M_DISABLE_W<'a> {
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
#[doc = "Field `SYS_CLK_SEL` reader - Selects the clock source. 0x0: XTAL32M (check the XTAL32M_SETTLED and XTAL32M_TRIM_READY bits!!) 0x1: RC32M 0x2/0x3: LP_CLK"]
pub struct SYS_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl SYS_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYS_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_CLK_SEL` writer - Selects the clock source. 0x0: XTAL32M (check the XTAL32M_SETTLED and XTAL32M_TRIM_READY bits!!) 0x1: RC32M 0x2/0x3: LP_CLK"]
pub struct SYS_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u16 & 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Indicates that the XTAL32M clock is used as clock, and may not be switched off"]
    #[inline(always)]
    pub fn running_at_xtal32m(&self) -> RUNNING_AT_XTAL32M_R {
        RUNNING_AT_XTAL32M_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that the RC32M clock is used as clock"]
    #[inline(always)]
    pub fn running_at_rc32m(&self) -> RUNNING_AT_RC32M_R {
        RUNNING_AT_RC32M_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that either the LP_CLK is being used as system clock"]
    #[inline(always)]
    pub fn running_at_lp_clk(&self) -> RUNNING_AT_LP_CLK_R {
        RUNNING_AT_LP_CLK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Sets the clock source of the LowerPower clock 0x0: RC32K 0x1: RCX 0x2: XTAL32K through the oscillator with an external Crystal. 0x3: XTAL32K through an external square wave generator (set PID of P0\\[3\\]
to FUNC_GPIO) Change this setting before using this clock, and while RUNNING_AT_LP_CLK == 0."]
    #[inline(always)]
    pub fn lp_clk_sel(&self) -> LP_CLK_SEL_R {
        LP_CLK_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 2 - Setting this bit instantaneously disables the 32 MHz crystal oscillator. Also, after sleep/wakeup cycle, the oscillator will not be enabled. This bit may not be set to '1'when \"RUNNING_AT_XTAL32M is '1' to prevent deadlock. After resetting this bit, wait for XTAL32M_SETTLED or XTAL32M_TRIM_READY to become '1' before switching to XTAL32M clock source."]
    #[inline(always)]
    pub fn xtal32m_disable(&self) -> XTAL32M_DISABLE_R {
        XTAL32M_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 0:1 - Selects the clock source. 0x0: XTAL32M (check the XTAL32M_SETTLED and XTAL32M_TRIM_READY bits!!) 0x1: RC32M 0x2/0x3: LP_CLK"]
    #[inline(always)]
    pub fn sys_clk_sel(&self) -> SYS_CLK_SEL_R {
        SYS_CLK_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4 - Sets the clock source of the LowerPower clock 0x0: RC32K 0x1: RCX 0x2: XTAL32K through the oscillator with an external Crystal. 0x3: XTAL32K through an external square wave generator (set PID of P0\\[3\\]
to FUNC_GPIO) Change this setting before using this clock, and while RUNNING_AT_LP_CLK == 0."]
    #[inline(always)]
    pub fn lp_clk_sel(&mut self) -> LP_CLK_SEL_W {
        LP_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 2 - Setting this bit instantaneously disables the 32 MHz crystal oscillator. Also, after sleep/wakeup cycle, the oscillator will not be enabled. This bit may not be set to '1'when \"RUNNING_AT_XTAL32M is '1' to prevent deadlock. After resetting this bit, wait for XTAL32M_SETTLED or XTAL32M_TRIM_READY to become '1' before switching to XTAL32M clock source."]
    #[inline(always)]
    pub fn xtal32m_disable(&mut self) -> XTAL32M_DISABLE_W {
        XTAL32M_DISABLE_W { w: self }
    }
    #[doc = "Bits 0:1 - Selects the clock source. 0x0: XTAL32M (check the XTAL32M_SETTLED and XTAL32M_TRIM_READY bits!!) 0x1: RC32M 0x2/0x3: LP_CLK"]
    #[inline(always)]
    pub fn sys_clk_sel(&mut self) -> SYS_CLK_SEL_W {
        SYS_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl_reg](index.html) module"]
pub struct CLK_CTRL_REG_SPEC;
impl crate::RegisterSpec for CLK_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [clk_ctrl_reg::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl_reg::W](W) writer structure"]
impl crate::Writable for CLK_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CTRL_REG to value 0x41"]
impl crate::Resettable for CLK_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x41
    }
}
