#[doc = "Register `TIMER1_CAPTURE_REG` reader"]
pub struct R(crate::R<TIMER1_CAPTURE_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_CAPTURE_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_CAPTURE_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_CAPTURE_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1_CAPTURE_REG` writer"]
pub struct W(crate::W<TIMER1_CAPTURE_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_CAPTURE_REG_SPEC>;
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
impl From<crate::W<TIMER1_CAPTURE_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_CAPTURE_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER1_IN2_STAMP_TYPE` reader - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
pub struct TIMER1_IN2_STAMP_TYPE_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN2_STAMP_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN2_STAMP_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_STAMP_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN2_STAMP_TYPE` writer - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
pub struct TIMER1_IN2_STAMP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN2_STAMP_TYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `TIMER1_IN2_PERIOD_MAX` reader - Gives the number of periods +1 of IN2, in which module counts"]
pub struct TIMER1_IN2_PERIOD_MAX_R(crate::FieldReader<u8, u8>);
impl TIMER1_IN2_PERIOD_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_IN2_PERIOD_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_PERIOD_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN2_PERIOD_MAX` writer - Gives the number of periods +1 of IN2, in which module counts"]
pub struct TIMER1_IN2_PERIOD_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN2_PERIOD_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 21)) | ((value as u32 & 0x3f) << 21);
        self.w
    }
}
#[doc = "Field `TIMER1_IN2_IRQ_EN` reader - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN2_PERIOD_MAX 0 = Interrupt is masked"]
pub struct TIMER1_IN2_IRQ_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN2_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN2_IRQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_IRQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN2_IRQ_EN` writer - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN2_PERIOD_MAX 0 = Interrupt is masked"]
pub struct TIMER1_IN2_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN2_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `TIMER1_IN2_COUNT_EN` reader - 0 = Capture mode 1 = Count mode"]
pub struct TIMER1_IN2_COUNT_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN2_COUNT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN2_COUNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_COUNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN2_COUNT_EN` writer - 0 = Capture mode 1 = Count mode"]
pub struct TIMER1_IN2_COUNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN2_COUNT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `TIMER1_IN2_EVENT_FALL_EN` reader - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO2_CONF=0 to prevent false events"]
pub struct TIMER1_IN2_EVENT_FALL_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN2_EVENT_FALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN2_EVENT_FALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN2_EVENT_FALL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN2_EVENT_FALL_EN` writer - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO2_CONF=0 to prevent false events"]
pub struct TIMER1_IN2_EVENT_FALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN2_EVENT_FALL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `TIMER1_GPIO2_CONF` reader - 0,13,14,15 = IN2 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN2"]
pub struct TIMER1_GPIO2_CONF_R(crate::FieldReader<u8, u8>);
impl TIMER1_GPIO2_CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_GPIO2_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_GPIO2_CONF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_GPIO2_CONF` writer - 0,13,14,15 = IN2 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN2"]
pub struct TIMER1_GPIO2_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_GPIO2_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | ((value as u32 & 0x0f) << 14);
        self.w
    }
}
#[doc = "Field `TIMER1_IN1_STAMP_TYPE` reader - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
pub struct TIMER1_IN1_STAMP_TYPE_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN1_STAMP_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN1_STAMP_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_STAMP_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_STAMP_TYPE` writer - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
pub struct TIMER1_IN1_STAMP_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN1_STAMP_TYPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `TIMER1_IN1_PERIOD_MAX` reader - Gives the number of periods +1 of IN1, in which module counts"]
pub struct TIMER1_IN1_PERIOD_MAX_R(crate::FieldReader<u8, u8>);
impl TIMER1_IN1_PERIOD_MAX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_IN1_PERIOD_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_PERIOD_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_PERIOD_MAX` writer - Gives the number of periods +1 of IN1, in which module counts"]
pub struct TIMER1_IN1_PERIOD_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN1_PERIOD_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `TIMER1_IN1_IRQ_EN` reader - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN1_PERIOD_MAX 0 = Interrupt is masked"]
pub struct TIMER1_IN1_IRQ_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN1_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN1_IRQ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_IRQ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_IRQ_EN` writer - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN1_PERIOD_MAX 0 = Interrupt is masked"]
pub struct TIMER1_IN1_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN1_IRQ_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `TIMER1_IN1_COUNT_EN` reader - 0 = Capture mode 1 = Count mode"]
pub struct TIMER1_IN1_COUNT_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN1_COUNT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN1_COUNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_COUNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_COUNT_EN` writer - 0 = Capture mode 1 = Count mode"]
pub struct TIMER1_IN1_COUNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN1_COUNT_EN_W<'a> {
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
#[doc = "Field `TIMER1_IN1_EVENT_FALL_EN` reader - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO1_CONF=0 to prevent false events"]
pub struct TIMER1_IN1_EVENT_FALL_EN_R(crate::FieldReader<bool, bool>);
impl TIMER1_IN1_EVENT_FALL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_IN1_EVENT_FALL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_IN1_EVENT_FALL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_IN1_EVENT_FALL_EN` writer - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO1_CONF=0 to prevent false events"]
pub struct TIMER1_IN1_EVENT_FALL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_IN1_EVENT_FALL_EN_W<'a> {
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
#[doc = "Field `TIMER1_GPIO1_CONF` reader - 0,13,14,15 = IN1 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN1"]
pub struct TIMER1_GPIO1_CONF_R(crate::FieldReader<u8, u8>);
impl TIMER1_GPIO1_CONF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_GPIO1_CONF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_GPIO1_CONF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER1_GPIO1_CONF` writer - 0,13,14,15 = IN1 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN1"]
pub struct TIMER1_GPIO1_CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_GPIO1_CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
    #[inline(always)]
    pub fn timer1_in2_stamp_type(&self) -> TIMER1_IN2_STAMP_TYPE_R {
        TIMER1_IN2_STAMP_TYPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 21:26 - Gives the number of periods +1 of IN2, in which module counts"]
    #[inline(always)]
    pub fn timer1_in2_period_max(&self) -> TIMER1_IN2_PERIOD_MAX_R {
        TIMER1_IN2_PERIOD_MAX_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN2_PERIOD_MAX 0 = Interrupt is masked"]
    #[inline(always)]
    pub fn timer1_in2_irq_en(&self) -> TIMER1_IN2_IRQ_EN_R {
        TIMER1_IN2_IRQ_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - 0 = Capture mode 1 = Count mode"]
    #[inline(always)]
    pub fn timer1_in2_count_en(&self) -> TIMER1_IN2_COUNT_EN_R {
        TIMER1_IN2_COUNT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO2_CONF=0 to prevent false events"]
    #[inline(always)]
    pub fn timer1_in2_event_fall_en(&self) -> TIMER1_IN2_EVENT_FALL_EN_R {
        TIMER1_IN2_EVENT_FALL_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 14:17 - 0,13,14,15 = IN2 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN2"]
    #[inline(always)]
    pub fn timer1_gpio2_conf(&self) -> TIMER1_GPIO2_CONF_R {
        TIMER1_GPIO2_CONF_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
    #[inline(always)]
    pub fn timer1_in1_stamp_type(&self) -> TIMER1_IN1_STAMP_TYPE_R {
        TIMER1_IN1_STAMP_TYPE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Gives the number of periods +1 of IN1, in which module counts"]
    #[inline(always)]
    pub fn timer1_in1_period_max(&self) -> TIMER1_IN1_PERIOD_MAX_R {
        TIMER1_IN1_PERIOD_MAX_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN1_PERIOD_MAX 0 = Interrupt is masked"]
    #[inline(always)]
    pub fn timer1_in1_irq_en(&self) -> TIMER1_IN1_IRQ_EN_R {
        TIMER1_IN1_IRQ_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = Capture mode 1 = Count mode"]
    #[inline(always)]
    pub fn timer1_in1_count_en(&self) -> TIMER1_IN1_COUNT_EN_R {
        TIMER1_IN1_COUNT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO1_CONF=0 to prevent false events"]
    #[inline(always)]
    pub fn timer1_in1_event_fall_en(&self) -> TIMER1_IN1_EVENT_FALL_EN_R {
        TIMER1_IN1_EVENT_FALL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 0:3 - 0,13,14,15 = IN1 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN1"]
    #[inline(always)]
    pub fn timer1_gpio1_conf(&self) -> TIMER1_GPIO1_CONF_R {
        TIMER1_GPIO1_CONF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 27 - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
    #[inline(always)]
    pub fn timer1_in2_stamp_type(&mut self) -> TIMER1_IN2_STAMP_TYPE_W {
        TIMER1_IN2_STAMP_TYPE_W { w: self }
    }
    #[doc = "Bits 21:26 - Gives the number of periods +1 of IN2, in which module counts"]
    #[inline(always)]
    pub fn timer1_in2_period_max(&mut self) -> TIMER1_IN2_PERIOD_MAX_W {
        TIMER1_IN2_PERIOD_MAX_W { w: self }
    }
    #[doc = "Bit 20 - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN2_PERIOD_MAX 0 = Interrupt is masked"]
    #[inline(always)]
    pub fn timer1_in2_irq_en(&mut self) -> TIMER1_IN2_IRQ_EN_W {
        TIMER1_IN2_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 19 - 0 = Capture mode 1 = Count mode"]
    #[inline(always)]
    pub fn timer1_in2_count_en(&mut self) -> TIMER1_IN2_COUNT_EN_W {
        TIMER1_IN2_COUNT_EN_W { w: self }
    }
    #[doc = "Bit 18 - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO2_CONF=0 to prevent false events"]
    #[inline(always)]
    pub fn timer1_in2_event_fall_en(&mut self) -> TIMER1_IN2_EVENT_FALL_EN_W {
        TIMER1_IN2_EVENT_FALL_EN_W { w: self }
    }
    #[doc = "Bits 14:17 - 0,13,14,15 = IN2 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN2"]
    #[inline(always)]
    pub fn timer1_gpio2_conf(&mut self) -> TIMER1_GPIO2_CONF_W {
        TIMER1_GPIO2_CONF_W { w: self }
    }
    #[doc = "Bit 13 - 0 = On each event store the counter value 1 = On each event store the RTC time stamp"]
    #[inline(always)]
    pub fn timer1_in1_stamp_type(&mut self) -> TIMER1_IN1_STAMP_TYPE_W {
        TIMER1_IN1_STAMP_TYPE_W { w: self }
    }
    #[doc = "Bits 7:12 - Gives the number of periods +1 of IN1, in which module counts"]
    #[inline(always)]
    pub fn timer1_in1_period_max(&mut self) -> TIMER1_IN1_PERIOD_MAX_W {
        TIMER1_IN1_PERIOD_MAX_W { w: self }
    }
    #[doc = "Bit 6 - 1 = Interrupt is generated when capture is occurred or was counted TIMER1_IN1_PERIOD_MAX 0 = Interrupt is masked"]
    #[inline(always)]
    pub fn timer1_in1_irq_en(&mut self) -> TIMER1_IN1_IRQ_EN_W {
        TIMER1_IN1_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 5 - 0 = Capture mode 1 = Count mode"]
    #[inline(always)]
    pub fn timer1_in1_count_en(&mut self) -> TIMER1_IN1_COUNT_EN_W {
        TIMER1_IN1_COUNT_EN_W { w: self }
    }
    #[doc = "Bit 4 - 0 = Rising edge event 1 = Falling edge event it should be written when TIMER1_GPIO1_CONF=0 to prevent false events"]
    #[inline(always)]
    pub fn timer1_in1_event_fall_en(&mut self) -> TIMER1_IN1_EVENT_FALL_EN_W {
        TIMER1_IN1_EVENT_FALL_EN_W { w: self }
    }
    #[doc = "Bits 0:3 - 0,13,14,15 = IN1 is not used 1..12 = Defines the P0 pin (0..11) module will use as IN1"]
    #[inline(always)]
    pub fn timer1_gpio1_conf(&mut self) -> TIMER1_GPIO1_CONF_W {
        TIMER1_GPIO1_CONF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer1 Capture control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_capture_reg](index.html) module"]
pub struct TIMER1_CAPTURE_REG_SPEC;
impl crate::RegisterSpec for TIMER1_CAPTURE_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_capture_reg::R](R) reader structure"]
impl crate::Readable for TIMER1_CAPTURE_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1_capture_reg::W](W) writer structure"]
impl crate::Writable for TIMER1_CAPTURE_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1_CAPTURE_REG to value 0"]
impl crate::Resettable for TIMER1_CAPTURE_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
