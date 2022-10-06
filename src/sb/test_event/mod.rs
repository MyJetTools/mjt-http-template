{% if is_use_sb == "subscriber" or is_use_sb == "both" %}mod test_event_listener;{% endif %}
mod test_event_sb_contract;

{% if is_use_sb == "subscriber" or is_use_sb == "both" %}pub use test_event_listener::*;{% endif %}
pub use test_event_sb_contract::*;