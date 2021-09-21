/*
 * @file
 * lwip Private MIB
 *
 * @todo create MIB file for this example
 * @note the lwip enterprise tree root (26381) is owned by the lwIP project.
 * It is NOT allowed to allocate new objects under this ID (26381) without our,
 * the lwip developers, permission!
 *
 * Please apply for your own ID with IANA: http://www.iana.org/numbers.html
 *
 * lwip        OBJECT IDENTIFIER ::= { enterprises 26381 }
 * example     OBJECT IDENTIFIER ::= { lwip 1 }
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
 */

/* Directory where the sensor files are */
// #define SENSORS_DIR           "w:\\sensors"
/* Set to 1 to read sensor values from files (in directory defined by SENSORS_DIR) */
pub const SENSORS_USE_FILES: u32 = 0;
/* Set to 1 to search sensor files at startup (in directory defined by SENSORS_DIR) */
pub const SENSORS_SEARCH_FILES: u32 = 0;

/* When not using & searching files, defines the number of sensors */
pub const SENSOR_COUNT: u32 = 4;

/*
  This example presents a table for a few (at most 10) sensors.
  Sensor detection takes place at initialization (once only).
  Sensors may and can not be added or removed after agent
  has started. Note this is only a limitation of this crude example,
  the agent does support dynamic object insertions and removals.

  You'll need to manually create a directory called "sensors" and
  a few single line text files with an integer temperature value.
  The files must be called [0..9].txt.

  ./sensors/0.txt [content: 20]
  ./sensors/3.txt [content: 75]

  The sensor values may be changed in runtime by editing the
  text files in the "sensors" directory.
*/

pub const SENSOR_MAX: u32 = 10;
pub const SENSOR_NAME_LEN: u32 = 20;

pub struct sensor_inf {
    pub num: u8,
    pub file: String,
    /* When not using files, contains the value of the sensor */
    pub letvalue: i32,
}

// static struct sensor_inf sensors[SENSOR_MAX];

// static i16      sensor_count_get_value(instance: &mut snmp_node_instance, value: &mut Vec<u8>);
// pub fn snmp_sensor_table_get_cell_instance( column: &mut u32,  row_oid: &mut u32, row_oid_len: u8, cell_instance: &mut snmp_node_instance) -> Result<(), LwipError>;pub fn snmp_sensor_table_get_cell_instance( column: &mut u32,  row_oid: &mut u32, row_oid_len: u8, cell_instance: &mut snmp_node_instance) -> Result<(), LwipError>
// static snmp_sensor_table_get_next_cell_instance: err_t( column: &mut u32, row_oid: &mut snmp_obj_id, cell_instance: &mut snmp_node_instance);
// static i16      sensor_table_get_value(instance: &mut snmp_node_instance, value: &mut Vec<u8>);
// pub fn snmp_sensor_table_set_value(instance: &mut snmp_node_instance, len: usize, value: &mut Vec<u8>) -> Result<(), LwipError>;

/* sensorentry .1.3.6.1.4.1.26381.1.1.1 (.level0.level1)
where level 0 is the table column (temperature/file name)
and level 1 the table row (sensor index) */
pub const sensor_table_columns: [snmp_table_col_def] = [
    snmp_table_col_def::new(1, SNMP_ASN1_TYPE_INTEGER, SNMP_NODE_INSTANCE_READ_WRITE),
    snmp_table_col_def::new(2, SNMP_ASN1_TYPE_OCTET_STRING, SNMP_NODE_INSTANCE_READ_ONLY),
];

/* sensortable .1.3.6.1.4.1.26381.1.1 */
// static const struct snmp_table_node sensor_table = SNMP_TABLE_CREATE(
//   1, sensor_table_columns,
//   sensor_table_get_cell_instance, sensor_table_get_next_cell_instance,
//   sensor_table_get_value, snmp_set_test_ok, sensor_table_set_value);

/* sensorcount .1.3.6.1.4.1.26381.1.2 */
// pub const sensor_count: snmp_scalar_node = SNMP_SCALAR_CREATE_NODE_READONLY(
//   2, SNMP_ASN1_TYPE_INTEGER, sensor_count_get_value);

/* example .1.3.6.1.4.1.26381.1 */
// static const struct snmp_node* const example_nodes[] = {
//   &sensor_table.node.node,
//   &sensor_count.node.node
// };
// static pub const example_node: snmp_tree_node = SNMP_CREATE_TREE_NODE(1, example_nodes);

