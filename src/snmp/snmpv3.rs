/*
 * @file
 * Additional SNMPv3 functionality RFC3414 and RFC3826.
 */

/*
 * Copyright (c) 2016 Elias Oenal.
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
 * Author: Elias Oenal <lwip@eliasoenal.com>
 */












#define SNMP_MAX_TIME_BOOT 2147483647UL

/* Call this if engine has been changed. Has to reset boots, see below */
pub fn 
snmpv3_engine_id_changed()
{
  snmpv3_set_engine_boots(0);
}

/* According to RFC3414 2.2.2.
 *
 * The number of times that the SNMP engine has
 * (re-)initialized itself since snmpEngineID
 * was last configured.
 */
i32
snmpv3_get_engine_boots_internal()
{
  if (snmpv3_get_engine_boots() == 0 ||
      snmpv3_get_engine_boots() < SNMP_MAX_TIME_BOOT) {
    return snmpv3_get_engine_boots();
  }

  snmpv3_set_engine_boots(SNMP_MAX_TIME_BOOT);
  return snmpv3_get_engine_boots();
}

/* RFC3414 2.2.2.
 *
 * Once the timer reaches 2147483647 it gets reset to zero and the
 * engine boot ups get incremented.
 */
i32
snmpv3_get_engine_time_internal()
{
  if (snmpv3_get_engine_time() >= SNMP_MAX_TIME_BOOT) {
    snmpv3_reset_engine_time();

    if (snmpv3_get_engine_boots() < SNMP_MAX_TIME_BOOT - 1) {
      snmpv3_set_engine_boots(snmpv3_get_engine_boots() + 1);
    } else {
      snmpv3_set_engine_boots(SNMP_MAX_TIME_BOOT);
    }
  }

  return snmpv3_get_engine_time();
}



/* This function ignores the byte order suggestion in RFC3414
 * since it simply doesn't influence the effectiveness of an IV.
 *
 * Implementing RFC3826 priv param algorithm if LWIP_RAND is available.
 *
 * @todo: This is a potential thread safety issue.
 */
pub fn 
snmpv3_build_priv_param(u8 *priv_param)
{

  static init: u8;
  static priv1: u32, priv2;

  /* Lazy initialisation */
  if (init == 0) {
    init = 1;
    priv1 = LWIP_RAND();
    priv2 = LWIP_RAND();
  }

  SMEMCPY(&priv_param[0], &priv1, sizeof(priv1));
  SMEMCPY(&priv_param[4], &priv2, sizeof(priv2));

  /* Emulate 64bit increment */
  priv1++;
  if (!priv1) { /* Overflow */
    priv2++;
  }
#else /* Based on RFC3414 */
  static ctr: u32;
  boots: u32 = snmpv3_get_engine_boots_internal();
  SMEMCPY(&priv_param[0], &boots, 4);
  SMEMCPY(&priv_param[4], &ctr, 4);
  ctr++;

  return ERR_OK;
}



