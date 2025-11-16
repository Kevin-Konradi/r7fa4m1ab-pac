#[doc = "Register `VBTWCH2OTSR` reader"]
pub type R = crate::R<Vbtwch2otsrSpec>;
#[doc = "Register `VBTWCH2OTSR` writer"]
pub type W = crate::W<Vbtwch2otsrSpec>;
#[doc = "VBATWIO2 Output VBATWIO0 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2vch0te {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    _1 = 1,
}
impl From<Ch2vch0te> for bool {
    #[inline(always)]
    fn from(variant: Ch2vch0te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2VCH0TE` reader - VBATWIO2 Output VBATWIO0 Trigger Enable"]
pub type Ch2vch0teR = crate::BitReader<Ch2vch0te>;
impl Ch2vch0teR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2vch0te {
        match self.bits {
            false => Ch2vch0te::_0,
            true => Ch2vch0te::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch2vch0te::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch2vch0te::_1
    }
}
#[doc = "Field `CH2VCH0TE` writer - VBATWIO2 Output VBATWIO0 Trigger Enable"]
pub type Ch2vch0teW<'a, REG> = crate::BitWriter<'a, REG, Ch2vch0te>;
impl<'a, REG> Ch2vch0teW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vch0te::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO0 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vch0te::_1)
    }
}
#[doc = "VBATWIO2 Output VBATWIO1 Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2vch1te {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    _1 = 1,
}
impl From<Ch2vch1te> for bool {
    #[inline(always)]
    fn from(variant: Ch2vch1te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2VCH1TE` reader - VBATWIO2 Output VBATWIO1 Trigger Enable"]
pub type Ch2vch1teR = crate::BitReader<Ch2vch1te>;
impl Ch2vch1teR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2vch1te {
        match self.bits {
            false => Ch2vch1te::_0,
            true => Ch2vch1te::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch2vch1te::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch2vch1te::_1
    }
}
#[doc = "Field `CH2VCH1TE` writer - VBATWIO2 Output VBATWIO1 Trigger Enable"]
pub type Ch2vch1teW<'a, REG> = crate::BitWriter<'a, REG, Ch2vch1te>;
impl<'a, REG> Ch2vch1teW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vch1te::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the VBATWIO1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vch1te::_1)
    }
}
#[doc = "VBATWIO2 Output RTC Periodic Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2vrtcte {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    _1 = 1,
}
impl From<Ch2vrtcte> for bool {
    #[inline(always)]
    fn from(variant: Ch2vrtcte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2VRTCTE` reader - VBATWIO2 Output RTC Periodic Signal Enable"]
pub type Ch2vrtcteR = crate::BitReader<Ch2vrtcte>;
impl Ch2vrtcteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2vrtcte {
        match self.bits {
            false => Ch2vrtcte::_0,
            true => Ch2vrtcte::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch2vrtcte::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch2vrtcte::_1
    }
}
#[doc = "Field `CH2VRTCTE` writer - VBATWIO2 Output RTC Periodic Signal Enable"]
pub type Ch2vrtcteW<'a, REG> = crate::BitWriter<'a, REG, Ch2vrtcte>;
impl<'a, REG> Ch2vrtcteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vrtcte::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC periodic signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vrtcte::_1)
    }
}
#[doc = "VBATWIO2 Output RTC Alarm Signal Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2vrtcate {
    #[doc = "0: VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    _0 = 0,
    #[doc = "1: VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    _1 = 1,
}
impl From<Ch2vrtcate> for bool {
    #[inline(always)]
    fn from(variant: Ch2vrtcate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2VRTCATE` reader - VBATWIO2 Output RTC Alarm Signal Enable"]
pub type Ch2vrtcateR = crate::BitReader<Ch2vrtcate>;
impl Ch2vrtcateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2vrtcate {
        match self.bits {
            false => Ch2vrtcate::_0,
            true => Ch2vrtcate::_1,
        }
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ch2vrtcate::_0
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ch2vrtcate::_1
    }
}
#[doc = "Field `CH2VRTCATE` writer - VBATWIO2 Output RTC Alarm Signal Enable"]
pub type Ch2vrtcateW<'a, REG> = crate::BitWriter<'a, REG, Ch2vrtcate>;
impl<'a, REG> Ch2vrtcateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vrtcate::_0)
    }
    #[doc = "VBATT wakeup I/O 2 output trigger by the RTC alarm signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2vrtcate::_1)
    }
}
impl R {
    #[doc = "Bit 0 - VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch0te(&self) -> Ch2vch0teR {
        Ch2vch0teR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch1te(&self) -> Ch2vch1teR {
        Ch2vch1teR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - VBATWIO2 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcte(&self) -> Ch2vrtcteR {
        Ch2vrtcteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VBATWIO2 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcate(&self) -> Ch2vrtcateR {
        Ch2vrtcateR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VBTWCH2OTSR")
            .field("ch2vrtcate", &self.ch2vrtcate())
            .field("ch2vrtcte", &self.ch2vrtcte())
            .field("ch2vch1te", &self.ch2vch1te())
            .field("ch2vch0te", &self.ch2vch0te())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VBATWIO2 Output VBATWIO0 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch0te(&mut self) -> Ch2vch0teW<Vbtwch2otsrSpec> {
        Ch2vch0teW::new(self, 0)
    }
    #[doc = "Bit 1 - VBATWIO2 Output VBATWIO1 Trigger Enable"]
    #[inline(always)]
    pub fn ch2vch1te(&mut self) -> Ch2vch1teW<Vbtwch2otsrSpec> {
        Ch2vch1teW::new(self, 1)
    }
    #[doc = "Bit 3 - VBATWIO2 Output RTC Periodic Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcte(&mut self) -> Ch2vrtcteW<Vbtwch2otsrSpec> {
        Ch2vrtcteW::new(self, 3)
    }
    #[doc = "Bit 4 - VBATWIO2 Output RTC Alarm Signal Enable"]
    #[inline(always)]
    pub fn ch2vrtcate(&mut self) -> Ch2vrtcateW<Vbtwch2otsrSpec> {
        Ch2vrtcateW::new(self, 4)
    }
}
#[doc = "VBATT Wakeup I/O 2 Output Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vbtwch2otsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtwch2otsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vbtwch2otsrSpec;
impl crate::RegisterSpec for Vbtwch2otsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`vbtwch2otsr::R`](R) reader structure"]
impl crate::Readable for Vbtwch2otsrSpec {}
#[doc = "`write(|w| ..)` method takes [`vbtwch2otsr::W`](W) writer structure"]
impl crate::Writable for Vbtwch2otsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VBTWCH2OTSR to value 0"]
impl crate::Resettable for Vbtwch2otsrSpec {}
