use crate::{ffi::DartCObject, IntoDart};

impl<T> IntoDart for chrono::DateTime<T>
where
    T: chrono::TimeZone,
{
    fn into_dart(self) -> DartCObject { self.timestamp_micros().into_dart() }
}

impl IntoDart for chrono::NaiveDateTime {
    fn into_dart(self) -> DartCObject { self.timestamp_micros().into_dart() }
}
