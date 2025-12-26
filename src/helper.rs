use cidre::cg::{EventSrc, EventSrcStateId};

pub fn fix_cursor() {
    let state_id = EventSrcStateId::CombinedSession;
    let mut event_source_ref = EventSrc::with_state(state_id);
    if let Some(ref mut retained) = event_source_ref {
        EventSrc::set_local_events_suppression_interval(retained, 0.0);
    }
}
