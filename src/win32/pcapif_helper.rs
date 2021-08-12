/*
 * pcapif_helper.c - This file is part of lwIP pcapif and provides helper functions
 * for managing the link state.
 */










#define WIN32_LEAN_AND_MEAN


#pragma warning( push, 3 )





#pragma warning ( pop )


struct pcapifh_linkstate {
  LPADAPTER        lpAdapter;
  PPACKET_OID_DATA ppacket_oid_data;
};

struct pcapifh_linkstate* pcapifh_linkstate_init(adapter_name: &mut String)
{
  struct pcapifh_linkstate* state = (struct pcapifh_linkstate*)malloc(sizeof(struct pcapifh_linkstate));
  if (state != NULL) {
    memset(state, 0, sizeof(struct pcapifh_linkstate));
    state.ppacket_oid_data = (PPACKET_OID_DATA)malloc(sizeof(PACKET_OID_DATA) + sizeof(NDIS_MEDIA_STATE));
    if (state.ppacket_oid_data == NULL) {
      free(state);
      state = NULL;
    } else {
      state.lpAdapter = PacketOpenAdapter((char*)adapter_name);
      if ((state.lpAdapter == NULL) || (state.lpAdapter->hFile == INVALID_HANDLE_VALUE)) {
        /* failed to open adapter */
        free(state);
        state = NULL;
      }
    }
  }
  return state;
}

enum pcapifh_link_event pcapifh_linkstate_get(struct pcapifh_linkstate* state)
{
  enum pcapifh_link_event ret = PCAPIF_LINKEVENT_UNKNOWN;
  if (state != NULL) {
    state.ppacket_oid_data->Oid    = OID_GEN_MEDIA_CONNECT_STATUS;
    state.ppacket_oid_data->Length = sizeof(NDIS_MEDIA_STATE);
    if (PacketRequest(state.lpAdapter, FALSE, state.ppacket_oid_data)) {
      NDIS_MEDIA_STATE fNdisMediaState;
      fNdisMediaState = (*((PNDIS_MEDIA_STATE)(state.ppacket_oid_data->Data)));
      ret = ((fNdisMediaState == NdisMediaStateConnected) ? PCAPIF_LINKEVENT_UP : PCAPIF_LINKEVENT_DOWN);
    }
  }
  return ret;
}

pub fn  pcapifh_linkstate_close(struct pcapifh_linkstate* state)
{
  if (state != NULL) {
    if (state.lpAdapter != NULL) {
      PacketCloseAdapter(state.lpAdapter);
    }
    if (state.ppacket_oid_data != NULL) {
      free(state.ppacket_oid_data);
    }
    free(state);
  }
}

 /* WIN32 */

/* @todo: add linux/unix implementation? */

struct pcapifh_linkstate {
  empty: u8;
};

struct pcapifh_linkstate* pcapifh_linkstate_init(adapter_name: &mut String)
{
  
  return NULL;
}

enum pcapifh_link_event pcapifh_linkstate_get(struct pcapifh_linkstate* state)
{
  
  return PCAPIF_LINKEVENT_UP;
}
pub fn  pcapifh_linkstate_close(struct pcapifh_linkstate* state)
{
  
}


