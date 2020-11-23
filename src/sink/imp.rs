use glib::subclass;
use glib::subclass::prelude::*;

use gst::prelude::*;
use gst::subclass::prelude::*;
use gst::{QueryView, StateChange, StateChangeError, StateChangeSuccess};
use gst_base::prelude::BaseSinkExtManual;
use gst_base::subclass::prelude::*;

use once_cell::sync::Lazy;

pub struct OboeSink {}

unsafe impl Send for OboeSink {}
unsafe impl Sync for OboeSink {}

static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new("oboesink", gst::DebugColorFlags::empty(), Some("Oboe Sink"))
});

// Bytes per frame, 2 bytes for S16
const BPF: usize = 2;

impl OboeSink {}

impl ObjectSubclass for OboeSink {
    const NAME: &'static str = "OboeSink";
    type Type = super::OboeSink;
    type ParentType = gst_base::BaseSink;
    type Instance = gst::subclass::ElementInstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib_object_subclass!();

    fn new() -> Self {
        Self {}
    }

    fn class_init(klass: &mut Self::Class) {
        klass.set_metadata(
            "Oboe Sink",
            "Sink/Audio",
            "Ouptut sound using Android Oboe Library",
            "Sirius Wu <ccwu660601@mapacode.tw",
        );

        let caps = gst::Caps::new_simple(
            "audio/x-raw",
            &[
                (
                    "format",
                    &gst::List::new(&[&gst_audio::AUDIO_FORMAT_S16.to_str()]),
                ),
                ("layout", &"interleaved"),
                ("rate", &gst::IntRange::<i32>::new(1, i32::MAX)),
                ("channels", &1),
            ],
        );
        let sink_pad_template = gst::PadTemplate::new(
            "sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &caps,
        )
        .unwrap();
        klass.add_pad_template(sink_pad_template);
    }
}

impl ObjectImpl for OboeSink {}
impl ElementImpl for OboeSink {}
impl BaseSinkImpl for OboeSink {
    fn start(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        // TODO
        self.parent_start(element)
    }
    fn stop(&self, element: &Self::Type) -> Result<(), gst::ErrorMessage> {
        // TODO
        self.parent_stop(element)
    }
    fn render(
        &self,
        element: &Self::Type,
        buffer: &gst::Buffer,
    ) -> Result<gst::FlowSuccess, gst::FlowError> {
        // TODO
        let size = buffer.get_size();
        if size % BPF != 0 {
            gst_debug!(
                CAT,
                obj: element,
                "wrong size, buffer size: {}, bpf: {}",
                size,
                BPF
            );
            return Err(gst::FlowError::Error);
        }
        let samples = size / BPF;
        let pts = buffer.get_pts();
        let start = self.get_segment().get_start();
        self.parent_render(element, buffer)
    }
}
