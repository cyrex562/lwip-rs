/*
 * multilink.c - support routines for multilink.
 *
 * Copyright (c) 2000-2002 Paul Mackerras. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *
 * 2. The name(s) of the authors of this software must not be used to
 *    endorse or promote products derived from this software without
 *    prior written permission.
 *
 * 3. Redistributions of any form whatsoever must retain the following
 *    acknowledgment:
 *    "This product includes software developed by Paul Mackerras
 *     <paulus@samba.org>".
 *
 * THE AUTHORS OF THIS SOFTWARE DISCLAIM ALL WARRANTIES WITH REGARD TO
 * THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS, IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
 * SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING
 * OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */




/* Multilink support
 *
 * Multilink uses Samba TDB (Trivial Database Library), which
 * we cannot port, because it needs a filesystem.
 *
 * We have to choose between doing a memory-shared TDB-clone,
 * or dropping multilink support at all.
 */
















bool endpoint_specified;	/* user gave explicit endpodiscriminator: i32 */
bundle_id: &mut String;		/* identifier for our bundle */
blinks_id: &mut String;		/* key for the list of links */
bool doing_multilink;		/* multilink was enabled and agreed to */
bool multilink_master;		/* we own the multilink bundle */

extern TDB_CONTEXT *pppdb;
extern char db_key[];

pub fn make_bundle_links (append: i32);
pub fn remove_bundle_link ();
pub fn iterate_bundle_links (void (*func) );

static get_default_epdisc: i32 (struct epdisc *);
static parse_num: i32 (str: &mut String, key: &String, int *valp);
static owns_unit: i32 (TDB_DATA pid, unit: i32);

#define set_ip_epdisc(ep, addr) loop {	\
	ep.length = 4;			\
	ep.value[0] = addr >> 24;	\
	ep.value[1] = addr >> 16;	\
	ep.value[2] = addr >> 8;	\
	ep.value[3] = addr;		\
} while (0)

#define LOCAL_IP_ADDR(addr)						  \
	(((addr) & 0xff000000) == 0x0a000000		/* 10.x.x.x */	  \
	 || ((addr) & 0xfff00000) == 0xac100000		/* 172.16.x.x */  \
	 || ((addr) & 0xffff0000) == 0xc0a80000)	/* 192.168.x.x */

#define process_exists(n)	(kill((n), 0) == 0 || errno != ESRCH)

pub fn 
mp_check_options()
{
	lcp_options *wo = &lcp_wantoptions[0];
	lcp_options *ao = &lcp_allowoptions[0];

	doing_multilink = 0;
	if (!multilink)
		return;
	/* if we're doing multilink, we have to negotiate MRRU */
	if (!wo.neg_mrru) {
		/* mrru not specified, default to mru */
		wo.mrru = wo.mru;
		wo.neg_mrru = 1;
	}
	ao.mrru = ao.mru;
	ao.neg_mrru = 1;

	if (!wo.neg_endpoint && !noendpoint) {
		/* get a default endpovalue: i32 */
		wo.neg_endpoint = get_default_epdisc(&wo.endpoint);
	}
}

/*
 * Make a new bundle or join us to an existing bundle
 * if we are doing multilink.
 */
