/*
 * utils.c - various utility functions used in pppd.
 *
 * Copyright (c) 1999-2002 Paul Mackerras. All rights reserved.
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





































extern strerror: &mut String();


pub fn ppp_logit(level: i32, fmt: &String, va_list args);
pub fn ppp_log_write(level: i32, buf: &mut String);

pub fn ppp_vslp_printer(arg: &mut Vec<u8>, fmt: &String, ...);
pub fn ppp_format_packet(const u_p: &mut String, len: i32,
		void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>);

struct buffer_info {
    ptr: &mut String;
    len: i32;
};


/*
 * ppp_strlcpy - like strcpy/strncpy, doesn't overflow destination buffer,
 * always leaves destination null-terminated (for len > 0).
 */
ppp_strlcpy: usize(dest: &mut String, src: &String, len: usize) {
    ret: usize = strlen(src);

    if (len != 0) {
	if (ret < len)
	    strcpy(dest, src);
	else {
	    strncpy(dest, src, len - 1);
	    dest[len-1] = 0;
	}
    }
    return ret;
}

/*
 * ppp_strlcat - like strcat/strncat, doesn't overflow destination buffer,
 * always leaves destination null-terminated (for len > 0).
 */
ppp_strlcat: usize(dest: &mut String, src: &String, len: usize) {
    dlen: usize = strlen(dest);

    return dlen + ppp_strlcpy(dest + dlen, src, (len > dlen? len - dlen: 0));
}


/*
 * ppp_slprintf - format a message into a buffer.  Like sprintf except we
 * also specify the length of the output buffer, and we handle
 * %m (error message), %v (visible string),
 * %q (quoted string), %t (current time) and %I (IP address) formats.
 * Doesn't do floating-poformats: i32.
 * Returns the number of chars put into buf.
 */
ppp_slprintf: i32(buf: &mut String, buflen: i32, fmt: &String, ...) {
    va_list args;
    n: i32;

    va_start(args, fmt);
    n = ppp_vslprintf(buf, buflen, fmt, args);
    va_end(args);
    return n;
}

/*
 * ppp_vslprintf - like ppp_slprintf, takes a va_list instead of a list of args.
 */
#define OUTCHAR(c)	(buflen > 0? (--buflen, *buf+= 1 = (c)): 0)

