/*
 * @file
 * Abstract Syntax Notation One (ISO 8824, 8825) encoding
 *
 * @todo not optimised (yet), favor correctness over speed, favor speed over size
 */

/*
 * Copyright (c) 2006 Axon Digital Design B.V., The Netherlands.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * Author: Christiaan Simons <christiaan.simons@axon.tv>
 *         Martin Hentschel <info@cl-soft.de>
 */







#define PBUF_OP_EXEC(code) \
  if ((code) != ERR_OK) { \
    return ERR_BUF; \
  }

/*
 * Encodes a TLV into a pbuf stream.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param tlv TLV to encode
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) encode
 */
pub fn 
snmp_ans1_enc_tlv(pbuf_stream: &mut snmp_pbuf_stream, tlv: &mut snmp_asn1_tlv)
{
  let data: u8;
  let length_bytes_required: u8;

  /* write type */
  if ((tlv.type & SNMP_ASN1_DATATYPE_MASK) == SNMP_ASN1_DATATYPE_EXTENDED) {
    /* extended format is not used by SNMP so we do not accept those values */
    return ERR_ARG;
  }
  if (tlv.type_len != 0) {
    /* any other value as auto is not accepted for type (we always use one byte because extended syntax is prohibited) */
    return ERR_ARG;
  }

  PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, tlv.type));
  tlv.type_len = 1;

  /* write length */
  if (tlv.value_len <= 127) {
    length_bytes_required = 1;
  } else if (tlv.value_len <= 255) {
    length_bytes_required = 2;
  } else  {
    length_bytes_required = 3;
  }

  /* check for forced min length */
  if (tlv.length_len > 0) {
    if (tlv.length_len < length_bytes_required) {
      /* unable to code requested length in requested number of bytes */
      return ERR_ARG;
    }

    length_bytes_required = tlv.length_len;
  } else {
    tlv.length_len = length_bytes_required;
  }

  if (length_bytes_required > 1) {
    /* multi byte representation required */
    length_bytes_required -= 1;
    data = 0x80 | length_bytes_required; /* extended length definition, 1 length byte follows */

    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, data));

    while (length_bytes_required > 1) {
      if (length_bytes_required == 2) {
        /* append high byte */
        data = (tlv.value_len >> 8);
      } else {
        /* append leading 0x00 */
        data = 0x00;
      }

      PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, data));
      length_bytes_required -= 1;
    }
  }

  /* append low byte */
  data = (tlv.value_len & 0xFF);
  PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, data));

 return Ok(());
}

/*
 * Encodes raw data (octet string, opaque) into a pbuf chained ASN1 msg.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param raw_len raw data length
 * @param raw points raw data
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) encode
 */
pub fn 
snmp_asn1_enc_raw(pbuf_stream: &mut snmp_pbuf_stream,  raw: &mut Vec<u8>, raw_len: u16)
{
  PBUF_OP_EXEC(snmp_pbuf_stream_writebuf(pbuf_stream, raw, raw_len));

 return Ok(());
}

/*
 * Encodes u32 (counter, gauge, timeticks) into a pbuf chained ASN1 msg.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param octets_needed encoding length (from snmp_asn1_enc_u32t_cnt())
 * @param value is the host order u32 value to be encoded
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) encode
 *
 * @see snmp_asn1_enc_u32t_cnt()
 */
pub fn 
snmp_asn1_enc_u32t(pbuf_stream: &mut snmp_pbuf_stream, octets_needed: u16, value: u32)
{
  if (octets_needed > 5) {
    return ERR_ARG;
  }
  if (octets_needed == 5) {
    /* not enough bits in 'value' add leading 0x00 */
    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, 0x00));
    octets_needed -= 1;
  }

  while (octets_needed > 1) {
    octets_needed -= 1;
    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, (value >> (octets_needed << 3))));
  }

  /* (only) one least significant octet */
  PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, value));

 return Ok(());
}
/*
 * Encodes i32 integer into a pbuf chained ASN1 msg.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param octets_needed encoding length (from snmp_asn1_enc_s32t_cnt())
 * @param value is the host order i32 value to be encoded
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) encode
 *
 * @see snmp_asn1_enc_s32t_cnt()
 */
pub fn 
snmp_asn1_enc_s32t(pbuf_stream: &mut snmp_pbuf_stream, octets_needed: u16, i32 value)
{
  while (octets_needed > 1) {
    octets_needed -= 1;

    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, (value >> (octets_needed << 3))));
  }

  /* (only) one least significant octet */
  PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, value));

 return Ok(());
}

/*
 * Encodes object identifier into a pbuf chained ASN1 msg.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param oid points to object identifier array
 * @param oid_len object identifier array length
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) encode
 */