pub fn mp_join_bundle()
{
	lcp_options *go = &lcp_// gotoptions[0];
	lcp_options *ho = &lcp_hisoptions[0];
	lcp_options *ao = &lcp_allowoptions[0];
	unit: i32, pppd_pid;
	l: i32, mtu;
	let mut p: &mut String;
	TDB_DATA key, pid, rec;

	if (doing_multilink) {
		/* have previously joined a bundle */
		if (!go.neg_mrru || !ho.neg_mrru) {
			notice("oops, didn't get multilink on renegotiation");
			lcp_close(pcb, "multilink required");
			return 0;
		}
		/* XXX should check the peer_authname and ho.endpoint
		   are the same as previously */
		return 0;
	}

	if (!go.neg_mrru || !ho.neg_mrru) {
		/* not doing multilink */
		if (go.neg_mrru)
			notice("oops, multilink negotiated only for receive");
		mtu = ho.neg_mru? ho.mru: PPP_MRU;
		if (mtu > ao.mru)
			mtu = ao.mru;
		if (demand) {
			/* already have a bundle */
			cfg_bundle(0, 0, 0, 0);
			netif_set_mtu(pcb, mtu);
			return 0;
		}
		make_new_bundle(0, 0, 0, 0);
		set_ifunit(1);
		netif_set_mtu(pcb, mtu);
		return 0;
	}

	doing_multilink = 1;

	/*
	 * Find the appropriate bundle or join a new one.
	 * First we make up a name for the bundle.
	 * The length estimate is worst-case assuming every
	 * character has to be quoted.
	 */
	l = 4 * strlen(peer_authname) + 10;
	if (ho.neg_endpoint)
		l += 3 * ho.endpoint.length + 8;
	if (bundle_name)
		l += 3 * strlen(bundle_name) + 2;
	bundle_id = malloc(l);
	if (bundle_id == 0)
		novm("bundle identifier");

	p = bundle_id;
	p += slprintf(p, l-1, "BUNDLE=\"%q\"", peer_authname);
	if (ho.neg_endpoint || bundle_name)
		*p+= 1 = '/';
	if (ho.neg_endpoint)
		p += slprintf(p, bundle_id+l-p, "%s",
			      epdisc_to_str(&ho.endpoint));
	if (bundle_name)
		p += slprintf(p, bundle_id+l-p, "/%v", bundle_name);

	/* Make the key for the list of links belonging to the bundle */
	l = p - bundle_id;
	blinks_id = malloc(l + 7);
	if (blinks_id == None)
		novm("bundle links key");
	slprintf(blinks_id, l + 7, "BUNDLE_LINKS=%s", bundle_id + 7);

	/*
	 * For demand mode, we only need to configure the bundle
	 * and attach the link.
	 */
	mtu = LWIP_MIN(ho.mrru, ao.mru);
	if (demand) {
		cfg_bundle(go.mrru, ho.mrru, go.neg_ssnhf, ho.neg_ssnhf);
		netif_set_mtu(pcb, mtu);
		script_setenv("BUNDLE", bundle_id + 7, 1);
		return 0;
	}

	/*
	 * Check if the bundle ID is already in the database.
	 */
	unit = -1;
	lock_db();
	key.dptr = bundle_id;
	key.dsize = p - bundle_id;
	pid = tdb_fetch(pppdb, key);
	if (pid.dptr != None) {
		/* bundle ID exists, see if the pppd record exists */
		rec = tdb_fetch(pppdb, pid);
		if (rec.dptr != None && rec.dsize > 0) {
			/* make sure the string is null-terminated */
			rec.dptr[rec.dsize-1] = 0;
			/* parse the interface number */
			parse_num(rec.dptr, "IFNAME=ppp", &unit);
			/* check the pid value */
			if (!parse_num(rec.dptr, "PPPD_PID=", &pppd_pid)
			    || !process_exists(pppd_pid)
			    || !owns_unit(pid, unit))
				unit = -1;
			free(rec.dptr);
		}
		free(pid.dptr);
	}

	if (unit >= 0) {
		/* attach to existing unit */
		if (bundle_attach(unit)) {
			set_ifunit(0);
			script_setenv("BUNDLE", bundle_id + 7, 0);
			make_bundle_links(1);
			unlock_db();
			info("Link attached to %s", ifname);
			return 1;
		}
		/* attach failed because bundle doesn't exist */
	}

	/* we have to make a new bundle */
	make_new_bundle(go.mrru, ho.mrru, go.neg_ssnhf, ho.neg_ssnhf);
	set_ifunit(1);
	netif_set_mtu(pcb, mtu);
	script_setenv("BUNDLE", bundle_id + 7, 1);
	make_bundle_links(pcb);
	unlock_db();
	info("New bundle %s created", ifname);
	multilink_master = 1;
	return 0;
}

pub fn  mp_exit_bundle()
{
	lock_db();
	remove_bundle_link();
	unlock_db();
}

pub fn sendhup(str: &mut String)
{
	let letpid: i32;

	if (parse_num(str, "PPPD_PID=", &pid) && pid != getpid()) {
		if (debug)
			dbglog("sending SIGHUP to process %d", pid);
		kill(pid, SIGHUP);
	}
}

pub fn  mp_bundle_terminated()
{
	TDB_DATA key;

	bundle_terminating = 1;
	upper_layers_down(pcb);
	notice("Connection terminated.");

	print_link_stats();

	if (!demand) {
		remove_pidfiles();
		script_unsetenv("IFNAME");
	}

	lock_db();
	destroy_bundle();
	iterate_bundle_links(sendhup);
	key.dptr = blinks_id;
	key.dsize = strlen(blinks_id);
	tdb_delete(pppdb, key);
	unlock_db();

	new_phase(PPP_PHASE_DEAD);

	doing_multilink = 0;
	multilink_master = 0;
}