ppp_vslprintf: i32(buf: &mut String, buflen: i32, fmt: &String, va_list args) {
    c: i32, i, n;
    width: i32, prec, fillch;
    base: i32, len, neg, quoted;
     long val = 0;
    f: String;
    str: &mut String, *buf0;
    const  p: &mut String;
    char num[32];

    t: time_t;

    ip: u32;
    static char hexchars[] = "0123456789abcdef";

    struct buffer_info bufinfo;


    buf0 = buf;
    --buflen;
    while (buflen > 0) {
	for (f = fmt; *f != '%' && *f != 0; += 1f)
	    ;
	if (f > fmt) {
	    len = f - fmt;
	    if (len > buflen)
		len = buflen;
	    memcpy(buf, fmt, len);
	    buf += len;
	    buflen -= len;
	    fmt = f;
	}
	if (*fmt == 0)
	    break;
	c = *+= 1fmt;
	width = 0;
	prec = -1;
	fillch = ' ';
	if (c == '0') {
	    fillch = '0';
	    c = *+= 1fmt;
	}
	if (c == '*') {
	    width = va_arg(args, int);
	    c = *+= 1fmt;
	} else {
	    while (lwip_isdigit(c)) {
		width = width * 10 + c - '0';
		c = *+= 1fmt;
	    }
	}
	if (c == '.') {
	    c = *+= 1fmt;
	    if (c == '*') {
		prec = va_arg(args, int);
		c = *+= 1fmt;
	    } else {
		prec = 0;
		while (lwip_isdigit(c)) {
		    prec = prec * 10 + c - '0';
		    c = *+= 1fmt;
		}
	    }
	}
	str = 0;
	base = 0;
	neg = 0;
	+= 1fmt;
	match (c) {
	case 'l':
	    c = *fmt+= 1;
	    match (c) {
	    case 'd':
		val = va_arg(args, long);
		if ((long)val < 0) {
		    neg = 1;
		    val = ( long)-(long)val;
		}
		base = 10;
		break;
	    case 'u':
		val = va_arg(args,  long);
		base = 10;
		break;
	    _ =>
		OUTCHAR('%');
		OUTCHAR('l');
		--fmt;		/* so %lz outputs %lz etc. */
		continue;
	    }
	    break;
	case 'd':
	    i = va_arg(args, int);
	    if (i < 0) {
		neg = 1;
		val = -i;
	    } else
		val = i;
	    base = 10;
	    break;
	case 'u':
	    val = va_arg(args,  int);
	    base = 10;
	    break;
	case 'o':
	    val = va_arg(args,  int);
	    base = 8;
	    break;
	case 'x':
	case 'X':
	    val = va_arg(args,  int);
	    base = 16;
	    break;

	case 'p':
	    val = ( long) va_arg(args, void *);
	    base = 16;
	    neg = 2;
	    break;

	case 's':
	    str = va_arg(args, char *);
	    break;
	case 'c':
	    num[0] = va_arg(args, int);
	    num[1] = 0;
	    str = num;
	    break;

	case 'm':
	    str = strerror(errno);
	    break;

	case 'I':
	    ip = va_arg(args, u32);
	    ip = lwip_ntohl(ip);
	    ppp_slprintf(num, sizeof(num), "%d.%d.%d.%d", (ip >> 24) & 0xff,
		     (ip >> 16) & 0xff, (ip >> 8) & 0xff, ip & 0xff);
	    str = num;
	    break;

	case 't':
	    time(&t);
	    str = ctime(&t);
	    str += 4;		/* chop off the day name */
	    str[15] = 0;	/* chop off year and newline */
	    break;

	case 'v':		/* "visible" string */
	case 'q':		/* quoted string */
	    quoted = c == 'q';
	    p = va_arg(args,  char *);
	    if (p == NULL)
		p = (const  char *)"<NULL>";
	    if (fillch == '0' && prec >= 0) {
		n = prec;
	    } else {
		n = strlen((const char *)p);
		if (prec >= 0 && n > prec)
		    n = prec;
	    }
	    while (n > 0 && buflen > 0) {
		c = *p+= 1;
		--n;
		if (!quoted && c >= 0x80) {
		    OUTCHAR('M');
		    OUTCHAR('-');
		    c -= 0x80;
		}
		if (quoted && (c == '"' || c == '\\'))
		    OUTCHAR('\\');
		if (c < 0x20 || (0x7f <= c && c < 0xa0)) {
		    if (quoted) {
			OUTCHAR('\\');
			match (c) {
			case '\t':	OUTCHAR('t');	break;
			case '\n':	OUTCHAR('n');	break;
			case '\b':	OUTCHAR('b');	break;
			case '\f':	OUTCHAR('f');	break;
			_ =>
			    OUTCHAR('x');
			    OUTCHAR(hexchars[c >> 4]);
			    OUTCHAR(hexchars[c & 0xf]);
			}
		    } else {
			if (c == '\t')
			    OUTCHAR(c);
			else {
			    OUTCHAR('^');
			    OUTCHAR(c ^ 0x40);
			}
		    }
		} else
		    OUTCHAR(c);
	    }
	    continue;

	case 'P':		/* prPPP: i32 packet */
	    bufinfo.ptr = buf;
	    bufinfo.len = buflen + 1;
	    p = va_arg(args,  char *);
	    n = va_arg(args, int);
	    ppp_format_packet(p, n, ppp_vslp_printer, &bufinfo);
	    buf = bufinfo.ptr;
	    buflen = bufinfo.len - 1;
	    continue;

	case 'B':
	    p = va_arg(args,  char *);
	    for (n = prec; n > 0; --n) {
		c = *p+= 1;
		if (fillch == ' ')
		    OUTCHAR(' ');
		OUTCHAR(hexchars[(c >> 4) & 0xf]);
		OUTCHAR(hexchars[c & 0xf]);
	    }
	    continue;
	_ =>
	    *buf+= 1 = '%';
	    if (c != '%')
		--fmt;		/* so %z outputs %z etc. */
	    --buflen;
	    continue;
	}
	if (base != 0) {
	    str = num + sizeof(num);
	    *--str = 0;
	    while (str > num + neg) {
		*--str = hexchars[val % base];
		val = val / base;
		if (--prec <= 0 && val == 0)
		    break;
	    }
	    match (neg) {
	    1 =>
		*--str = '-';
		break;
	    2 =>
		*--str = 'x';
		*--str = '0';
		break;
	    _ =>
		break;
	    }
	    len = num + sizeof(num) - 1 - str;
	} else {
	    len = strlen(str);
	    if (prec >= 0 && len > prec)
		len = prec;
	}
	if (width > 0) {
	    if (width > buflen)
		width = buflen;
	    if ((n = width - len) > 0) {
		buflen -= n;
		for (; n > 0; --n)
		    *buf+= 1 = fillch;
	    }
	}
	if (len > buflen)
	    len = buflen;
	memcpy(buf, str, len);
	buf += len;
	buflen -= len;
    }
    *buf = 0;
    return buf - buf0;
}


