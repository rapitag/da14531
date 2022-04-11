#[doc = "Register `KBRD_CTRL_REG` reader"]
pub struct R(crate::R<KBRD_CTRL_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KBRD_CTRL_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KBRD_CTRL_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KBRD_CTRL_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KBRD_CTRL_REG` writer"]
pub struct W(crate::W<KBRD_CTRL_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KBRD_CTRL_REG_SPEC>;
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
impl From<crate::W<KBRD_CTRL_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KBRD_CTRL_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KBRD_REL` reader - 0 = No interrupt on key release 1 = Interrupt also on key release (also debouncing if enabled)"]
pub struct KBRD_REL_R(crate::FieldReader<bool, bool>);
impl KBRD_REL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_REL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_REL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_REL` writer - 0 = No interrupt on key release 1 = Interrupt also on key release (also debouncing if enabled)"]
pub struct KBRD_REL_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_REL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u16 & 1) << 7);
        self.w
    }
}
#[doc = "Field `KBRD_LEVEL` reader - 0 = enabled input will generate KBRD IRQ if that input is high. 1 = enabled input will generate KBRD IRQ if that input is low."]
pub struct KBRD_LEVEL_R(crate::FieldReader<bool, bool>);
impl KBRD_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KBRD_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KBRD_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KBRD_LEVEL` writer - 0 = enabled input will generate KBRD IRQ if that input is high. 1 = enabled input will generate KBRD IRQ if that input is low."]
pub struct KBRD_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> KBRD_LEVEL_W<'a> {
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
#[doc = "Field `KEY_REPEAT` reader - While key is pressed, automatically generate repeating KEYB_INT after specified time unequal to 0. Repeat time: N*1 ms. N =1..63, N=0 disables the timer."]
pub struct KEY_REPEAT_R(crate::FieldReader<u8, u8>);
impl KEY_REPEAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_REPEAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_REPEAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_REPEAT` writer - While key is pressed, automatically generate repeating KEYB_INT after specified time unequal to 0. Repeat time: N*1 ms. N =1..63, N=0 disables the timer."]
pub struct KEY_REPEAT_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_REPEAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - 0 = No interrupt on key release 1 = Interrupt also on key release (also debouncing if enabled)"]
    #[inline(always)]
    pub fn kbrd_rel(&self) -> KBRD_REL_R {
        KBRD_REL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = enabled input will generate KBRD IRQ if that input is high. 1 = enabled input will generate KBRD IRQ if that input is low."]
    #[inline(always)]
    pub fn kbrd_level(&self) -> KBRD_LEVEL_R {
        KBRD_LEVEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 0:5 - While key is pressed, automatically generate repeating KEYB_INT after specified time unequal to 0. Repeat time: N*1 ms. N =1..63, N=0 disables the timer."]
    #[inline(always)]
    pub fn key_repeat(&self) -> KEY_REPEAT_R {
        KEY_REPEAT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - 0 = No interrupt on key release 1 = Interrupt also on key release (also debouncing if enabled)"]
    #[inline(always)]
    pub fn kbrd_rel(&mut self) -> KBRD_REL_W {
        KBRD_REL_W { w: self }
    }
    #[doc = "Bit 6 - 0 = enabled input will generate KBRD IRQ if that input is high. 1 = enabled input will generate KBRD IRQ if that input is low."]
    #[inline(always)]
    pub fn kbrd_level(&mut self) -> KBRD_LEVEL_W {
        KBRD_LEVEL_W { w: self }
    }
    #[doc = "Bits 0:5 - While key is pressed, automatically generate repeating KEYB_INT after specified time unequal to 0. Repeat time: N*1 ms. N =1..63, N=0 disables the timer."]
    #[inline(always)]
    pub fn key_repeat(&mut self) -> KEY_REPEAT_W {
        KEY_REPEAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Kbrd control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kbrd_ctrl_reg](index.html) module"]
pub struct KBRD_CTRL_REG_SPEC;
impl crate::RegisterSpec for KBRD_CTRL_REG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [kbrd_ctrl_reg::R](R) reader structure"]
impl crate::Readable for KBRD_CTRL_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kbrd_ctrl_reg::W](W) writer structure"]
impl crate::Writable for KBRD_CTRL_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KBRD_CTRL_REG to value 0"]
impl crate::Resettable for KBRD_CTRL_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