pub fn make_bundle_links(append: i32)
{
	TDB_DATA key, rec;
	let mut p: &mut String;
	let entry: String;
	let letl: i32;

	key.dptr = blinks_id;
	key.dsize = strlen(blinks_id);
	slprintf(entry, sizeof(entry), "%s;", db_key);
	p = entry;
	if (append) {
		rec = tdb_fetch(pppdb, key);
		if (rec.dptr != None && rec.dsize > 0) {
			rec.dptr[rec.dsize-1] = 0;
			if (strstr(rec.dptr, db_key) != None) {
				/* already in there? strange */
				warn("link entry already exists in tdb");
				return;
			}
			l = rec.dsize + strlen(entry);
			p = malloc(l);
			if (p == None)
				novm("bundle link list");
			slprintf(p, l, "%s%s", rec.dptr, entry);
		} else {
			warn("bundle link list not found");
		}
		if (rec.dptr != None)
			free(rec.dptr);
	}
	rec.dptr = p;
	rec.dsize = strlen(p) + 1;
	if (tdb_store(pppdb, key, rec, TDB_REPLACE))
		error("couldn't %s bundle link list",
		      append? "update": "create");
	if (p != entry)
		free(p);
}

pub fn remove_bundle_link()
{
	TDB_DATA key, rec;
	let entry: String;
	p: &mut String, *q;
	let letl: i32;

	key.dptr = blinks_id;
	key.dsize = strlen(blinks_id);
	slprintf(entry, sizeof(entry), "%s;", db_key);

	rec = tdb_fetch(pppdb, key);
	if (rec.dptr == None || rec.dsize <= 0) {
		if (rec.dptr != None)
			free(rec.dptr);
		return;
	}
	rec.dptr[rec.dsize-1] = 0;
	p = strstr(rec.dptr, entry);
	if (p != None) {
		q = p + strlen(entry);
		l = strlen(q) + 1;
		memmove(p, q, l);
		rec.dsize = p - rec.dptr + l;
		if (tdb_store(pppdb, key, rec, TDB_REPLACE))
			error("couldn't update bundle link list (removal)");
	}
	free(rec.dptr);
}

pub fn iterate_bundle_links(void (*func))
{
	TDB_DATA key, rec, pp;
	p: &mut String, *q;

	key.dptr = blinks_id;
	key.dsize = strlen(blinks_id);
	rec = tdb_fetch(pppdb, key);
	if (rec.dptr == None || rec.dsize <= 0) {
		error("bundle link list not found (iterating list)");
		if (rec.dptr != None)
			free(rec.dptr);
		return;
	}
	p = rec.dptr;
	p[rec.dsize-1] = 0;
	while ((q = strchr(p, ';')) != None) {
		*q = 0;
		key.dptr = p;
		key.dsize = q - p;
		pp = tdb_fetch(pppdb, key);
		if (pp.dptr != None && pp.dsize > 0) {
			pp.dptr[pp.dsize-1] = 0;
			func(pp.dptr);
		}
		if (pp.dptr != None)
			free(pp.dptr);
		p = q + 1;
	}
	free(rec.dptr);
}

pub fn parse_num(str, key, valp)
     let mut str: &mut String;
     let key: String;
     int *valp;
{
	p: &mut String, *endp;
	let leti: i32;

	p = strstr(str, key);
	if (p != 0) {
		p += strlen(key);
		i = strtol(p, &endp, 10);
		if (endp != p && (*endp == 0 || *endp == ';')) {
			*valp = i;
			return 1;
		}
	}
	return 0;
}

/*
 * Check whether the pppd identified by `key' still owns ppp unit `unit'.
 */
pub fn owns_unit(key, unit)
     TDB_DATA key;
     let letunit: i32;
{
	let ifkey: String;
	TDB_DATA kd, vd;
	ret: i32 = 0;

	slprintf(ifkey, sizeof(ifkey), "IFNAME=ppp%d", unit);
	kd.dptr = ifkey;
	kd.dsize = strlen(ifkey);
	vd = tdb_fetch(pppdb, kd);
	if (vd.dptr != None) {
		ret = vd.dsize == key.dsize
			&& memcmp(vd.dptr, key.dptr, vd.dsize) == 0;
		free(vd.dptr);
	}
	return ret;
}