// static const prvmib_base_oid: [u32;] = { 1,3,6,1,4,1,26381,1 };
// const struct snmp_mib mib_private = SNMP_MIB_CREATE(prvmib_base_oid, &example_node.node);

/* for reference: we could also have expressed it like this: */

/* lwip .1.3.6.1.4.1.26381 */
// static const struct snmp_node* const lwip_nodes[] = {
//   &example_node.node
// };
// static pub const lwip_node: snmp_tree_node = SNMP_CREATE_TREE_NODE(26381, lwip_nodes);

/* enterprises .1.3.6.1.4.1 */
// static const struct snmp_node* const enterprises_nodes[] = {
//   &lwip_node.node
// };
// static pub const enterprises_node: snmp_tree_node = SNMP_CREATE_TREE_NODE(1, enterprises_nodes);

/* private .1.3.6.1.4 */
// static const struct snmp_node* const private_nodes[] = {
//   &enterprises_node.node
// };
// static pub const private_root: snmp_tree_node = SNMP_CREATE_TREE_NODE(4, private_nodes);

// static const prvmib_base_oid: [u32;] = { 1,3,6,1,4 };
// const struct snmp_mib mib_private = SNMP_MIB_CREATE(prvmib_base_oid, &private_root.node);

/*
 * Initialises this private MIB before use.
 * @see main.c
 */
pub fn lwip_privmib_init() {
    let buf: &mut String;
    let ebuf: &mut String;
    let cp: &mut String;
    let bufsize: usize;
    let letnbytes: i32;
    let sb: stat;
    let mut dp: &mut dirent;
    let letfd: i32;
    /* SENSORS_USE_FILES && SENSORS_SEARCH_FILES */
    let i: u8;

    //memset(sensors, 0, sizeof(sensors));

    printf("SNMP private MIB start, detecting sensors.\n");

    /* look for sensors in sensors directory */
    fd = open(SENSORS_DIR, O_RDONLY);
    if (fd > -1) {
        fstat(fd, &sb);
        bufsize = sb.st_size;
        if (bufsize < sb.st_blksize) {
            bufsize = sb.st_blksize;
        }
        buf = malloc(bufsize);
        if (buf != None) {
            loop {
                let base: i32;

                nbytes = getdirentries(fd, buf, bufsize, &base);
                if (nbytes > 0) {
                    ebuf = buf + nbytes;
                    cp = buf;
                    while (cp < ebuf) {
                        dp = cp;
                        if (lwip_isdigit(dp.d_name[0])) {
                            let idx: char = dp.d_name[0] - '0';

                            sensors[idx].num = idx + 1;
                            strncpy(&sensors[idx].file[0], dp.d_name, SENSOR_NAME_LEN);
                            printf("%s\n", sensors[idx].file);
                        }
                        cp += dp.d_reclen;
                    }
                }
                if !(nbytes > 0) {
                    break;
                }
            }

            free(buf);
        }
        close(fd);
    }
    /* SENSORS_USE_FILES && SENSORS_SEARCH_FILES */
    // for (i = 0; i < SENSOR_COUNT; i+= 1) {
    //   sensors[i].num = (i + 1);
    //   snprintf(sensors[i].file, sizeof(sensors[i].file), "%d.txt", i);

    //   /* initialize sensor value to != zero */
    //   sensors[i].value = 11 * (i+1);

    // }
}

/* sensorcount .1.3.6.1.4.1.26381.1.2 */
pub fn sensor_count_get_value(instance: &mut snmp_node_instance, value: &mut Vec<u8>) -> usize {
    let count: usize = 0;
    let uint_ptr: &mut u32 = value;

    // for(count=0; count<LWIP_ARRAYSIZE(sensors); count+= 1) {
    //   if(sensors[count].num == 0) {
    //     *uint_ptr = count;
    //     return sizeof(*uint_ptr);
    //   }
    // }

    return 0;
}

/* sensortable .1.3.6.1.4.1.26381.1.1 */
/* list of allowed value ranges for incoming OID */
// pub const sensor_table_oid_ranges: [snmp_oid_range] = {
//   { 1, SENSOR_MAX+1 }
// };

