#[doc = "Register `PSPMPUPT` reader"]
pub type R = crate::R<PspmpuptSpec>;
#[doc = "Register `PSPMPUPT` writer"]
pub type W = crate::W<PspmpuptSpec>;
#[doc = "Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)\n\nValue on reset: 0"]
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
#[doc = "Field `PROTECT` reader - Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)"]
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
#[doc = "Field `PROTECT` writer - Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)"]
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
#[doc = "Key Code The data written to these bits are not stored.\n\nValue on reset: 0"]
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
#[doc = "Field `KEY` writer - Key Code The data written to these bits are not stored."]
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
    #[doc = "Bit 0 - Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)"]
    #[inline(always)]
    pub fn protect(&self) -> ProtectR {
        ProtectR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSPMPUPT")
            .field("protect", &self.protect())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Protection register (PSPMPUAC, PSPMPUSA and PSPMPUSE)"]
    #[inline(always)]
    pub fn protect(&mut self) -> ProtectW<PspmpuptSpec> {
        ProtectW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Key Code The data written to these bits are not stored."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<PspmpuptSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Stack Pointer Monitor Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pspmpupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pspmpupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PspmpuptSpec;
impl crate::RegisterSpec for PspmpuptSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pspmpupt::R`](R) reader structure"]
impl crate::Readable for PspmpuptSpec {}
#[doc = "`write(|w| ..)` method takes [`pspmpupt::W`](W) writer structure"]
impl crate::Writable for PspmpuptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSPMPUPT to value 0"]
impl crate::Resettable for PspmpuptSpec {}
