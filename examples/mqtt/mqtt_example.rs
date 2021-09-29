/*
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
 * This file is part of the lwIP TCP/IP stack.
 * 
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */






/* Define this to a compile-time IP address initialization
 * to connect anything else than IPv4 loopback
 */


// #define LWIP_MQTT_EXAMPLE_IPADDR_INIT = IPADDR4_INIT(IPADDR_LOOPBACK)
// #else
// #define LWIP_MQTT_EXAMPLE_IPADDR_INIT



// static LwipAddr mqtt_ip LWIP_MQTT_EXAMPLE_IPADDR_INIT;
// static mqtt_client_t* mqtt_client;

// const mqtt_client_info: MqttConnectClientInfo =
// {
//   "test",
//   NULL, //  user 
//   NULL, //  pass 
//   100,  //  keep alive 
//   NULL, //  will_topic 
//   NULL, //  will_msg 
//   0,    //  will_qos 
//   0     //  will_retain 

//   , NULL

// }

pub fn
mqtt_incoming_data_cb(arg: &mut Vec<u8>, data: &Vec<u8>, len: usize, flags: u8)
{
  let  client_info: MqttConnectClientInfo = arg;
  

  // printf("MQTT client \"%s\" data cb: len %d, flags %d\n", client_info.client_id,
  //         len, flags);
}

pub fn
mqtt_incoming_publish_cb(arg: &mut Vec<u8>, topic: &String, tot_len: u32)
{
  let client_info: mqtt_connect_client_info_t = arg;

  // printf("MQTT client \"%s\" publish cb: topic %s, len %d\n", client_info.client_id,
  //         topic, tot_len);
}

pub fn
mqtt_request_cb(arg: &mut Vec<u8>, err: err_t)
{
  let  client_info: &mqtt_connect_client_info_t = arg;

  printf("MQTT client \"%s\" request cb: err %d\n", client_info.client_id, err);
}

pub fn
mqtt_connection_cb(
  client: &mut mqtt_client_t, 
  arg: &mut Vec<u8>, 
  status: mqtt_connection_status_t)
{
  let client_info: mqtt_connect_client_info_t = arg;
  

  printf("MQTT client \"%s\" connection cb: status %d\n", client_info.client_id, status);

  if (status == MQTT_CONNECT_ACCEPTED) {
    mqtt_sub_unsub(client,
            "topic_qos1", 1,
            mqtt_request_cb, client_info,
            1);
    mqtt_sub_unsub(client,
            "topic_qos0", 0,
            mqtt_request_cb, client_info,
            1);
  }
}


pub fn 
mqtt_example_init()
{

  mqtt_client = mqtt_client_new();

  mqtt_set_inpub_callback(mqtt_client,
          mqtt_incoming_publish_cb,
          mqtt_incoming_data_cb,
          &mqtt_client_info);

  mqtt_client_connect(mqtt_client,
          &mqtt_ip, MQTT_PORT,
          mqtt_connection_cb, &mqtt_client_info,
          &mqtt_client_info);

}