pub fn sensor_table_get_cell_instance(
    column: &mut u32,
    row_oid: &mut u32,
    row_oid_len: u8,
    cell_instance: &mut snmp_node_instance,
) {
    let sensor_num: u32;
    let i: usize;

    /* check if incoming OID length and if values are in plausible range */
    if (!snmp_oid_in_range(
        row_oid,
        row_oid_len,
        sensor_table_oid_ranges,
        LWIP_ARRAYSIZE(sensor_table_oid_ranges),
    )) {
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    /* get sensor index from incoming OID */
    sensor_num = row_oid[0];

    /* find sensor with index */
    // for(i=0; i<LWIP_ARRAYSIZE(sensors); i+= 1) {
    //   if(sensors[i].num != 0) {
    //     if(sensors[i].num == sensor_num) {
    //       /* store sensor index for subsequent operations (get/test/set) */
    //       cell_instance.reference.u32 = i;
    //       return SNMP_ERR_NOERROR;
    //     }
    //   }
    // }

    /* not found */
    return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn sensor_table_get_next_cell_instance(
    column: &mut u32,
    row_oid: &mut snmp_obj_id,
    cell_instance: &mut snmp_node_instance,
) {
    let i: usize;
    let state: snmp_next_oid_state;
    let result_temp: Vec<u32>;

    /* init struct to search next oid */
    snmp_next_oid_init(
        &state,
        row_oid.id,
        row_oid.len,
        result_temp,
        LWIP_ARRAYSIZE(sensor_table_oid_ranges),
    );

    /* iterate over all possible OIDs to find the next one */
    // for(i=0; i<LWIP_ARRAYSIZE(sensors); i+= 1) {
    //   if(sensors[i].num != 0) {
    //     test_oid: [u32;LWIP_ARRAYSIZE(sensor_table_oid_ranges)];

    //     test_oid[0] = sensors[i].num;

    //     /* check generated OID: is it a candidate for the next one? */
    //     snmp_next_oid_check(&state, test_oid, LWIP_ARRAYSIZE(sensor_table_oid_ranges), i);
    //   }
    // }

    /* did we find a next one? */
    if (state.status == SNMP_NEXT_OID_STATUS_SUCCESS) {
        snmp_oid_assign(row_oid, state.next_oid, state.next_oid_len);
        /* store sensor index for subsequent operations (get/test/set) */
        cell_instance.reference.u32 = LWIP_CONST_CAST(u32, state.reference);
        return SNMP_ERR_NOERROR;
    }

    /* not found */
    return SNMP_ERR_NOSUCHINSTANCE;
}

pub fn sensor_table_get_value(instance: &mut snmp_node_instance, value: &mut Vec<u8>) {
    let i: u32 = instance.reference.u32;
    let temperature = value;

    match (SNMP_TABLE_GET_COLUMN_FROM_OID(instance.instance_oid.id)) {
        1 => {
            /* sensor value */

            // FILE* sensf;
            let sensf: &mut FILE;

            // let senspath: [char;sizeof(SENSORS_DIR)+1+SENSOR_NAME_LEN+1] = SENSORS_DIR"/";

            strncpy(
                &senspath[sizeof(SENSORS_DIR)],
                sensors[i].file,
                SENSOR_NAME_LEN,
            );
            sensf = fopen(senspath, "r");
            if (sensf != None) {
                // fscanf(sensf,"%"S32_F,temperature);
                fclose(sensf);
            }
            /* SENSORS_USE_FILES */
            *temperature = sensors[i].value;

            return sizeof;
        }
        2 => {
            /* file name */
            MEMCPY(value, sensors[i].file, strlen(sensors[i].file));
            return strlen(sensors[i].file);
        }
        _ => {
            return 0;
        }
    }
}

pub fn sensor_table_set_value(instance: &mut snmp_node_instance, len: usize, value: &mut Vec<u8>) {
    let i: u32 = instance.reference.u32;
    let temperature = value;

    // FILE* sensf;
    // senspath: [u8;sizeof(SENSORS_DIR)+1+SENSOR_NAME_LEN+1] = SENSORS_DIR"/";

    strncpy(
        &senspath[sizeof(SENSORS_DIR)],
        sensors[i].file,
        SENSOR_NAME_LEN,
    );
    sensf = fopen(senspath, "w");
    if (sensf != None) {
        // fprintf(sensf, "%"S32_F, *temperature);
        fclose(sensf);
    }
    /* SENSORS_USE_FILES */
    sensors[i].value = *temperature;

    return SNMP_ERR_NOERROR;
}
