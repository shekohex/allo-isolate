use crate::{ffi::DartCObject, IntoDart};

impl IntoDart for chrono::DateTime<chrono::Utc> {
    fn into_dart(self) -> DartCObject {
        self.timestamp_micros().into_dart()
    }
}
