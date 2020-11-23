use glib::prelude::*;

mod imp;

glib_wrapper! {
    pub struct OboeSink(ObjectSubclass<imp::OboeSink>) @extends gst_base::BaseSink, gst::Element, gst::Object;
}

unsafe impl Send for OboeSink {}
unsafe impl Sync for OboeSink {}

pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "oboesink",
        gst::Rank::None,
        OboeSink::static_type(),
    )
}