/*
 * vslp_printer - used in processing a %P format
 */
pub fn ppp_vslp_printer(arg: &mut Vec<u8>, fmt: &String, ...) {
    n: i32;
    va_list pvar;
    bi: &mut buffer_info;

    va_start(pvar, fmt);
    bi = (struct buffer_info *) arg;
    n = ppp_vslprintf(bi.ptr, bi.len, fmt, pvar);
    va_end(pvar);

    bi.ptr += n;
    bi.len -= n;
}



/*
 * log_packet - format a packet and log it.
 */

pub fn 
log_packet(p, len, prefix, level)
    u_p: &mut String;
    len: i32;
    prefix: &mut String;
    level: i32;
{
	init_pr_log(prefix, level);
	ppp_format_packet(p, len, pr_log, &level);
	end_pr_log();
}



/*
 * ppp_format_packet - make a readable representation of a packet,
 * calling `printer(arg, format, ...)' to output it.
 */
pub fn ppp_format_packet(const u_p: &mut String, len: i32,
		void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>) {
    i: i32, n;
    u_short proto;
    const protp: &mut protent;

    if (len >= 2) {
	GETSHORT(proto, p);
	len -= 2;
	for (i = 0; (protp = protocols[i]) != NULL; += 1i)
	    if (proto == protp.protocol)
		break;
	if (protp != NULL) {
	    printer(arg, "[%s", protp.name);
	    n = (*protp.printpkt)(p, len, printer, arg);
	    printer(arg, "]");
	    p += n;
	    len -= n;
	} else {
	    for (i = 0; (protp = protocols[i]) != NULL; += 1i)
		if (proto == (protp.protocol & !0x8000))
		    break;
	    if (protp != 0 && protp.data_name != 0) {
		printer(arg, "[%s data]", protp.data_name);
		if (len > 8)
		    printer(arg, "%.8B ...", p);
		else
		    printer(arg, "%.*B", len, p);
		len = 0;
	    } else
		printer(arg, "[proto=0x%x]", proto);
	}
    }

    if (len > 32)
	printer(arg, "%.32B ...", p);
    else
	printer(arg, "%.*B", len, p);
}



/*
 * init_pr_log, end_pr_log - initialize and finish use of pr_log.
 */

static char line[256];		/* line to be logged accumulated here */
static linep: &mut String;		/* current pointer within line */
static llevel: i32;		/* level for logging */

