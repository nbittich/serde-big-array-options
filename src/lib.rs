// Credit: https://github.com/rust-lang/rust/issues/54542#issuecomment-425238990

use std::marker::PhantomData;

use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};
pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
    data: &[Option<T>; N],
    ser: S,
) -> Result<S::Ok, S::Error> {
    let mut s = ser.serialize_tuple(N)?;
    for item in data {
        s.serialize_element(item)?;
    }
    s.end()
}

struct ArrayVisitor<T, const N: usize>(PhantomData<Option<T>>);

impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<Option<T>, N>
where
    T: Deserialize<'de> + Copy,
{
    type Value = [Option<T>; N];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an array of length {}", N))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut buf = [None; N];
        for i in 0..N {
            match (seq.next_element())? {
                v @ Some(_) => buf[i] = v,
                None => break,
            }
        }
        Ok(buf)
    }
}
pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[Option<T>; N], D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Copy,
{
    deserializer.deserialize_tuple(N, ArrayVisitor::<Option<T>, N>(PhantomData))
}
