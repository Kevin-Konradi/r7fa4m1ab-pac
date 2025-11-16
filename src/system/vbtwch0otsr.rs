#[doc = "Register `VBTWCH0OTSR` reader"]
pub type R = crate::R<Vbtwch0otsrSpec>;
#[doc = "Register `VBTWCH0OTSR` writer"]
pub type W = crate::W<Vbtwch0otsrSpec>;
#[doc = "VBATWIO0 Output VBATWIO1 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0vch1te {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
    _1 = 1,
}
impl From<Ch0vch1te> for bool {
    #[inline(always)]
    fn from(variant: Ch0vch1te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0VCH1TE` reader - VBATWIO0 Output VBATWIO1 Trigger Enable"]
pub type Ch0vch1teR = crate::BitReader<Ch0vch1te>;
impl Ch0vch1teR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0vch1te {
        match self.bits {
            false => Ch0vch1te::_0,
            true => Ch0vch1te::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch0vch1te::_0
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch0vch1te::_1
    }
}
#[doc = "Field `CH0VCH1TE` writer - VBATWIO0 Output VBATWIO1 Trigger Enable"]
pub type Ch0vch1teW<'a, REG> = crate::BitWriter<'a, REG, Ch0vch1te>;
impl<'a, REG> Ch0vch1teW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vch1te::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vch1te::_1)
    }
}
#[doc = "VBATWIO0 Output VBATWIO2 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0vch2te {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
    _1 = 1,
}
impl From<Ch0vch2te> for bool {
    #[inline(always)]
    fn from(variant: Ch0vch2te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0VCH2TE` reader - VBATWIO0 Output VBATWIO2 Trigger Enable"]
pub type Ch0vch2teR = crate::BitReader<Ch0vch2te>;
impl Ch0vch2teR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0vch2te {
        match self.bits {
            false => Ch0vch2te::_0,
            true => Ch0vch2te::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch0vch2te::_0
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch0vch2te::_1
    }
}
#[doc = "Field `CH0VCH2TE` writer - VBATWIO0 Output VBATWIO2 Trigger Enable"]
pub type Ch0vch2teW<'a, REG> = crate::BitWriter<'a, REG, Ch0vch2te>;
impl<'a, REG> Ch0vch2teW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vch2te::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the VBATWIO2 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vch2te::_1)
    }
}
#[doc = "VBATWIO0 Output RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0vrtcte {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
    _1 = 1,
}
impl From<Ch0vrtcte> for bool {
    #[inline(always)]
    fn from(variant: Ch0vrtcte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0VRTCTE` reader - VBATWIO0 Output RTC Periodic Signal Enable"]
pub type Ch0vrtcteR = crate::BitReader<Ch0vrtcte>;
impl Ch0vrtcteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0vrtcte {
        match self.bits {
            false => Ch0vrtcte::_0,
            true => Ch0vrtcte::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch0vrtcte::_0
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch0vrtcte::_1
    }
}
#[doc = "Field `CH0VRTCTE` writer - VBATWIO0 Output RTC Periodic Signal Enable"]
pub type Ch0vrtcteW<'a, REG> = crate::BitWriter<'a, REG, Ch0vrtcte>;
impl<'a, REG> Ch0vrtcteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vrtcte::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vrtcte::_1)
    }
}
#[doc = "VBATWIO0 Output RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0vrtcate {
    #[doc = "0: VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<Ch0vrtcate> for bool {
    #[inline(always)]
    fn from(variant: Ch0vrtcate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0VRTCATE` reader - VBATWIO0 Output RTC Alarm Signal Enable"]
pub type Ch0vrtcateR = crate::BitReader<Ch0vrtcate>;
impl Ch0vrtcateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0vrtcate {
        match self.bits {
            false => Ch0vrtcate::_0,
            true => Ch0vrtcate::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch0vrtcate::_0
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch0vrtcate::_1
    }
}
#[doc = "Field `CH0VRTCATE` writer - VBATWIO0 Output RTC Alarm Signal Enable"]
pub type Ch0vrtcateW<'a, REG> = crate::BitWriter<'a, REG, Ch0vrtcate>;
impl<'a, REG> Ch0vrtcateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vrtcate::_0)
    }
    #[doc = "VBATT wakeup I/O 0 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0vrtcate::_1)
    }
}
impl R {
    #[doc = "Bit 1 - VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch1te(&self) -> Ch0vch1teR {
        Ch0vch1teR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch2te(&self) -> Ch0vch2teR {
        Ch0vch2teR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATWIO0 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcte(&self) -> Ch0vrtcteR {
        Ch0vrtcteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATWIO0 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcate(&self) -> Ch0vrtcateR {
        Ch0vrtcateR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWCH0OTSR")
            .field("ch0vrtcate", &self.ch0vrtcate())
            .field("ch0vrtcte", &self.ch0vrtcte())
            .field("ch0vch2te", &self.ch0vch2te())
            .field("ch0vch1te", &self.ch0vch1te())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - VBATWIO0 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch1te(&mut self) -> Ch0vch1teW<Vbtwch0otsrSpec> {
        Ch0vch1teW::new(self, 1)
    }
    #[doc = "Bit 2 - VBATWIO0 Output VBATWIO2 Trigger Enable"]
    #[inline(always)]
    pub fn ch0vch2te(&mut self) -> Ch0vch2teW<Vbtwch0otsrSpec> {
        Ch0vch2teW::new(self, 2)
    }
    #[doc = "Bit 3 - VBATWIO0 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcte(&mut self) -> Ch0vrtcteW<Vbtwch0otsrSpec> {
        Ch0vrtcteW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATWIO0 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch0vrtcate(&mut self) -> Ch0vrtcateW<Vbtwch0otsrSpec> {
        Ch0vrtcateW::new(self, 4)
    }
}
#[doc = "VBATT Wakeup I/O 0 Output Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwch0otsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch0otsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbtwch0otsrSpec;
impl crate::RegisterSpec for Vbtwch0otsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwch0otsr::R`](R) reader structure"]
impl crate::Readable for Vbtwch0otsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwch0otsr::W`](W) writer structure"]
impl crate::Writable for Vbtwch0otsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTWCH0OTSR to value 0"]
impl crate::Resettable for Vbtwch0otsrSpec {}