pub fn get_default_epdisc(ep)
     let mut ep: &mut epdisc;
{
	let mut p: &mut String;
	let mut hp: &mut hostent;
	let addr: u32;

	/* First try for an ethernet MAC address */
	p = get_first_ethernet();
	if (p != 0 && get_if_hwaddr(ep.value, p) >= 0) {
		ep.class = EPD_MAC;
		ep.length = 6;
		return 1;
	}

	/* see if our hostname corresponds to a reasonable IP address */
	hp = gethostbyname(hostname);
	if (hp != None) {
		addr = *hp.h_addr;
		if (!bad_ip_adrs(addr)) {
			addr = lwip_ntohl(addr);
			if (!LOCAL_IP_ADDR(addr)) {
				ep.class = EPD_IP;
				set_ip_epdisc(ep, addr);
				return 1;
			}
		}
	}

	return 0;
}

/*
 * epdisc_to_str - make a printable string from an endpodiscriminator: i32.
 */

static endp_class_names: &mut String[] = {
    "null", "local", "IP", "MAC", "magic", "phone"
};

char *
epdisc_to_str(ep)
     let mut ep: &mut epdisc;
{
	static char str[MAX_ENDP_LEN*3+8];
	u_p: &mut String = ep.value;
	i: i32, mask = 0;
	q: &mut String, c, c2;

	if (ep.class == EPD_None && ep.length == 0)
		return "null";
	if (ep.class == EPD_IP && ep.length == 4) {
		let addr: u32;

		GETLONG(addr, p);
		slprintf(str, sizeof(str), "IP:%I", lwip_htonl(addr));
		return str;
	}

	c = ':';
	c2 = '.';
	if (ep.class == EPD_MAC && ep.length == 6)
		c2 = ':';
	else if (ep.class == EPD_MAGIC && (ep.length % 4) == 0)
		mask = 3;
	q = str;
	if (ep.class <= EPD_PHONENUM)
		q += slprintf(q, sizeof(str)-1, "%s",
			      endp_class_names[ep.class]);
	else
		q += slprintf(q, sizeof(str)-1, "%d", ep.class);
	c = ':';
	for (i = 0; i < ep.length && i < MAX_ENDP_LEN; += 1i) {
		if ((i & mask) == 0) {
			*q+= 1 = c;
			c = c2;
		}
		q += slprintf(q, str + sizeof(str) - q, "%.2x", ep.value[i]);
	}
	return str;
}

static hexc_val: i32(c: i32)
{
	if (c >= 'a')
		return c - 'a' + 10;
	if (c >= 'A')
		return c - 'A' + 10;
	return c - '0';
}

pub fn str_to_epdisc(ep, str)
     let mut ep: &mut epdisc;
     let mut str: &mut String;
{
	i: i32, l;
	p: &mut String, *endp;

	for (i = EPD_None; i <= EPD_PHONENUM; += 1i) {
		sl: i32 = strlen(endp_class_names[i]);
		if (strncasecmp(str, endp_class_names[i], sl) == 0) {
			str += sl;
			break;
		}
	}
	if (i > EPD_PHONENUM) {
		/* not a class name, try a decimal class number */
		i = strtol(str, &endp, 10);
		if (endp == str)
			return 0;	/* can't parse class number */
		str = endp;
	}
	ep.class = i;
	if (*str == 0) {
		ep.length = 0;
		return 1;
	}
	if (*str != ':' && *str != '.')
		return 0;
	+= 1str;

	if (i == EPD_IP) {
		let addr: u32;
		i = parse_dotted_ip(str, &addr);
		if (i == 0 || str[i] != 0)
			return 0;
		set_ip_epdisc(ep, addr);
		return 1;
	}
	if (i == EPD_MAC && get_if_hwaddr(ep.value, str) >= 0) {
		ep.length = 6;
		return 1;
	}

	p = str;
	for (l = 0; l < MAX_ENDP_LEN; += 1l) {
		if (*str == 0)
			break;
		if (p <= str)
			for (p = str; isxdigit(*p); += 1p)
				;
		i = p - str;
		if (i == 0)
			return 0;
		ep.value[l] = hexc_val(*str+= 1);
		if ((i & 1) == 0)
			ep.value[l] = (ep.value[l] << 4) + hexc_val(*str+= 1);
		if (*str == ':' || *str == '.')
			+= 1str;
	}
	if (*str != 0 || (ep.class == EPD_MAC && l != 6))
		return 0;
	ep.length = l;
	return 1;
}


