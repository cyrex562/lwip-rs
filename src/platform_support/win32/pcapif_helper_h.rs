// #define LWIP_PCAPIF_HELPER_H

// struct pcapifh_linkstate;

pub enum pcapifh_link_event {
    PCAPIF_LINKEVENT_UNKNOWN,
    PCAPIF_LINKEVENT_UP,
    PCAPIF_LINKEVENT_DOWN,
}

// struct pcapifh_linkstate* pcapifh_linkstate_init(adapter_name: &mut String);
// pcapifh_linkstate_get: pcapifh_link_event(struct pcapifh_linkstate* state);
// pub fn  pcapifh_linkstate_close(struct pcapifh_linkstate* state);