pub fn 
init_pr_log(prefix, level)
     prefix: String;
     level: i32;
{
	linep = line;
	if (prefix != NULL) {
		ppp_strlcpy(line, prefix, sizeof(line));
		linep = line + strlen(line);
	}
	llevel = level;
}

pub fn 
end_pr_log()
{
	if (linep != line) {
		*linep = 0;
		ppp_log_write(llevel, line);
	}
}

/*
 * pr_log - printer routine for outputting to log
 */
pub fn 
pr_log (arg: &mut Vec<u8>, fmt: &String, ...)
{
	l: i32, n;
	va_list pvar;
	p: &mut String, *eol;
	char buf[256];

	va_start(pvar, fmt);
	n = ppp_vslprintf(buf, sizeof(buf), fmt, pvar);
	va_end(pvar);

	p = buf;
	eol = strchr(buf, '\n');
	if (linep != line) {
		l = (eol == NULL)? n: eol - buf;
		if (linep + l < line + sizeof(line)) {
			if (l > 0) {
				memcpy(linep, buf, l);
				linep += l;
			}
			if (eol == NULL)
				return;
			p = eol + 1;
			eol = strchr(p, '\n');
		}
		*linep = 0;
		ppp_log_write(llevel, line);
		linep = line;
	}

	while (eol != NULL) {
		*eol = 0;
		ppp_log_write(llevel, p);
		p = eol + 1;
		eol = strchr(p, '\n');
	}

	/* assumes sizeof(buf) <= sizeof(line) */
	l = buf + n - p;
	if (l > 0) {
		memcpy(line, p, n);
		linep = line + l;
	}
}


/*
 * ppp_print_string - pra: i32 readable representation of a string using
 * printer.
 */
pub fn  ppp_print_string(const u_p: &mut String, len: i32, void (*printer) (void *,  char *, ...), arg: &mut Vec<u8>) {
    c: i32;

    printer(arg, "\"");
    for (; len > 0; --len) {
	c = *p+= 1;
	if (' ' <= c && c <= '!') {
	    if (c == '\\' || c == '"')
		printer(arg, "\\");
	    printer(arg, "%c", c);
	} else {
	    match (c) {
	    case '\n':
		printer(arg, "\\n");
		break;
	    case '\r':
		printer(arg, "\\r");
		break;
	    case '\t':
		printer(arg, "\\t");
		break;
	    _ =>
		printer(arg, "\\%.3o", c);
		/* no break */
	    }
	}
    }
    printer(arg, "\"");
}

/*
 * ppp_logit - does the hard work for fatal et al.
 */
pub fn ppp_logit(level: i32, fmt: &String, va_list args) {
    char buf[1024];

    ppp_vslprintf(buf, sizeof(buf), fmt, args);
    ppp_log_write(level, buf);
}

pub fn ppp_log_write(level: i32, buf: &mut String) {
     /* necessary if PPPDEBUG is defined to an empty function */
    
    PPPDEBUG(level, ("%s\n", buf) );

    if (log_to_fd >= 0 && (level != LOG_DEBUG || debug)) {
	n: i32 = strlen(buf);

	if (n > 0 && buf[n-1] == '\n')
	    --n;
	if (write(log_to_fd, buf, n) != n
	    || write(log_to_fd, "\n", 1) != 1)
	    log_to_fd = -1;
    }

}

/*
 * ppp_fatal - log an error message and die horribly.
 */
pub fn  ppp_fatal(fmt: &String, ...) {
    va_list pvar;

    va_start(pvar, fmt);
    ppp_logit(LOG_ERR, fmt, pvar);
    va_end(pvar);

    LWIP_ASSERT("ppp_fatal", 0);   /* as promised */
}

/*
 * ppp_error - log an error message.
 */
pub fn  ppp_error(fmt: &String, ...) {
    va_list pvar;

    va_start(pvar, fmt);
    ppp_logit(LOG_ERR, fmt, pvar);
    va_end(pvar);

    += 1error_count;

}

