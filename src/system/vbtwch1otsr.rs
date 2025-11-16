#[doc = "Register `VBTWCH1OTSR` reader"]
pub type R = crate::R<Vbtwch1otsrSpec>;
#[doc = "Register `VBTWCH1OTSR` writer"]
pub type W = crate::W<Vbtwch1otsrSpec>;
#[doc = "VBATWIO1 Output VBATWIO0 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1vch0te {
    #[doc = "0: VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled."]
    _1 = 1,
}
impl From<Ch1vch0te> for bool {
    #[inline(always)]
    fn from(variant: Ch1vch0te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1VCH0TE` reader - VBATWIO1 Output VBATWIO0 Trigger Enable"]
pub type Ch1vch0teR = crate::BitReader<Ch1vch0te>;
impl Ch1vch0teR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1vch0te {
        match self.bits {
            false => Ch1vch0te::_0,
            true => Ch1vch0te::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch1vch0te::_0
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch1vch0te::_1
    }
}
#[doc = "Field `CH1VCH0TE` writer - VBATWIO1 Output VBATWIO0 Trigger Enable"]
pub type Ch1vch0teW<'a, REG> = crate::BitWriter<'a, REG, Ch1vch0te>;
impl<'a, REG> Ch1vch0teW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vch0te::_0)
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vch0te::_1)
    }
}
#[doc = "VBATWIO1 Output VBATWIO2 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1vch2te {
    #[doc = "0: VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled."]
    _1 = 1,
}
impl From<Ch1vch2te> for bool {
    #[inline(always)]
    fn from(variant: Ch1vch2te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1VCH2TE` reader - VBATWIO1 Output VBATWIO2 Trigger Enable"]
pub type Ch1vch2teR = crate::BitReader<Ch1vch2te>;
impl Ch1vch2teR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1vch2te {
        match self.bits {
            false => Ch1vch2te::_0,
            true => Ch1vch2te::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch1vch2te::_0
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch1vch2te::_1
    }
}
#[doc = "Field `CH1VCH2TE` writer - VBATWIO1 Output VBATWIO2 Trigger Enable"]
pub type Ch1vch2teW<'a, REG> = crate::BitWriter<'a, REG, Ch1vch2te>;
impl<'a, REG> Ch1vch2teW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vch2te::_0)
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vch2te::_1)
    }
}
#[doc = "VBATWIO1 Output RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1vrtcte {
    #[doc = "0: VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled"]
    _1 = 1,
}
impl From<Ch1vrtcte> for bool {
    #[inline(always)]
    fn from(variant: Ch1vrtcte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1VRTCTE` reader - VBATWIO1 Output RTC Periodic Signal Enable"]
pub type Ch1vrtcteR = crate::BitReader<Ch1vrtcte>;
impl Ch1vrtcteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1vrtcte {
        match self.bits {
            false => Ch1vrtcte::_0,
            true => Ch1vrtcte::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch1vrtcte::_0
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch1vrtcte::_1
    }
}
#[doc = "Field `CH1VRTCTE` writer - VBATWIO1 Output RTC Periodic Signal Enable"]
pub type Ch1vrtcteW<'a, REG> = crate::BitWriter<'a, REG, Ch1vrtcte>;
impl<'a, REG> Ch1vrtcteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vrtcte::_0)
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC periodic signal is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vrtcte::_1)
    }
}
#[doc = "VBATWIO1 Output RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1vrtcate {
    #[doc = "0: VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<Ch1vrtcate> for bool {
    #[inline(always)]
    fn from(variant: Ch1vrtcate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1VRTCATE` reader - VBATWIO1 Output RTC Alarm Signal Enable"]
pub type Ch1vrtcateR = crate::BitReader<Ch1vrtcate>;
impl Ch1vrtcateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1vrtcate {
        match self.bits {
            false => Ch1vrtcate::_0,
            true => Ch1vrtcate::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch1vrtcate::_0
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch1vrtcate::_1
    }
}
#[doc = "Field `CH1VRTCATE` writer - VBATWIO1 Output RTC Alarm Signal Enable"]
pub type Ch1vrtcateW<'a, REG> = crate::BitWriter<'a, REG, Ch1vrtcate>;
impl<'a, REG> Ch1vrtcateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vrtcate::_0)
    }
    #[doc = "VBATT wakeup I/O 1 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1vrtcate::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO1 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch1vch0te(&self) -> Ch1vch0teR {
        Ch1vch0teR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO1 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch1vch2te(&self) -> Ch1vch2teR {
        Ch1vch2teR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATWIO1 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch1vrtcte(&self) -> Ch1vrtcteR {
        Ch1vrtcteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATWIO1 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch1vrtcate(&self) -> Ch1vrtcateR {
        Ch1vrtcateR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWCH1OTSR")
            .field("ch1vrtcate", &self.ch1vrtcate())
            .field("ch1vrtcte", &self.ch1vrtcte())
            .field("ch1vch2te", &self.ch1vch2te())
            .field("ch1vch0te", &self.ch1vch0te())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO1 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch1vch0te(&mut self) -> Ch1vch0teW<Vbtwch1otsrSpec> {
        Ch1vch0teW::new(self, 0)
    }
    #[doc = "Bit 2 - VBATWIO1 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch1vch2te(&mut self) -> Ch1vch2teW<Vbtwch1otsrSpec> {
        Ch1vch2teW::new(self, 2)
    }
    #[doc = "Bit 3 - VBATWIO1 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch1vrtcte(&mut self) -> Ch1vrtcteW<Vbtwch1otsrSpec> {
        Ch1vrtcteW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATWIO1 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch1vrtcate(&mut self) -> Ch1vrtcateW<Vbtwch1otsrSpec> {
        Ch1vrtcateW::new(self, 4)
    }
}
#[doc = "VBATT Wakeup I/O 1 Output Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwch1otsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch1otsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbtwch1otsrSpec;
impl crate::RegisterSpec for Vbtwch1otsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwch1otsr::R`](R) reader structure"]
impl crate::Readable for Vbtwch1otsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwch1otsr::W`](W) writer structure"]
impl crate::Writable for Vbtwch1otsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTWCH1OTSR to value 0"]
impl crate::Resettable for Vbtwch1otsrSpec {}
