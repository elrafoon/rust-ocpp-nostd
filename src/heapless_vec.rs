use core::ops::{Deref, DerefMut};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Vec<T, const N: usize>(
    heapless::Vec<T, N>
);

impl<T, const N: usize> Deref for Vec<T, N> {
    type Target = heapless::Vec<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        let mut vec = heapless::Vec::<T, N>::new();
        for item in d.array_iter_with::<C, T>(ctx)? {
            vec.push(item?).unwrap()
        }

        Ok(Self(vec))
    }
}