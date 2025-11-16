#[doc = "Register `MSPMPUPT` reader"]
pub type R = crate::R<MspmpuptSpec>;
#[doc = "Register `MSPMPUPT` writer"]
pub type W = crate::W<MspmpuptSpec>;
#[doc = "Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Protect {
    #[doc = "0: Stack Pointer Monitor register writing is possible."]
    _0 = 0,
    #[doc = "1: Stack Pointer Monitor register writing is protected."]
    _1 = 1,
}
impl From<Protect> for bool {
    #[inline(always)]
    fn from(variant: Protect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTECT` reader - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
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
    #[doc = "Stack Pointer Monitor register writing is possible."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Protect::_0
    }
    #[doc = "Stack Pointer Monitor register writing is protected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Protect::_1
    }
}
#[doc = "Field `PROTECT` writer - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
pub type ProtectW<'a, REG> = crate::BitWriter<'a, REG, Protect>;
impl<'a, REG> ProtectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stack Pointer Monitor register writing is possible."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_0)
    }
    #[doc = "Stack Pointer Monitor register writing is protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Protect::_1)
    }
}
#[doc = "Write Keyword The data written to these bits are not stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    _0xA5 = 165,
    #[doc = "0: Writing to the PROTECT bit is invalid."]
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
#[doc = "Field `KEY` writer - Write Keyword The data written to these bits are not stored."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key, crate::Safe>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the PROTECT bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(Key::_0xA5)
    }
    #[doc = "Writing to the PROTECT bit is invalid."]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Others)
    }
}
impl R {
    #[doc = "Bit 0 - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPMPUPT")
            .field("protect", &self.protect())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Protection of register (MSPMPUAC, MSPMPUSA and MSPMPUSE)"]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<MspmpuptSpec> {
        ProtectW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MspmpuptSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mspmpupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mspmpupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MspmpuptSpec;
impl crate::RegisterSpec for MspmpuptSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mspmpupt::R`](R) reader structure"]
impl crate::Readable for MspmpuptSpec {}
#[doc = "`write(|w| ..)` method takes [`mspmpupt::W`](W) writer structure"]
impl crate::Writable for MspmpuptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSPMPUPT to value 0"]
impl crate::Resettable for MspmpuptSpec {}
