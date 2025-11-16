#[doc = "Register `LCDM1` reader"]
pub type R = crate::R<Lcdm1Spec>;
#[doc = "Register `LCDM1` writer"]
pub type W = crate::W<Lcdm1Spec>;
#[doc = "Voltage Boosting Pin Initial Value Switching Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdvlm {
    #[doc = "0: Set when VDD >= 2.7 V"]
    _0 = 0,
    #[doc = "1: Set when VDD <= 4.2 V"]
    _1 = 1,
}
impl From<Lcdvlm> for bool {
    #[inline(always)]
    fn from(variant: Lcdvlm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDVLM` reader - Voltage Boosting Pin Initial Value Switching Control"]
pub type LcdvlmR = crate::BitReader<Lcdvlm>;
impl LcdvlmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdvlm {
        match self.bits {
            false => Lcdvlm::_0,
            true => Lcdvlm::_1,
        }
    }
    #[doc = "Set when VDD >= 2.7 V"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lcdvlm::_0
    }
    #[doc = "Set when VDD <= 4.2 V"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcdvlm::_1
    }
}
#[doc = "Field `LCDVLM` writer - Voltage Boosting Pin Initial Value Switching Control"]
pub type LcdvlmW<'a, REG> = crate::BitWriter<'a, REG, Lcdvlm>;
impl<'a, REG> LcdvlmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set when VDD >= 2.7 V"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdvlm::_0)
    }
    #[doc = "Set when VDD <= 4.2 V"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdvlm::_1)
    }
}
#[doc = "Display data area control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdsel {
    #[doc = "0: Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    _0 = 0,
    #[doc = "1: Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    _1 = 1,
}
impl From<Lcdsel> for bool {
    #[inline(always)]
    fn from(variant: Lcdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSEL` reader - Display data area control"]
pub type LcdselR = crate::BitReader<Lcdsel>;
impl LcdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdsel {
        match self.bits {
            false => Lcdsel::_0,
            true => Lcdsel::_1,
        }
    }
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lcdsel::_0
    }
    #[doc = "Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcdsel::_1
    }
}
#[doc = "Field `LCDSEL` writer - Display data area control"]
pub type LcdselW<'a, REG> = crate::BitWriter<'a, REG, Lcdsel>;
impl<'a, REG> LcdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdsel::_0)
    }
    #[doc = "Displaying a B-pattern area data (higher four bits of LCD display data register)(BLON=0)/Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))(BLON=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdsel::_1)
    }
}
#[doc = "Display data area control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blon {
    #[doc = "0: Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    _0 = 0,
    #[doc = "1: Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    _1 = 1,
}
impl From<Blon> for bool {
    #[inline(always)]
    fn from(variant: Blon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLON` reader - Display data area control"]
pub type BlonR = crate::BitReader<Blon>;
impl BlonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blon {
        match self.bits {
            false => Blon::_0,
            true => Blon::_1,
        }
    }
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Blon::_0
    }
    #[doc = "Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Blon::_1
    }
}
#[doc = "Field `BLON` writer - Display data area control"]
pub type BlonW<'a, REG> = crate::BitWriter<'a, REG, Blon>;
impl<'a, REG> BlonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Displaying an A-pattern area data (lower four bits of LCD display data register)(LCDSEL=0)/Displaying a B-pattern area data (higher four bits of LCD display data register)(LCDSEL=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Blon::_0)
    }
    #[doc = "Alternately displaying A-pattern and B-pattern area data (blinking display corresponding to the constant-period interrupt (INTRTC) timing of the real-time clock (RTC))"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Blon::_1)
    }
}
#[doc = "Voltage boost circuit or capacitor split circuit operation enable/disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vlcon {
    #[doc = "0: Stops voltage boost circuit or capacitor split circuit operation"]
    _0 = 0,
    #[doc = "1: Enables voltage boost circuit or capacitor split circuit operation"]
    _1 = 1,
}
impl From<Vlcon> for bool {
    #[inline(always)]
    fn from(variant: Vlcon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLCON` reader - Voltage boost circuit or capacitor split circuit operation enable/disable"]
pub type VlconR = crate::BitReader<Vlcon>;
impl VlconR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vlcon {
        match self.bits {
            false => Vlcon::_0,
            true => Vlcon::_1,
        }
    }
    #[doc = "Stops voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Vlcon::_0
    }
    #[doc = "Enables voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Vlcon::_1
    }
}
#[doc = "Field `VLCON` writer - Voltage boost circuit or capacitor split circuit operation enable/disable"]
pub type VlconW<'a, REG> = crate::BitWriter<'a, REG, Vlcon>;
impl<'a, REG> VlconW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcon::_0)
    }
    #[doc = "Enables voltage boost circuit or capacitor split circuit operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Vlcon::_1)
    }
}
#[doc = "LCD Display Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scoc {
    #[doc = "0: Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    _0 = 0,
    #[doc = "1: Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    _1 = 1,
}
impl From<Scoc> for bool {
    #[inline(always)]
    fn from(variant: Scoc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCOC` reader - LCD Display Enable/Disable"]
pub type ScocR = crate::BitReader<Scoc>;
impl ScocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scoc {
        match self.bits {
            false => Scoc::_0,
            true => Scoc::_1,
        }
    }
    #[doc = "Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Scoc::_0
    }
    #[doc = "Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Scoc::_1
    }
}
#[doc = "Field `SCOC` writer - LCD Display Enable/Disable"]
pub type ScocW<'a, REG> = crate::BitWriter<'a, REG, Scoc>;
impl<'a, REG> ScocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output ground level to segment/common pin(LCDON=0)/Output ground level to segment/common pin(LCDON=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Scoc::_0)
    }
    #[doc = "Display off (all segment outputs are deselected)(LCDON=0)/Display on(LCDON=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Scoc::_1)
    }
}
#[doc = "LCD Display Enable/Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdon {
    #[doc = "0: Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    _0 = 0,
    #[doc = "1: Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    _1 = 1,
}
impl From<Lcdon> for bool {
    #[inline(always)]
    fn from(variant: Lcdon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDON` reader - LCD Display Enable/Disable"]
pub type LcdonR = crate::BitReader<Lcdon>;
impl LcdonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdon {
        match self.bits {
            false => Lcdon::_0,
            true => Lcdon::_1,
        }
    }
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lcdon::_0
    }
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lcdon::_1
    }
}
#[doc = "Field `LCDON` writer - LCD Display Enable/Disable"]
pub type LcdonW<'a, REG> = crate::BitWriter<'a, REG, Lcdon>;
impl<'a, REG> LcdonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display off (all segment outputs are deselected)(SCOC=1)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdon::_0)
    }
    #[doc = "Output ground level to segment/common pin(SCOC=0)/Display on(SCOC=1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdon::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    pub fn lcdvlm(&self) -> LcdvlmR {
        LcdvlmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Display data area control"]
    #[inline(always)]
    pub fn lcdsel(&self) -> LcdselR {
        LcdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Display data area control"]
    #[inline(always)]
    pub fn blon(&self) -> BlonR {
        BlonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[inline(always)]
    pub fn vlcon(&self) -> VlconR {
        VlconR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn scoc(&self) -> ScocR {
        ScocR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn lcdon(&self) -> LcdonR {
        LcdonR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCDM1")
            .field("lcdon", &self.lcdon())
            .field("scoc", &self.scoc())
            .field("vlcon", &self.vlcon())
            .field("blon", &self.blon())
            .field("lcdsel", &self.lcdsel())
            .field("lcdvlm", &self.lcdvlm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Boosting Pin Initial Value Switching Control"]
    #[inline(always)]
    pub fn lcdvlm(&mut self) -> LcdvlmW<Lcdm1Spec> {
        LcdvlmW::new(self, 0)
    }
    #[doc = "Bit 3 - Display data area control"]
    #[inline(always)]
    pub fn lcdsel(&mut self) -> LcdselW<Lcdm1Spec> {
        LcdselW::new(self, 3)
    }
    #[doc = "Bit 4 - Display data area control"]
    #[inline(always)]
    pub fn blon(&mut self) -> BlonW<Lcdm1Spec> {
        BlonW::new(self, 4)
    }
    #[doc = "Bit 5 - Voltage boost circuit or capacitor split circuit operation enable/disable"]
    #[inline(always)]
    pub fn vlcon(&mut self) -> VlconW<Lcdm1Spec> {
        VlconW::new(self, 5)
    }
    #[doc = "Bit 6 - LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn scoc(&mut self) -> ScocW<Lcdm1Spec> {
        ScocW::new(self, 6)
    }
    #[doc = "Bit 7 - LCD Display Enable/Disable"]
    #[inline(always)]
    pub fn lcdon(&mut self) -> LcdonW<Lcdm1Spec> {
        LcdonW::new(self, 7)
    }
}
#[doc = "LCD Mode Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdm1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdm1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lcdm1Spec;
impl crate::RegisterSpec for Lcdm1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcdm1::R`](R) reader structure"]
impl crate::Readable for Lcdm1Spec {}
#[doc = "`write(|w| ..)` method takes [`lcdm1::W`](W) writer structure"]
impl crate::Writable for Lcdm1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCDM1 to value 0"]
impl crate::Resettable for Lcdm1Spec {}
