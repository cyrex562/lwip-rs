pub struct MacFilterOps {
    pub add_igmp_mac_filter: Option<add_igmp_mac_filter_fn>,
    pub del_igmp_mac_filter: Option<del_igmp_mac_filter_fn>,
    pub add_mld_mac_filter: Option<add_mld6_mac_filter_fn>,
    pub del_mld_mac_filter: Option<del_mld6_mac_filter_fn>,
}