pub fn 
snmp_asn1_enc_oid(pbuf_stream: &mut snmp_pbuf_stream,  u32 *oid, oid_len: u16)
{
  if (oid_len > 1) {
    /* write compressed first two sub id's */
    compressed_byte: u32 = ((oid[0] * 40) + oid[1]);
    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, compressed_byte));
    oid_len -= 2;
    oid += 2;
  } else {
    /* @bug:  allow empty varbinds for symmetry (we must decode them for getnext), allow partial compression?? */
    /* ident_len <= 1, at least we need zeroDotZero (0.0) (ident_len == 2) */
    return ERR_ARG;
  }

  while (oid_len > 0) {
    let sub_id: u32;
    shift: u8, tail;

    oid_len -= 1;
    sub_id = *oid;
    tail = 0;
    shift = 28;
    while (shift > 0) {
      let code: u8;

      code = (sub_id >> shift);
      if ((code != 0) || (tail != 0)) {
        tail = 1;
        PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, code | 0x80));
      }
      shift -= 7;
    }
    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, sub_id & 0x7F));

    /* proceed to next sub-identifier */
    oid+= 1;
  }
 return Ok(());
}

/*
 * Returns octet count for length.
 *
 * @param length parameter length
 * @param octets_needed points to the return value
 */
pub fn 
snmp_asn1_enc_length_cnt(length: u16, octets_needed: &mut Vec<u8>)
{
  if (length < 0x80) {
    *octets_needed = 1;
  } else if (length < 0x100) {
    *octets_needed = 2;
  } else {
    *octets_needed = 3;
  }
}

/*
 * Returns octet count for an u32.
 *
 * @param value value to be encoded
 * @param octets_needed points to the return value
 *
 * @note ASN coded integers are _always_ signed. E.g. +0xFFFF is coded
 * as 0x00,0xFF,0xFF. Note the leading sign octet. A positive value
 * of 0xFFFFFFFF is preceded with 0x00 and the length is 5 octets!!
 */
pub fn 
snmp_asn1_enc_u32t_cnt(value: u32, octets_needed: &mut u16)
{
  if (value < 0x80) {
    *octets_needed = 1;
  } else if (value < 0x8000) {
    *octets_needed = 2;
  } else if (value < 0x800000) {
    *octets_needed = 3;
  } else if (value < 0x80000000) {
    *octets_needed = 4;
  } else {
    *octets_needed = 5;
  }
}

/*
 * Returns octet count for an i32.
 *
 * @param value value to be encoded
 * @param octets_needed points to the return value
 *
 * @note ASN coded integers are _always_ signed.
 */
pub fn 
snmp_asn1_enc_s32t_cnt(i32 value, octets_needed: &mut u16)
{
  if (value < 0) {
    value = !value;
  }
  if (value < 0x80L) {
    *octets_needed = 1;
  } else if (value < 0x8000L) {
    *octets_needed = 2;
  } else if (value < 0x800000L) {
    *octets_needed = 3;
  } else {
    *octets_needed = 4;
  }
}

/*
 * Returns octet count for an object identifier.
 *
 * @param oid points to object identifier array
 * @param oid_len object identifier array length
 * @param octets_needed points to the return value
 */
pub fn 
snmp_asn1_enc_oid_cnt(const u32 *oid, oid_len: u16, octets_needed: &mut u16)
{
  let sub_id: u32;

  *octets_needed = 0;
  if (oid_len > 1) {
    /* compressed prefix in one octet */
    (*octets_needed)+= 1;
    oid_len -= 2;
    oid += 2;
  }
  while (oid_len > 0) {
    oid_len -= 1;
    sub_id = *oid;

    sub_id >>= 7;
    (*octets_needed)+= 1;
    while (sub_id > 0) {
      sub_id >>= 7;
      (*octets_needed)+= 1;
    }
    oid+= 1;
  }
}

/*
 * Decodes a TLV from a pbuf stream.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param tlv returns decoded TLV
 * @return ERR_OK if successful, ERR_VAL if we can't decode
 */
