#[doc = "Register `DEVADD%s` reader"]
pub type R = crate::R<DevaddSpec>;
#[doc = "Register `DEVADD%s` writer"]
pub type W = crate::W<DevaddSpec>;
#[doc = "Transfer Speed of Communication Target Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbspd {
    #[doc = "0: DEVADDn is not used"]
    _00 = 0,
    #[doc = "1: Low speed"]
    _01 = 1,
    #[doc = "2: Full speed"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<Usbspd> for u8 {
    #[inline(always)]
    fn from(variant: Usbspd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbspd {
    type Ux = u8;
}
impl crate::IsEnum for Usbspd {}
#[doc = "Field `USBSPD` reader - Transfer Speed of Communication Target Device"]
pub type UsbspdR = crate::FieldReader<Usbspd>;
impl UsbspdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbspd {
        match self.bits {
            0 => Usbspd::_00,
            1 => Usbspd::_01,
            2 => Usbspd::_10,
            3 => Usbspd::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "DEVADDn is not used"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Usbspd::_00
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Usbspd::_01
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Usbspd::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Usbspd::_11
    }
}
#[doc = "Field `USBSPD` writer - Transfer Speed of Communication Target Device"]
pub type UsbspdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbspd, crate::Safe>;
impl<'a, REG> UsbspdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DEVADDn is not used"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_00)
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_01)
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspd::_11)
    }
}
impl R {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(&self) -> UsbspdR {
        UsbspdR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVADD")
            .field("usbspd", &self.usbspd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(&mut self) -> UsbspdW<DevaddSpec> {
        UsbspdW::new(self, 6)
    }
}
#[doc = "Device Address %s Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaddSpec;
impl crate::RegisterSpec for DevaddSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`devadd::R`](R) reader structure"]
impl crate::Readable for DevaddSpec {}
#[doc = "`write(|w| ..)` method takes [`devadd::W`](W) writer structure"]
impl crate::Writable for DevaddSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVADD%s to value 0"]
impl crate::Resettable for DevaddSpec {}