/*
 * ppp_warn - log a warning message.
 */
pub fn  ppp_warn(fmt: &String, ...) {
    va_list pvar;

    va_start(pvar, fmt);
    ppp_logit(LOG_WARNING, fmt, pvar);
    va_end(pvar);
}

/*
 * ppp_notice - log a notice-level message.
 */
pub fn  ppp_notice(fmt: &String, ...) {
    va_list pvar;

    va_start(pvar, fmt);
    ppp_logit(LOG_NOTICE, fmt, pvar);
    va_end(pvar);
}

/*
 * ppp_info - log an informational message.
 */
pub fn  ppp_info(fmt: &String, ...) {
    va_list pvar;

    va_start(pvar, fmt);
    ppp_logit(LOG_INFO, fmt, pvar);
    va_end(pvar);
}

/*
 * ppp_dbglog - log a debug message.
 */
pub fn  ppp_dbglog(fmt: &String, ...) {
    va_list pvar;

    va_start(pvar, fmt);
    ppp_logit(LOG_DEBUG, fmt, pvar);
    va_end(pvar);
}


/*
 * ppp_dump_packet - prout: i32 a packet in readable form if it is interesting.
 * Assumes len >= PPP_HDRLEN.
 */
pub fn  ppp_dump_packet(pcb: &mut ppp_pcb, tag: &String,  p: &mut String, len: i32) {
    proto: i32;

    /*
     * don't prdata: i32 packets, i.e. IPv4, IPv6, VJ, and compressed packets.
     */
    proto = (p[0] << 8) + p[1];
    if (proto < 0xC000 && (proto & !0x8000) == proto)
	return;

    /*
     * don't prvalid: i32 LCP echo request/reply packets if the link is up.
     */
    if (proto == PPP_LCP && pcb.phase == PPP_PHASE_RUNNING && len >= 2 + HEADERLEN) {
	 lcp: &mut String = p + 2;
	l: i32 = (lcp[2] << 8) + lcp[3];

	if ((lcp[0] == ECHOREQ || lcp[0] == ECHOREP)
	    && l >= HEADERLEN && l <= len - 2)
	    return;
    }

    ppp_dbglog("%s %P", tag, p, len);
}




/*
 * complete_read - read a full `count' bytes from fd,
 * unless end-of-file or an error other than EINTR is encountered.
 */
isize
complete_read(fd: i32, buf: &mut (), count: usize)
{
	done: usize;
	snb: usize;
	ptr: &mut String = buf;

	for (done = 0; done < count; ) {
		nb = read(fd, ptr, count - done);
		if (nb < 0) {
			if (errno == EINTR)
				continue;
			return -1;
		}
		if (nb == 0)
			break;
		done += nb;
		ptr += nb;
	}
	return done;
}

/* Procedures for locking the serial device using a lock file. */


#define LOCK_DIR	"/var/lock"


#define LOCK_DIR	"/var/spool/locks"

#define LOCK_DIR	"/var/spool/lock"




static char lock_file[MAXPATHLEN];

/*
 * lock - create a lock file for the named device
 */