pub fn 
snmp_asn1_dec_tlv(pbuf_stream: &mut snmp_pbuf_stream, tlv: &mut snmp_asn1_tlv)
{
  let data: u8;

  /* decode type first */
  PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
  tlv.type = data;

  if ((tlv.type & SNMP_ASN1_DATATYPE_MASK) == SNMP_ASN1_DATATYPE_EXTENDED) {
    /* extended format is not used by SNMP so we do not accept those values */
    return ERR_VAL;
  }
  tlv.type_len = 1;

  /* now, decode length */
  PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));

  if (data < 0x80) { /* short form */
    tlv.length_len = 1;
    tlv.value_len  = data;
  } else if (data > 0x80) { /* long form */
    length_bytes: u8 = data - 0x80;
    if (length_bytes > pbuf_stream.length) {
      return ERR_VAL;
    }
    tlv.length_len = length_bytes + 1; /* this byte + defined number of length bytes following */
    tlv.value_len = 0;

    while (length_bytes > 0) {
      /* we only support up to u16.maxvalue-1 (2 bytes) but have to accept leading zero bytes */
      if (tlv.value_len > 0xFF) {
        return ERR_VAL;
      }
      PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
      tlv.value_len <<= 8;
      tlv.value_len |= data;

      /* take care for special value used for indefinite length */
      if (tlv.value_len == 0xFFFF) {
        return ERR_VAL;
      }

      length_bytes -= 1;
    }
  } else { /* data == 0x80 indefinite length form */
    /* (not allowed for SNMP; RFC 1157, 3.2.2) */
    return ERR_VAL;
  }

 return Ok(());
}

/*
 * Decodes positive integer (counter, gauge, timeticks) into u32.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param len length of the coded integer field
 * @param value return host order integer
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) decode
 *
 * @note ASN coded integers are _always_ signed. E.g. +0xFFFF is coded
 * as 0x00,0xFF,0xFF. Note the leading sign octet. A positive value
 * of 0xFFFFFFFF is preceded with 0x00 and the length is 5 octets!!
 */
pub fn 
snmp_asn1_dec_u32t(pbuf_stream: &mut snmp_pbuf_stream, len: usize, u32 *value)
{
  let data: u8;

  if ((len > 0) && (len <= 5)) {
    PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));

    /* expecting sign bit to be zero, only  please! */
    if (((len == 5) && (data == 0x00)) || ((len < 5) && ((data & 0x80) == 0))) {
      *value = data;
      len -= 1;

      while (len > 0) {
        PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
        len -= 1;

        *value <<= 8;
        *value |= data;
      }

     return Ok(());
    }
  }

  return ERR_VAL;
}

/*
 * Decodes integer into i32.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param len length of the coded integer field
 * @param value return host order integer
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) decode
 *
 * @note ASN coded integers are _always_ signed!
 */
pub fn 
snmp_asn1_dec_s32t(pbuf_stream: &mut snmp_pbuf_stream, len: usize, i32 *value)
{
  let data: u8;

  if ((len > 0) && (len < 5)) {
    PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));

    if (data & 0x80) {
      /* negative, start from -1 */
      *value = -1;
      *value = (*value << 8) | data;
    } else {
      /* positive, start from 0 */
      *value = data;
    }
    len -= 1;
    /* shift in the remaining value */
    while (len > 0) {
      PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
      *value = (*value << 8) | data;
      len -= 1;
    }
   return Ok(());
  }

  return ERR_VAL;
}

/*
 * Decodes object identifier from incoming message into array of u32.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param len length of the coded object identifier
 * @param oid return decoded object identifier
 * @param oid_len return decoded object identifier length
 * @param oid_max_len size of oid buffer
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) decode
 */
pub fn 
snmp_asn1_dec_oid(pbuf_stream: &mut snmp_pbuf_stream, len: usize, u32 *oid, oid_len: &mut Vec<u8>, oid_max_len: u8)
{
  u32 *oid_ptr;
  let data: u8;

  *oid_len = 0;
  oid_ptr = oid;
  if (len > 0) {
    if (oid_max_len < 2) {
      return ERR_MEM;
    }

    PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
    len -= 1;

    /* first compressed octet */
    if (data == 0x2B) {
      /* (most) common case 1.3 (iso.org) */
      *oid_ptr = 1;
      oid_ptr+= 1;
      *oid_ptr = 3;
      oid_ptr+= 1;
    } else if (data < 40) {
      *oid_ptr = 0;
      oid_ptr+= 1;
      *oid_ptr = data;
      oid_ptr+= 1;
    } else if (data < 80) {
      *oid_ptr = 1;
      oid_ptr+= 1;
      *oid_ptr = data - 40;
      oid_ptr+= 1;
    } else {
      *oid_ptr = 2;
      oid_ptr+= 1;
      *oid_ptr = data - 80;
      oid_ptr+= 1;
    }
    *oid_len = 2;
  } else {
    /* accepting zero length identifiers e.g. for getnext operation. uncommon but valid */
   return Ok(());
  }

  while ((len > 0) && (*oid_len < oid_max_len)) {
    PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
    len -= 1;

    if ((data & 0x80) == 0x00) {
      /* sub-identifier uses single octet */
      *oid_ptr = data;
    } else {
      /* sub-identifier uses multiple octets */
      sub_id: u32 = (data & !0x80);
      while ((len > 0) && ((data & 0x80) != 0)) {
        PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
        len -= 1;

        sub_id = (sub_id << 7) + (data & !0x80);
      }

      if ((data & 0x80) != 0) {
        /* "more bytes following" bit still set at end of len */
        return ERR_VAL;
      }
      *oid_ptr = sub_id;
    }
    oid_ptr+= 1;
    (*oid_len)+= 1;
  }

  if (len > 0) {
    /* OID to long to fit in our buffer */
    return ERR_MEM;
  }

 return Ok(());
}

