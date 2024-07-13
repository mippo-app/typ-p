use crate::typ_p::value_of::ValueOf;
use crate::typ_p::Value;

impl<T: Into<ValueOf>> From<Vec<T>> for ValueOf {
    fn from(f: Vec<T>) -> Self {
        let av = f.into_iter().map(Into::into).collect();

        ValueOf::ArrayValue(av)
    }
}

impl<T: Into<ValueOf>> FromIterator<T> for ValueOf {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let av = iter.into_iter().map(Into::into).collect();
        ValueOf::ArrayValue(av)
    }
}

impl<K: Into<String>, V: Into<ValueOf>> FromIterator<(K, V)> for ValueOf {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        ValueOf::Object(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}
