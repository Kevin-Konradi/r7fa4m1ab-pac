#[doc = "Register `SEMR` reader"]
pub type R = crate::R<SemrSpec>;
#[doc = "Register `SEMR` writer"]
pub type W = crate::W<SemrSpec>;
#[doc = "Bit Rate Modulation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brme {
    #[doc = "0: Bit rate modulation function is disabled."]
    _0 = 0,
    #[doc = "1: Bit rate modulation function is enabled."]
    _1 = 1,
}
impl From<Brme> for bool {
    #[inline(always)]
    fn from(variant: Brme) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRME` reader - Bit Rate Modulation Enable"]
pub type BrmeR = crate::BitReader<Brme>;
impl BrmeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brme {
        match self.bits {
            false => Brme::_0,
            true => Brme::_1,
        }
    }
    #[doc = "Bit rate modulation function is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Brme::_0
    }
    #[doc = "Bit rate modulation function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Brme::_1
    }
}
#[doc = "Field `BRME` writer - Bit Rate Modulation Enable"]
pub type BrmeW<'a, REG> = crate::BitWriter<'a, REG, Brme>;
impl<'a, REG> BrmeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit rate modulation function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Brme::_0)
    }
    #[doc = "Bit rate modulation function is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Brme::_1)
    }
}
#[doc = "Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abcse {
    #[doc = "0: Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    _0 = 0,
    #[doc = "1: Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    _1 = 1,
}
impl From<Abcse> for bool {
    #[inline(always)]
    fn from(variant: Abcse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABCSE` reader - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
pub type AbcseR = crate::BitReader<Abcse>;
impl AbcseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abcse {
        match self.bits {
            false => Abcse::_0,
            true => Abcse::_1,
        }
    }
    #[doc = "Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Abcse::_0
    }
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Abcse::_1
    }
}
#[doc = "Field `ABCSE` writer - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
pub type AbcseW<'a, REG> = crate::BitWriter<'a, REG, Abcse>;
impl<'a, REG> AbcseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock cycles for 1-bit period is decided with combination between BGDM and ABCS in SEMR."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Abcse::_0)
    }
    #[doc = "Baud rate is 6 base clock cycles for 1-bit period and the clock of a double frequency is output from the baud rate generator."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Abcse::_1)
    }
}
#[doc = "Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abcs {
    #[doc = "0: Selects 16 base clock cycles for 1-bit period."]
    _0 = 0,
    #[doc = "1: Selects 8 base clock cycles for 1-bit period."]
    _1 = 1,
}
impl From<Abcs> for bool {
    #[inline(always)]
    fn from(variant: Abcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABCS` reader - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
pub type AbcsR = crate::BitReader<Abcs>;
impl AbcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abcs {
        match self.bits {
            false => Abcs::_0,
            true => Abcs::_1,
        }
    }
    #[doc = "Selects 16 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Abcs::_0
    }
    #[doc = "Selects 8 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Abcs::_1
    }
}
#[doc = "Field `ABCS` writer - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
pub type AbcsW<'a, REG> = crate::BitWriter<'a, REG, Abcs>;
impl<'a, REG> AbcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects 16 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Abcs::_0)
    }
    #[doc = "Selects 8 base clock cycles for 1-bit period."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Abcs::_1)
    }
}
#[doc = "Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nfen {
    #[doc = "0: Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    _0 = 0,
    #[doc = "1: Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
    _1 = 1,
}
impl From<Nfen> for bool {
    #[inline(always)]
    fn from(variant: Nfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NFEN` reader - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
pub type NfenR = crate::BitReader<Nfen>;
impl NfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nfen {
        match self.bits {
            false => Nfen::_0,
            true => Nfen::_1,
        }
    }
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nfen::_0
    }
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nfen::_1
    }
}
#[doc = "Field `NFEN` writer - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG, Nfen>;
impl<'a, REG> NfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_0)
    }
    #[doc = "Noise cancellation function for the RXDn/SSCLn and SSDAn input signal is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nfen::_1)
    }
}
#[doc = "Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgdm {
    #[doc = "0: Baud rate generator outputs the clock with normal frequency."]
    _0 = 0,
    #[doc = "1: Baud rate generator outputs the clock with doubled frequency."]
    _1 = 1,
}
impl From<Bgdm> for bool {
    #[inline(always)]
    fn from(variant: Bgdm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGDM` reader - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
pub type BgdmR = crate::BitReader<Bgdm>;
impl BgdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgdm {
        match self.bits {
            false => Bgdm::_0,
            true => Bgdm::_1,
        }
    }
    #[doc = "Baud rate generator outputs the clock with normal frequency."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bgdm::_0
    }
    #[doc = "Baud rate generator outputs the clock with doubled frequency."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bgdm::_1
    }
}
#[doc = "Field `BGDM` writer - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
pub type BgdmW<'a, REG> = crate::BitWriter<'a, REG, Bgdm>;
impl<'a, REG> BgdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Baud rate generator outputs the clock with normal frequency."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgdm::_0)
    }
    #[doc = "Baud rate generator outputs the clock with doubled frequency."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgdm::_1)
    }
}
#[doc = "Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdesel {
    #[doc = "0: The low level on the RXDn pin is detected as the start bit."]
    _0 = 0,
    #[doc = "1: A falling edge on the RXDn pin is detected as the start bit."]
    _1 = 1,
}
impl From<Rxdesel> for bool {
    #[inline(always)]
    fn from(variant: Rxdesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDESEL` reader - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
pub type RxdeselR = crate::BitReader<Rxdesel>;
impl RxdeselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdesel {
        match self.bits {
            false => Rxdesel::_0,
            true => Rxdesel::_1,
        }
    }
    #[doc = "The low level on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxdesel::_0
    }
    #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxdesel::_1
    }
}
#[doc = "Field `RXDESEL` writer - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
pub type RxdeselW<'a, REG> = crate::BitWriter<'a, REG, Rxdesel>;
impl<'a, REG> RxdeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The low level on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdesel::_0)
    }
    #[doc = "A falling edge on the RXDn pin is detected as the start bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdesel::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(&self) -> BrmeR {
        BrmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[inline(always)]
    pub fn abcse(&self) -> AbcseR {
        AbcseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn abcs(&self) -> AbcsR {
        AbcsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
    #[inline(always)]
    pub fn bgdm(&self) -> BgdmR {
        BgdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn rxdesel(&self) -> RxdeselR {
        RxdeselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEMR")
            .field("rxdesel", &self.rxdesel())
            .field("bgdm", &self.bgdm())
            .field("nfen", &self.nfen())
            .field("abcs", &self.abcs())
            .field("abcse", &self.abcse())
            .field("brme", &self.brme())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Bit Rate Modulation Enable"]
    #[inline(always)]
    pub fn brme(&mut self) -> BrmeW<SemrSpec> {
        BrmeW::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronous Mode Extended Base Clock Select 1 (Valid only in asynchronous mode and SCR.CKE\\[1\\]=0)"]
    #[inline(always)]
    pub fn abcse(&mut self) -> AbcseW<SemrSpec> {
        AbcseW::new(self, 3)
    }
    #[doc = "Bit 4 - Asynchronous Mode Base Clock Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn abcs(&mut self) -> AbcsW<SemrSpec> {
        AbcsW::new(self, 4)
    }
    #[doc = "Bit 5 - Digital Noise Filter Function Enable (The NFEN bit should be 0 without simple I2C mode and asynchronous mode.) In asynchronous mode, for RXDn input only. In simple I2C mode, for RXDn/TxDn input."]
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<SemrSpec> {
        NfenW::new(self, 5)
    }
    #[doc = "Bit 6 - Baud Rate Generator Double-Speed Mode Select (Only valid the CKE\\[1\\] bit in SCR is 0 in asynchronous mode)."]
    #[inline(always)]
    pub fn bgdm(&mut self) -> BgdmW<SemrSpec> {
        BgdmW::new(self, 6)
    }
    #[doc = "Bit 7 - Asynchronous Start Bit Edge Detection Select (Valid only in asynchronous mode)"]
    #[inline(always)]
    pub fn rxdesel(&mut self) -> RxdeselW<SemrSpec> {
        RxdeselW::new(self, 7)
    }
}
#[doc = "Serial Extended Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`semr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SemrSpec;
impl crate::RegisterSpec for SemrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`semr::R`](R) reader structure"]
impl crate::Readable for SemrSpec {}
#[doc = "`write(|w| ..)` method takes [`semr::W`](W) writer structure"]
impl crate::Writable for SemrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEMR to value 0"]
impl crate::Resettable for SemrSpec {}