/*
 * Decodes (copies) raw data (ip-addresses, octet strings, opaque encoding)
 * from incoming message into array.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param len length of the coded raw data (zero is valid, e.g. empty string!)
 * @param buf return raw bytes
 * @param buf_len returns length of the raw return value
 * @param buf_max_len buffer size
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) decode
 */
pub fn 
snmp_asn1_dec_raw(pbuf_stream: &mut snmp_pbuf_stream, len: usize, buf: &mut Vec<u8>, buf_len: &mut u16, buf_max_len: u16)
{
  if (len > buf_max_len) {
    /* not enough dst space */
    return ERR_MEM;
  }
  *buf_len = len;

  while (len > 0) {
    PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, buf));
    buf+= 1;
    len -= 1;
  }

 return Ok(());
}


/*
 * Returns octet count for an u64_t.
 *
 * @param value value to be encoded
 * @param octets_needed points to the return value
 *
 * @note ASN coded integers are _always_ signed. E.g. +0xFFFF is coded
 * as 0x00,0xFF,0xFF. Note the leading sign octet. A positive value
 * of 0xFFFFFFFFFFFFFFFF is preceded with 0x00 and the length is 9 octets!!
 */
pub fn 
snmp_asn1_enc_u64t_cnt(u64_t value, octets_needed: &mut u16)
{
  /* check if high u32 is 0 */
  if ((value >> 32) == 0) {
    /* only low u32 is important */
    snmp_asn1_enc_u32t_cnt(value, octets_needed);
  } else {
    /* low u32 does not matter for length determination */
    snmp_asn1_enc_u32t_cnt((value >> 32), octets_needed);
    *octets_needed = *octets_needed + 4; /* add the 4 bytes of low u32 */
  }
}

/*
 * Decodes large positive integer (counter64) into 2x u32.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param len length of the coded integer field
 * @param value return 64 bit integer
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) decode
 *
 * @note ASN coded integers are _always_ signed. E.g. +0xFFFF is coded
 * as 0x00,0xFF,0xFF. Note the leading sign octet. A positive value
 * of 0xFFFFFFFFFFFFFFFF is preceded with 0x00 and the length is 9 octets!!
 */
pub fn 
snmp_asn1_dec_u64t(pbuf_stream: &mut snmp_pbuf_stream, len: usize, u64_t *value)
{
  let data: u8;

  if ((len > 0) && (len <= 9)) {
    PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));

    /* expecting sign bit to be zero, only  please! */
    if (((len == 9) && (data == 0x00)) || ((len < 9) && ((data & 0x80) == 0))) {
      *value = data;
      len -= 1;

      while (len > 0) {
        PBUF_OP_EXEC(snmp_pbuf_stream_read(pbuf_stream, &data));
        *value <<= 8;
        *value |= data;
        len -= 1;
      }

     return Ok(());
    }
  }

  return ERR_VAL;
}

/*
 * Encodes u64_t (counter64) into a pbuf chained ASN1 msg.
 *
 * @param pbuf_stream points to a pbuf stream
 * @param octets_needed encoding length (from snmp_asn1_enc_u32t_cnt())
 * @param value is the value to be encoded
 * @return ERR_OK if successful, ERR_ARG if we can't (or won't) encode
 *
 * @see snmp_asn1_enc_u64t_cnt()
 */
pub fn 
snmp_asn1_enc_u64t(pbuf_stream: &mut snmp_pbuf_stream, octets_needed: u16, u64_t value)
{
  if (octets_needed > 9) {
    return ERR_ARG;
  }
  if (octets_needed == 9) {
    /* not enough bits in 'value' add leading 0x00 */
    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, 0x00));
    octets_needed -= 1;
  }

  while (octets_needed > 1) {
    octets_needed -= 1;
    PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, (value >> (octets_needed << 3))));
  }

  /* always write at least one octet (also in case of value == 0) */
  PBUF_OP_EXEC(snmp_pbuf_stream_write(pbuf_stream, (value)));

 return Ok(());
}



