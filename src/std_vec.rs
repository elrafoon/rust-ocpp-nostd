use core::ops::{Deref, DerefMut};

use validator::HasLen;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Vec<T, const N: usize = 0>(
    std::vec::Vec<T>
);

impl<T, const N: usize> Vec<T, N> {
    pub fn from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
        Self(std::vec::Vec::from_iter(iter))
    }
}

impl<T, const N: usize> Deref for Vec<T, N> {
    type Target = std::vec::Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> HasLen for Vec<T, N> {
    fn length(&self) -> u64 {
        self.0.length()
    }
}

impl<'a, T, const N: usize> HasLen for &'a Vec<T, N> {
    fn length(&self) -> u64 {
        self.0.length()
    }
}

impl<T, const N: usize> From<std::vec::Vec<T>> for Vec<T, N> {
    fn from(value: std::vec::Vec<T>) -> Self {
        Self::from_iter(value.into_iter())
    }
}

impl<C, T: minicbor::Encode<C>, const N: usize> minicbor::Encode<C> for Vec<T, N> {
    fn encode<W: minicbor::encode::Write>(&self, e: &mut minicbor::Encoder<W>, ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.array(self.len() as u64)?;
        for item in self.0.iter() {
            item.encode(e, ctx)?;
        }
        Ok(())
    }
}

impl<'b, C, T: minicbor::Decode<'b, C> + core::fmt::Debug, const N: usize> minicbor::Decode<'b, C> for Vec<T, N> {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut vec = std::vec::Vec::<T>::new();
        for item in d.array_iter_with::<C, T>(ctx)? {
            vec.push(item?)
        }

        Ok(Self(vec))
    }
}