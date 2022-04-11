#[doc = "Register `WATCHDOG_REG` reader"]
pub struct R(crate::R<WATCHDOG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WATCHDOG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WATCHDOG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WATCHDOG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WATCHDOG_REG` writer"]
pub struct W(crate::W<WATCHDOG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WATCHDOG_REG_SPEC>;
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
impl From<crate::W<WATCHDOG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WATCHDOG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOG_WEN` writer - 0000.000 = Write enable for Watchdog timer else Write disable. This filter prevents unintentional presetting the watchdog with a SW run-away."]
pub struct WDOG_WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_WEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u16 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `WDOG_VAL_NEG` reader - 0 = Watchdog timer value is positive. 1 = Watchdog timer value is negative."]
pub struct WDOG_VAL_NEG_R(crate::FieldReader<bool, bool>);
impl WDOG_VAL_NEG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDOG_VAL_NEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_VAL_NEG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_VAL_NEG` writer - 0 = Watchdog timer value is positive. 1 = Watchdog timer value is negative."]
pub struct WDOG_VAL_NEG_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_VAL_NEG_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u16 & 1) << 8);
        self.w
    }
}
#[doc = "Field `WDOG_VAL` reader - Write: Watchdog timer reload value. Note that all bits 15-9 must be 0 to reload this register. Read: Actual Watchdog timer value. Decremented by 1 every 10.24 msec. Bit 8 indicates a negative counter value. 2, 1, 0, 1FF16, 1FE16 etc. An NMI or WDOG (SYS) reset is generated under the following conditions: If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 0 then If WDOG_VAL = 0 -> NMI (Non Maskable Interrupt) if WDOG_VAL = 1F016 -> WDOG reset -> reload FF16 If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 1 then if WDOG_VAL <= 0 -> WDOG reset -> reload FF16"]
pub struct WDOG_VAL_R(crate::FieldReader<u8, u8>);
impl WDOG_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDOG_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOG_VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOG_VAL` writer - Write: Watchdog timer reload value. Note that all bits 15-9 must be 0 to reload this register. Read: Actual Watchdog timer value. Decremented by 1 every 10.24 msec. Bit 8 indicates a negative counter value. 2, 1, 0, 1FF16, 1FE16 etc. An NMI or WDOG (SYS) reset is generated under the following conditions: If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 0 then If WDOG_VAL = 0 -> NMI (Non Maskable Interrupt) if WDOG_VAL = 1F016 -> WDOG reset -> reload FF16 If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 1 then if WDOG_VAL <= 0 -> WDOG reset -> reload FF16"]
pub struct WDOG_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOG_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - 0 = Watchdog timer value is positive. 1 = Watchdog timer value is negative."]
    #[inline(always)]
    pub fn wdog_val_neg(&self) -> WDOG_VAL_NEG_R {
        WDOG_VAL_NEG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Write: Watchdog timer reload value. Note that all bits 15-9 must be 0 to reload this register. Read: Actual Watchdog timer value. Decremented by 1 every 10.24 msec. Bit 8 indicates a negative counter value. 2, 1, 0, 1FF16, 1FE16 etc. An NMI or WDOG (SYS) reset is generated under the following conditions: If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 0 then If WDOG_VAL = 0 -> NMI (Non Maskable Interrupt) if WDOG_VAL = 1F016 -> WDOG reset -> reload FF16 If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 1 then if WDOG_VAL <= 0 -> WDOG reset -> reload FF16"]
    #[inline(always)]
    pub fn wdog_val(&self) -> WDOG_VAL_R {
        WDOG_VAL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 9:15 - 0000.000 = Write enable for Watchdog timer else Write disable. This filter prevents unintentional presetting the watchdog with a SW run-away."]
    #[inline(always)]
    pub fn wdog_wen(&mut self) -> WDOG_WEN_W {
        WDOG_WEN_W { w: self }
    }
    #[doc = "Bit 8 - 0 = Watchdog timer value is positive. 1 = Watchdog timer value is negative."]
    #[inline(always)]
    pub fn wdog_val_neg(&mut self) -> WDOG_VAL_NEG_W {
        WDOG_VAL_NEG_W { w: self }
    }
    #[doc = "Bits 0:7 - Write: Watchdog timer reload value. Note that all bits 15-9 must be 0 to reload this register. Read: Actual Watchdog timer value. Decremented by 1 every 10.24 msec. Bit 8 indicates a negative counter value. 2, 1, 0, 1FF16, 1FE16 etc. An NMI or WDOG (SYS) reset is generated under the following conditions: If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 0 then If WDOG_VAL = 0 -> NMI (Non Maskable Interrupt) if WDOG_VAL = 1F016 -> WDOG reset -> reload FF16 If WATCHDOG_CTRL_REG\\[NMI_RST\\]
= 1 then if WDOG_VAL <= 0 -> WDOG reset -> reload FF16"]
    #[inline(always)]
    pub fn wdog_val(&mut self) -> WDOG_VAL_W {
        WDOG_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timer register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [watchdog_reg](index.html) module"]
pub struct WATCHDOG_REG_SPEC;
impl crate::RegisterSpec for WATCHDOG_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [watchdog_reg::R](R) reader structure"]
impl crate::Readable for WATCHDOG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [watchdog_reg::W](W) writer structure"]
impl crate::Writable for WATCHDOG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WATCHDOG_REG to value 0xff"]
impl crate::Resettable for WATCHDOG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
