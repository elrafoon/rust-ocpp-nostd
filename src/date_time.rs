#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct DateTime(
    chrono::DateTime<chrono::Utc>
);

impl DateTime {
    pub fn from_chrono(date_time: chrono::DateTime<chrono::Utc>) -> Self {
        Self(date_time)
    }
}

impl From<chrono::DateTime<chrono::Utc>> for DateTime {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self::from_chrono(value)
    }
}

impl<C> minicbor::Encode<C> for DateTime {
    fn encode<W: minicbor::encode::Write>(&self, e: &mut minicbor::Encoder<W>, _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.i64(self.0.timestamp_micros())?;
        Ok(())
    }
}

impl<'b,C> minicbor::Decode<'b,C> for DateTime {
    fn decode(d: &mut minicbor::Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let t =
            chrono::NaiveDateTime::from_timestamp_micros(d.i64()?)
            .ok_or(minicbor::decode::Error::type_mismatch(minicbor::data::Type::I64))?
            .and_utc();

        Ok(DateTime(t))
    }
}