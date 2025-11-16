#[doc = "Register `SMPUCTL` reader"]
pub type R = crate::R<SmpuctlSpec>;
#[doc = "Register `SMPUCTL` writer"]
pub type W = crate::W<SmpuctlSpec>;
#[doc = "Master Group enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oad {
    #[doc = "0: Non-maskable interrupt."]
    _0 = 0,
    #[doc = "1: Internal reset."]
    _1 = 1,
}
impl From<Oad> for bool {
    #[inline(always)]
    fn from(variant: Oad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OAD` reader - Master Group enable"]
pub type OadR = crate::BitReader<Oad>;
impl OadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oad {
        match self.bits {
            false => Oad::_0,
            true => Oad::_1,
        }
    }
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Oad::_0
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Oad::_1
    }
}
#[doc = "Field `OAD` writer - Master Group enable"]
pub type OadW<'a, REG> = crate::BitWriter<'a, REG, Oad>;
impl<'a, REG> OadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_0)
    }
    #[doc = "Internal reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Oad::_1)
    }
}
#[doc = "Protection of register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: All Bus Slave register writing is possible."]
    _0 = 0,
    #[doc = "1: All Bus Slave register writing is protected. Read is possible."]
    _1 = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTECT` reader - Protection of register"]
pub type ProtectR = crate::BitReader<Protect>;
impl ProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Protect {
        match self.bits {
            false => Protect::_0,
            true => Protect::_1,
        }
    }
    #[doc = "All Bus Slave register writing is possible."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Protect::_0
    }
    #[doc = "All Bus Slave register writing is protected. Read is possible."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Protect::_1
    }
}
#[doc = "Field `PROTECT` writer - Protection of register"]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All Bus Slave register writing is possible."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_0)
    }
    #[doc = "All Bus Slave register writing is protected. Read is possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_1)
    }
}
#[doc = "Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
    _0xA5 = 165,
    #[doc = "0: Writing to the PROTECT and OAD bit is invalid."]
    Others = 0,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Key {
        match self.bits {
            165 => Key::_0xA5,
            _ => Key::Others,
        }
    }
    #[doc = "Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn is_0x_a5(&self) -> bool {
        *self == Key::_0xA5
    }
    #[doc = "Writing to the PROTECT and OAD bit is invalid."]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Key::Others)
    }
}
#[doc = "Field `KEY` writer - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the PROTECT and OAD bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(Key::_0xA5)
    }
    #[doc = "Writing to the PROTECT and OAD bit is invalid."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn oad(&self) -> OadR {
        OadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protection of register"]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMPUCTL")
            .field("key", &self.key())
            .field("protect", &self.protect())
            .field("oad", &self.oad())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Master Group enable"]
    #[inline(always)]
    pub fn oad(&mut self) -> OadW<SmpuctlSpec> {
        OadW::new(self, 0)
    }
    #[doc = "Bit 1 - Protection of register"]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<SmpuctlSpec> {
        ProtectW::new(self, 1)
    }
    #[doc = "Bits 8:15 - Key Code This bit is used to enable or disable rewriting of the PROTECT and OAD bit."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<SmpuctlSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Slave MPU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smpuctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpuctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmpuctlSpec;
impl crate::RegisterSpec for SmpuctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`smpuctl::R`](R) reader structure"]
impl crate::Readable for SmpuctlSpec {}
#[doc = "`write(|w| ..)` method takes [`smpuctl::W`](W) writer structure"]
impl crate::Writable for SmpuctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMPUCTL to value 0"]
impl crate::Resettable for SmpuctlSpec {}