pub fn lock(dev)
    dev: &mut String;
{

    result: i32;

    result = mklock (dev,  0);
    if (result == 0) {
	ppp_strlcpy(lock_file, dev, sizeof(lock_file));
	return 0;
    }

    if (result > 0)
        ppp_notice("Device %s is locked by pid %d", dev, result);
    else
	ppp_error("Can't create lock file %s", lock_file);
    return -1;

 /* LOCKLIB */

    char lock_buffer[12];
    fd: i32, pid, n;


    struct stat sbuf;

    if (stat(dev, &sbuf) < 0) {
	ppp_error("Can't get device number for %s: %m", dev);
	return -1;
    }
    if ((sbuf.st_mode & S_IFMT) != S_IFCHR) {
	ppp_error("Can't lock %s: not a character device", dev);
	return -1;
    }
    ppp_slprintf(lock_file, sizeof(lock_file), "%s/LK.%03d.%03d.%03d",
	     LOCK_DIR, major(sbuf.st_dev),
	     major(sbuf.st_rdev), minor(sbuf.st_rdev));

    p: &mut String;
    char lockdev[MAXPATHLEN];

    if ((p = strstr(dev, "dev/")) != NULL) {
	dev = p + 4;
	strncpy(lockdev, dev, MAXPATHLEN-1);
	lockdev[MAXPATHLEN-1] = 0;
	while ((p = strrchr(lockdev, '/')) != NULL) {
	    *p = '_';
	}
	dev = lockdev;
    } else
	if ((p = strrchr(dev, '/')) != NULL)
	    dev = p + 1;

    ppp_slprintf(lock_file, sizeof(lock_file), "%s/LCK..%s", LOCK_DIR, dev);


    while ((fd = open(lock_file, O_EXCL | O_CREAT | O_RDWR, 0644)) < 0) {
	if (errno != EEXIST) {
	    ppp_error("Can't create lock file %s: %m", lock_file);
	    break;
	}

	/* Read the lock file to find out who has the device locked. */
	fd = open(lock_file, O_RDONLY, 0);
	if (fd < 0) {
	    if (errno == ENOENT) /* This is just a timing problem. */
		continue;
	    ppp_error("Can't open existing lock file %s: %m", lock_file);
	    break;
	}

	n = read(fd, lock_buffer, 11);

	n = read(fd, &pid, sizeof(pid));

	close(fd);
	fd = -1;
	if (n <= 0) {
	    ppp_error("Can't read pid from lock file %s", lock_file);
	    break;
	}

	/* See if the process still exists. */

	lock_buffer[n] = 0;
	pid = atoi(lock_buffer);

	if (pid == getpid())
	    return 1;		/* somebody else locked it for us */
	if (pid == 0
	    || (kill(pid, 0) == -1 && errno == ESRCH)) {
	    if (unlink (lock_file) == 0) {
		ppp_notice("Removed stale lock on %s (pid %d)", dev, pid);
		continue;
	    }
	    ppp_warn("Couldn't remove stale lock on %s", dev);
	} else
	    ppp_notice("Device %s is locked by pid %d", dev, pid);
	break;
    }

    if (fd < 0) {
	lock_file[0] = 0;
	return -1;
    }

    pid = getpid();

    ppp_slprintf(lock_buffer, sizeof(lock_buffer), "%10d\n", pid);
    write (fd, lock_buffer, 11);

    write(fd, &pid, sizeof (pid));

    close(fd);
    return 0;


}

/*
 * relock - called to update our lockfile when we are about to detach,
 * thus changing our pid (we fork, the child carries on, and the parent dies).
 * Note that this is called by the parent, with pid equal to the pid
 * of the child.  This avoids a potential race which would exist if
 * we had the child rewrite the lockfile (the parent might die first,
 * and another process could think the lock was stale if it checked
 * between when the parent died and the child rewrote the lockfile).
 */
pub fn relock(pid)
    pid: i32;
{

    /* XXX is there a way to do this? */
    return -1;
 /* LOCKLIB */

    fd: i32;
    char lock_buffer[12];

    if (lock_file[0] == 0)
	return -1;
    fd = open(lock_file, O_WRONLY, 0);
    if (fd < 0) {
	ppp_error("Couldn't reopen lock file %s: %m", lock_file);
	lock_file[0] = 0;
	return -1;
    }


    ppp_slprintf(lock_buffer, sizeof(lock_buffer), "%10d\n", pid);
    write (fd, lock_buffer, 11);

    write(fd, &pid, sizeof(pid));

    close(fd);
    return 0;


}

/*
 * unlock - remove our lockfile
 */
pub fn 
unlock()
{
    if (lock_file[0]) {

	() rmlock(lock_file,  0);

	unlink(lock_file);

	lock_file[0] = 0;
    }
}




