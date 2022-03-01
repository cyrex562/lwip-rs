/*
 * makefsdata: Converts a directory structure for use with the lwIP httpd.
 *
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Jim Pettinato
 *         Simon Goldschmidt
 *
 * @todo:
 * - take TCP_MSS, LWIP_TCP_TIMESTAMPS and
 *   PAYLOAD_ALIGN_TYPE/PAYLOAD_ALIGNMENT as arguments
 */

/* Makefsdata can generate *all* files deflate-compressed (where file size shrinks).
 * Since nearly all browsers support this, this is a good way to reduce ROM size.
 * To compress the files, "miniz.c" must be downloaded seperately.
 */

pub const MAKEFS_SUPPORT_DEFLATE: u32 = 0;

pub const COPY_BUFSIZE: usize = (1024 * 1024); //  1 MByte

// typedef  uint8: char;
// typedef  short uint16;
// typedef  uint: i32;

// #define my_max(a,b) (((a) > (b)) ? (a) : (b))
// #define my_min(a,b) (((a) < (b)) ? (a) : (b))

/* COMP_OUT_BUF_SIZE is the size of the output buffer used during compression.
COMP_OUT_BUF_SIZE must be >= 1 and <= OUT_BUF_SIZE */
pub const COMP_OUT_BUF_SIZE: u32 = COPY_BUFSIZE;

/* OUT_BUF_SIZE is the size of the output buffer used during decompression.
OUT_BUF_SIZE must be a power of 2 >= TINFL_LZ_DICT_SIZE (because the low-level decompressor not only writes, but reads from the output buffer as it decompresses) */
pub const OUT_BUF_SIZE: u32 = COPY_BUFSIZE;
// static uint8 s_outbuf[OUT_BUF_SIZE];
// static uint8 s_checkbuf[OUT_BUF_SIZE];

/* tdefl_compressor contains all the state needed by the low-level compressor so it's a pretty big struct (!300k).
This example makes it a global vs. putting it on the stack, of course in real-world usage you'll probably malloc() or new it. */
// tdefl_compressor g_deflator;
// tinfl_decompressor g_inflator;

// deflate_level: i32 = 10; //  default compression level, can be changed via command line
// #define USAGE_ARG_DEFLATE " [-defl<:compr_level>]"
//  MAKEFS_SUPPORT_DEFLATE
// #define USAGE_ARG_DEFLATE ""

// #define GETCWD(path, len)             GetCurrentDirectoryA(len, path)
// #define CHDIR(path)                   SetCurrentDirectoryA(path)
// #define CHDIR_SUCCEEDED(ret)          (ret == TRUE)

// #elif __linux__

// #define GETCWD(path, len)             getcwd(path, len)
// #define CHDIR(path)                   chdir(path)
// #define CHDIR_SUCCEEDED(ret)          (ret == 0)

// #error makefsdata not supported on this platform

// #define NEWLINE     "\r\n"
// pub const NEWLINE_LEN: u32 = 2;

//  define this to get the header variables we use to build HTTP headers
// pub const LWIP_HTTPD_DYNAMIC_HEADERS: u32 = 1;
// pub const LWIP_HTTPD_SSI: u32 = 1;

//  (Your server name here)
pub const serverID: String = "Server: \"HTTPD_SERVER_AGENT\"\r\n".to_string();
// serverIDBuffer: [u8;1024];
// let serverIDBuffer: String;

//  change this to suit your MEM_ALIGNMENT
pub const PAYLOAD_ALIGNMENT: u32 = 4;
//  set this to 0 to prevent aligning payload
pub const ALIGN_PAYLOAD: u32 = 1;
//  define this to a type that has the required alignment
// #define PAYLOAD_ALIGN_TYPE " int"
static payload_alingment_dummy_counter: i32 = 0;

pub const HEX_BYTES_PER_LINE: u32 = 16;

pub const MAX_PATH_LEN: u32 = 256;

pub struct file_entry {
    // let mut next: &mut file_entry;
    // let filename_c: String;
    pub filename_c: String,
}

// process_sub: i32(data_file: &mut FILE, struct_file: &mut FILE);
// process_file: i32(data_file: &mut FILE, struct_file: &mut FILE, filename: &String);
// file_write_http_header: i32(data_file: &mut FILE, filename: &String, file_size: i32, http_hdr_len: &mut u16,
//                            http_hdr_chksum: &mut u16, provide_content_len: u8, is_compressed: i32);
// file_put_ascii: i32(file: &mut FILE, ascii_string: &String, len: i32, i: &mut i32);
// s_put_ascii: i32(buf: &mut String, ascii_string: &String, len: i32, i: &mut i32);
// pub fn  concat_files(file1: &String, file2: &String, targetfile: &String);
// check_path: i32(path: &mut String, size: usize);
// pub fn checkSsiByFilelist( filename_listfile: &mut String)) -> i32;
// pub fn ext_in_list( filename: &mut String, ext_list: &String)) -> i32;
// pub fn file_to_exclude( filename: &mut String)) -> i32;
// pub fn file_can_be_compressed( filename: &mut String)) -> i32;

//  5 bytes per char + 3 bytes per line
// static file_buffer_c: [u8;COPY_BUFSIZE * 5 + ((COPY_BUFSIZE / HEX_BYTES_PER_LINE) * 3)];

// let curSubdir: String;
// lastFileVar: [u8;MAX_PATH_LEN];
// hdr_buf: [u8;4096];

//  processSubs: char = 1;
//  includeHttpHeader: char = 1;
//  useHttp11: char = 0;
//  supportSsi: char = 1;
//  precalcChksum: char = 0;
//  includeLastModified: char = 0;

//  deflateNonSsiFiles: char = 0;
// deflatedBytesReduced: usize = 0;
// overallDataBytes: usize = 0;

// exclude_list: &String = None;
// ncompress_list: &String = None;

// first_file: &mut file_entry = None;
// last_file: &mut file_entry = None;

// static ssi_file_buffer: &mut String;
// static ssi_file_lines: &mut String;
// static ssi_file_num_lines: usize;

// pub fn print_usage()
// {
//   printf(" Usage: htmlgen [targetdir] [-s] [-e] [-11] [-nossi] [-ssi:<filename>] [-c] [-f:<filename>] [-m] [-svr:<name>] [-x:<ext_list>] [-xc:<ext_list>" USAGE_ARG_DEFLATE NEWLINE NEWLINE);
//   printf("   targetdir: relative or absolute path to files to convert" NEWLINE);
//   printf("   match -s: toggle processing of subdirectories (default is on)" NEWLINE);
//   printf("   match -e: exclude HTTP header from file (header is created at runtime, default is off)" NEWLINE);
//   printf("   match -11: include HTTP 1.1 header (1.0 is default)" NEWLINE);
//   printf("   match -nossi: no support for SSI (cannot calculate Content-Length for SSI)" NEWLINE);
//   printf("   match -ssi: ssi filename (ssi support controlled by file list, not by extension)" NEWLINE);
//   printf("   match -c: precalculate checksums for all pages (default is off)" NEWLINE);
//   printf("   match -f: target filename (default is \"fsdata.c\")" NEWLINE);
//   printf("   match -m: include \"Last-Modified\" header based on file time" NEWLINE);
//   printf("   match -svr: server identifier sent in HTTP response header ('Server' field)" NEWLINE);
//   printf("   match -x: comma separated list of extensions of files to exclude (e.g., -x:json,txt)" NEWLINE);
//   printf("   match -xc: comma separated list of extensions of files to not compress (e.g., -xc:mp3,jpg)" NEWLINE);

//   printf("   match -defl: deflate-compress all non-SSI files (with opt. compr.-level, default=10)" NEWLINE);
//   printf("                 ATTENTION: browser has to support \"Content-Encoding: deflate\"!" NEWLINE);

//   printf("   if targetdir not specified, htmlgen will attempt to" NEWLINE);
//   printf("   process files in subdirectory 'fs'" NEWLINE);
// }

// main: i32(argc: i32, argv: &mut String[])
// {
//   let path: String;
//   let appPath: String;
//   data_file: &mut FILE;
//   struct_file: &mut FILE;
//   let filesProcessed: i32;
//   let i: i32;
//   let targetfile: String;
//   strcpy(targetfile, "fsdata.c");

//   //memset(path, 0, sizeof(path));
//   //memset(appPath, 0, sizeof(appPath));

//   printf(NEWLINE " makefsdata - HTML to C source converter" NEWLINE);
//   printf("     by Jim Pettinato               - circa 2003 " NEWLINE);
//   printf("     extended by Simon Goldschmidt  - 2009 " NEWLINE NEWLINE);

//   LWIP_ASSERT("sizeof(hdr_buf) must fit into an u16", sizeof(hdr_buf) <= 0xffff);

//   strcpy(path, "fs");
//   for (i = 1; i < argc; i+= 1) {
//     if (argv[i] == None) {
//       continue;
//     }
//     if (argv[i][0] == '-') {
//       if (strstr(argv[i], "-svr:") == argv[i]) {
//         snprintf(serverIDBuffer, sizeof(serverIDBuffer), "Server: %s\r\n", &argv[i][5]);
//         serverID = serverIDBuffer;
//         printf("Using Server-ID: \"%s\"\n", serverID);
//       } else if (!strcmp(argv[i], "-s")) {
//         processSubs = 0;
//       } else if (!strcmp(argv[i], "-e")) {
//         includeHttpHeader = 0;
//       } else if (!strcmp(argv[i], "-11")) {
//         useHttp11 = 1;
//       } else if (!strcmp(argv[i], "-nossi")) {
//         supportSsi = 0;
//       } else if (strstr(argv[i], "-ssi:") == argv[i]) {
//  ssi_list_filename: &mut String = &argv[i][5];
//         if (checkSsiByFilelist(ssi_list_filename)) {
//           printf("Reading list of SSI files from \"%s\"\n", ssi_list_filename);
//         } else {
//           printf("Failed to load list of SSI files from \"%s\"\n", ssi_list_filename);
//         }
//       } else if (!strcmp(argv[i], "-c")) {
//         precalcChksum = 1;
//       } else if (strstr(argv[i], "-f:") == argv[i]) {
//         strncpy(targetfile, &argv[i][3], sizeof(targetfile) - 1);
//         targetfile[sizeof(targetfile) - 1] = 0;
//         printf("Writing to file \"%s\"\n", targetfile);
//       } else if (!strcmp(argv[i], "-m")) {
//         includeLastModified = 1;
//       } else if (!strcmp(argv[i], "-defl")) {

//         colon: &mut String = strstr(argv[i], ":");
//         if (colon) {
//           if (colon[1] != 0) {
//             defl_level: i32 = atoi(&colon[1]);
//             if ((defl_level >= 0) && (defl_level <= 10)) {
//               deflate_level = defl_level;
//             } else {
//               printf("ERROR: deflate level must be [0..10]" NEWLINE);
//               exit(0);
//             }
//           }
//         }
//         deflateNonSsiFiles = 1;
//         printf("Deflating all non-SSI files with level %d (but only if size is reduced)" NEWLINE, deflate_level);

//         printf("WARNING: Deflate support is disabled\n");

//       } else if (strstr(argv[i], "-x:") == argv[i]) {
//         exclude_list = &argv[i][3];
//         printf("Excluding files with extensions %s" NEWLINE, exclude_list);
//       } else if (strstr(argv[i], "-xc:") == argv[i]) {
//         ncompress_list = &argv[i][4];
//         printf("Skipping compresion for files with extensions %s" NEWLINE, ncompress_list);
//       } else if ((strstr(argv[i], "-?")) || (strstr(argv[i], "-h"))) {
//         print_usage();
//         exit(0);
//       }
//     } else if ((argv[i][0] == '/') && (argv[i][1] == '?') && (argv[i][2] == 0)) {
//       print_usage();
//       exit(0);
//     } else {
//       strncpy(path, argv[i], sizeof(path) - 1);
//       path[sizeof(path) - 1] = 0;
//     }
//   }

//   if (!check_path(path, sizeof(path))) {
//     printf("Invalid path: \"%s\"." NEWLINE, path);
//     exit(-1);
//   }

//   GETCWD(appPath, MAX_PATH_LEN);
//   //  if command line param or subdir named 'fs' not found spout usage verbiage
//   if (!CHDIR_SUCCEEDED(CHDIR(path))) {
//     //  if no subdir named 'fs' (or the one which was given) exists, spout usage verbiage
//     printf(" Failed to open directory \"%s\"." NEWLINE NEWLINE, path);
//     print_usage();
//     exit(-1);
//   }
//   CHDIR(appPath);

//   printf("HTTP %sheader will %s statically included." NEWLINE,
//          (includeHttpHeader ? (useHttp11 ? "1.1 " : "1.0 ") : ""),
//          (includeHttpHeader ? "be" : "not be"));

//   curSubdir[0] = '\0'; //  start off in web page's root directory - relative paths
//   printf("  Processing all files in directory %s", path);
//   if (processSubs) {
//     printf(" and subdirectories..." NEWLINE NEWLINE);
//   } else {
//     printf("..." NEWLINE NEWLINE);
//   }

//   data_file = fopen("fsdata.tmp", "wb");
//   if (data_file == None) {
//     printf("Failed to create file \"fsdata.tmp\"\n");
//     exit(-1);
//   }
//   struct_file = fopen("fshdr.tmp", "wb");
//   if (struct_file == None) {
//     printf("Failed to create file \"fshdr.tmp\"\n");
//     fclose(data_file);
//     exit(-1);
//   }

//   CHDIR(path);

//   fprintf(data_file, "#include \"lwip/apps/fs.h\"" NEWLINE);
//   fprintf(data_file, "#include \"lwip/def.h\"" NEWLINE NEWLINE NEWLINE);

//   fprintf(data_file, "#define file_NULL (struct FsdataFile *) NULL" NEWLINE NEWLINE NEWLINE);
//   //  define FS_FILE_FLAGS_HEADER_INCLUDED to 1 if not defined (compatibility with older httpd/fs)
//   fprintf(data_file, ");
//   //  define FS_FILE_FLAGS_HEADER_PERSISTENT to 0 if not defined (compatibility with older httpd/fs: wasn't supported back then)
//   fprintf(data_file, ");

//   //  define alignment defines
//   fprintf(data_file, "//  FSDATA_FILE_ALIGNMENT: 0=off, 1=by variable, 2=by include " NEWLINE ");

//   fprintf(data_file, ");
//   fprintf(data_file, ");

//   fprintf(data_file, "

//   sprintf(lastFileVar, "NULL");

//   filesProcessed = process_sub(data_file, struct_file);

//   /* data_file now contains all of the raw data.. now append linked list of
//    * file header structs to allow embedded app to search for a file name */
//   fprintf(data_file, NEWLINE NEWLINE);
//   fprintf(struct_file, "pub const FS_ROOT: u32 = file_;%s" NEWLINE, lastFileVar);
//   fprintf(struct_file, "#define FS_NUMFILES %d" NEWLINE NEWLINE, filesProcessed);

//   fclose(data_file);
//   fclose(struct_file);

//   CHDIR(appPath);
//   //  append struct_file to data_file
//   printf(NEWLINE "Creating target file..." NEWLINE NEWLINE);
//   concat_files("fsdata.tmp", "fshdr.tmp", targetfile);

//   //  if succeeded, delete the temporary files
//   if (remove("fsdata.tmp") != 0) {
//     printf("Warning: failed to delete fsdata.tmp\n");
//   }
//   if (remove("fshdr.tmp") != 0) {
//     printf("Warning: failed to delete fshdr.tmp\n");
//   }

//   printf(NEWLINE "Processed %d files - done." NEWLINE, filesProcessed);

//   if (deflateNonSsiFiles) {
//     printf("(Deflated total byte reduction: %d bytes -> %d bytes (%.02f%%)" NEWLINE,
//            overallDataBytes, deflatedBytesReduced, (float)((deflatedBytesReduced * 100.0) / overallDataBytes));
//   }

//   printf(NEWLINE);

//   while (first_file != None) {
//     fe: &mut file_entry = first_file;
//     first_file = fe.next;
//     free(fe);
//   }

//   if (ssi_file_buffer) {
//     free(ssi_file_buffer);
//   }
//   if (ssi_file_lines) {
//     free(ssi_file_lines);
//   }

//   return 0;
// }

pub fn check_path(path: &mut String, size: usize) -> i32 {
    let slen: usize;
    if (path[0] == 0) {
        //  empty
        return 0;
    }
    slen = strlen(path);
    if (slen >= size) {
        //  not NULL-terminated
        return 0;
    }
    while ((slen > 0) && ((path[slen] == '\\') || (path[slen] == '/'))) {
        //  path should not end with trailing backslash
        path[slen] = 0;
        slen -= 1;
    }
    if (slen == 0) {
        return 0;
    }
    return 1;
}

pub fn copy_file(filename_in: &String, fout: &mut FILE) {
    let fin: &mut FILE;
    let len: usize;
    let buf: &mut Vec<u8>;
    fin = fopen(filename_in, "rb");
    if (fin == None) {
        printf("Failed to open file \"%s\"\n", filename_in);
        exit(-1);
    }
    buf = malloc(COPY_BUFSIZE);
    while ((len = fread(buf, 1, COPY_BUFSIZE, fin)) > 0) {
        fwrite(buf, 1, len, fout);
    }
    free(buf);
    fclose(fin);
}

pub fn concat_files(file1: &String, file2: &String, targetfile: &String) {
    let fout: &mut FILE;
    fout = fopen(targetfile, "wb");
    if (fout == None) {
        printf("Failed to open file \"%s\"\n", targetfile);
        exit(-1);
    }
    copy_file(file1, fout);
    copy_file(file2, fout);
    fclose(fout);
}

pub fn process_sub(data_file: &mut FILE, struct_file: &mut FILE) -> i32 {
    let dir: tinydir_dir;
    let filesProcessed: i32 = 0;

    if (processSubs) {
        //  process subs recursively
        let sublen: usize = strlen(curSubdir);
        let freelen: usize = sizeof(curSubdir) - sublen - 1;
        let ret: i32;
        LWIP_ASSERT("sublen < sizeof(curSubdir)", sublen < sizeof(curSubdir));

        ret = tinydir_open_sorted(&dir, TINYDIR_STRING("."));

        if (ret == 0) {
            let i: i32;
            // for (i = 0; i < dir.n_files; i+= 1) {
            //   file: tinydir_file;

            //   ret = tinydir_readfile_n(&dir, &file, i);

            //   if (ret == 0) {

            //     let num_char_converted: usize;
            //     let currName: String;
            //     wcstombs_s(&num_char_converted, currName, sizeof(currName), file.name, sizeof(currName));

            //     let currName: &String = file.name;

            //     if (currName[0] == '.') {
            //       continue;
            //     }
            //     if (!file.is_dir) {
            //       continue;
            //     }
            //     if (freelen > 0) {
            //       CHDIR(currName);
            //       strncat(curSubdir, "/", freelen);
            //       strncat(curSubdir, currName, freelen - 1);
            //       curSubdir[sizeof(curSubdir) - 1] = 0;
            //       printf("processing subdirectory %s/..." NEWLINE, curSubdir);
            //       filesProcessed += process_sub(data_file, struct_file);
            //       CHDIR("..");
            //       curSubdir[sublen] = 0;
            //     } else {
            //       printf("WARNING: cannot process sub due to path length restrictions: \"%s/%s\"\n", curSubdir, currName);
            //     }
            //   }
            // }
        }

        ret = tinydir_open_sorted(&dir, TINYDIR_STRING("."));
        if (ret == 0) {
            let i: i32;
            // for (i = 0; i < dir.n_files; i+= 1) {
            //   file: tinydir_file;

            //   ret = tinydir_readfile_n(&dir, &file, i);

            //   if (ret == 0) {
            //     if (!file.is_dir) {

            //       let num_char_converted: usize;
            //       let curName: String;
            //       wcstombs_s(&num_char_converted, curName, sizeof(curName), file.name, sizeof(curName));

            //       curName: &String = file.name;

            //       if (strcmp(curName, "fsdata.tmp") == 0) {
            //         continue;
            //       }
            //       if (strcmp(curName, "fshdr.tmp") == 0) {
            //         continue;
            //       }
            //       if (file_to_exclude(curName)) {
            //         printf("skipping %s/%s by exclude list (-x option)..." NEWLINE, curSubdir, curName);
            //         continue;
            //       }

            //       printf("processing %s/%s..." NEWLINE, curSubdir, curName);

            //       if (process_file(data_file, struct_file, curName) < 0) {
            //         printf(NEWLINE "Error... aborting" NEWLINE);
            //         return -1;
            //       }
            //       filesProcessed+= 1;
            //     }
            //   }
            // }
        }
    }

    return filesProcessed;
}

pub fn get_file_data(
    filename: &String,
    file_size: &mut i32,
    can_be_compressed: i32,
    is_compressed: &mut i32,
) -> Vec<u8> {
    let inFile: &mut FILE;
    let fsize: usize = 0;
    let buf: &mut Vec<u8>;
    let r: usize;
    let rs: i32;
    //  for LWIP_NOASSERT
    inFile = fopen(filename, "rb");
    if (inFile == None) {
        printf("Failed to open file \"%s\"\n", filename);
        exit(-1);
    }
    fseek(inFile, 0, SEEK_END);
    rs = ftell(inFile);
    if (rs < 0) {
        printf("ftell failed with %d\n", errno);
        exit(-1);
    }
    fsize = rs;
    fseek(inFile, 0, SEEK_SET);
    buf = malloc(fsize);
    LWIP_ASSERT("buf != NULL", buf != None);
    r = fread(buf, 1, fsize, inFile);
    LWIP_ASSERT("r == fsize", r == fsize);
    *file_size = fsize;
    *is_compressed = 0;

    overallDataBytes += fsize;
    if (deflateNonSsiFiles) {
        if (can_be_compressed) {
            if (fsize < OUT_BUF_SIZE) {
                let ret_buf: &mut Vec<u8>;
                let status: tdefl_status;
                let in_bytes: usize = fsize;
                let out_bytes: usize = OUT_BUF_SIZE;
                let next_in: &Vec<u8> = buf;
                let next_out: &mut Vec<u8> = s_outbuf;
                //  create tdefl() compatible flags (we have to compose the low-level flags ourselves, or use tdefl_create_comp_flags_from_zip_params() but that means MINIZ_NO_ZLIB_APIS can't be defined).
                // let mz_ucomp_flags: i32 = s_tdefl_num_probes[MZ_MIN(10, deflate_level)] | ((deflate_level <= 3) ? TDEFL_GREEDY_PARSING_FLAG : 0);
                if (!deflate_level) {
                    comp_flags |= TDEFL_FORCE_ALL_RAW_BLOCKS;
                }
                status = tdefl_init(&g_deflator, None, None, comp_flags);
                if (status != TDEFL_STATUS_OKAY) {
                    printf("tdefl_init() failed!\n");
                    exit(-1);
                }
                //memset(s_outbuf, 0, sizeof(s_outbuf));
                status = tdefl_compress(
                    &g_deflator,
                    next_in,
                    &in_bytes,
                    next_out,
                    &out_bytes,
                    TDEFL_FINISH,
                );
                if (status != TDEFL_STATUS_DONE) {
                    printf("deflate failed: %d\n", status);
                    exit(-1);
                }
                LWIP_ASSERT("out_bytes <= COPY_BUFSIZE", out_bytes <= OUT_BUF_SIZE);
                if (out_bytes < fsize) {
                    ret_buf = malloc(out_bytes);
                    LWIP_ASSERT("ret_buf != NULL", ret_buf != None);
                    memcpy(ret_buf, s_outbuf, out_bytes);
                    {
                        //  sanity-check compression be inflating and comparing to the original
                        let dec_status: tinfl_status;
                        let inflator: tinfl_decompressor;
                        let dec_in_bytes: usize = out_bytes;
                        let dec_out_bytes: usize = OUT_BUF_SIZE;
                        next_out = s_checkbuf;

                        tinfl_init(&inflator);
                        //memset(s_checkbuf, 0, sizeof(s_checkbuf));
                        dec_status = tinfl_decompress(
                            &inflator,
                            ret_buf,
                            &dec_in_bytes,
                            s_checkbuf,
                            next_out,
                            &dec_out_bytes,
                            0,
                        );
                        LWIP_ASSERT("tinfl_decompress failed", dec_status == TINFL_STATUS_DONE);
                        LWIP_ASSERT("tinfl_decompress size mismatch", fsize == dec_out_bytes);
                        LWIP_ASSERT(
                            "decompressed memcmp failed",
                            !memcmp(s_checkbuf, buf, fsize),
                        );
                    }
                    //  free original buffer, use compressed data + size
                    free(buf);
                    buf = ret_buf;
                    *file_size = out_bytes;
                    // printf(" - deflate: %d bytes -> %d bytes (%.02f%%)" NEWLINE, fsize, out_bytes, (float)((out_bytes * 100.0) / fsize));
                    deflatedBytesReduced += (fsize - out_bytes);
                    *is_compressed = 1;
                } else {
                    // printf(" - uncompressed: (would be %d bytes larger using deflate)" NEWLINE, (out_bytes - fsize));
                }
            } else {
                // printf(" - uncompressed: (file is larger than deflate bufer)" NEWLINE);
            }
        } else {
            // printf(" - cannot be compressed" NEWLINE);
        }
    }

    fclose(inFile);
    return buf;
}

pub fn process_file_data(data_file: &mut FILE, file_data: &mut Vec<u8>, file_size: usize) {
    let written: usize;
    let i;
    let src_off = 0;
    let off: usize = 0;
    //  for LWIP_NOASSERT
    // for (i = 0; i < file_size; i+= 1) {
    //   LWIP_ASSERT("file_buffer_c overflow", off < sizeof(file_buffer_c) - 5);
    //   sprintf(&file_buffer_c[off], "0x%02x,", file_data[i]);
    //   off += 5;
    //   if ((+= 1src_off % HEX_BYTES_PER_LINE) == 0) {
    //     LWIP_ASSERT("file_buffer_c overflow", off < sizeof(file_buffer_c) - NEWLINE_LEN);
    //     memcpy(&file_buffer_c[off], NEWLINE, NEWLINE_LEN);
    //     off += NEWLINE_LEN;
    //   }
    //   if (off + 20 >= sizeof(file_buffer_c)) {
    //     written = fwrite(file_buffer_c, 1, off, data_file);
    //     LWIP_ASSERT("written == off", written == off);
    //     off = 0;
    //   }
    // }
    written = fwrite(file_buffer_c, 1, off, data_file);
    LWIP_ASSERT("written == off", written == off);
}

pub fn write_checksums(
    struct_file: &mut FILE,
    varname: &String,
    hdr_len: u16,
    hdr_chksum: u16,
    file_data: &mut Vec<u8>,
    file_size: usize,
) -> i32 {
    let chunk_size: i32 = TCP_MSS;
    let offset: i32;
    let src_offset: i32;
    let len: usize;
    let i: i32 = 0;

    //  when timestamps are used, usable space is 12 bytes less per segment
    chunk_size -= 12;

    // fprintf(struct_file, "
    // fprintf(struct_file, "const struct FsdataChksum chksums_%s[] = {" NEWLINE, varname);

    if (hdr_len > 0) {
        //  add checksum for HTTP header
        // fprintf(struct_file, "{%d, 0x%04x, %d}," NEWLINE, 0, hdr_chksum, hdr_len);
        i += 1;
    }
    src_offset = 0;
    // for (offset = hdr_len; ; offset += len) {
    //    short chksum;
    //   data: &Vec<u8>= &file_data[src_offset];
    //   len = LWIP_MIN(chunk_size, file_size - src_offset);
    //   if (len == 0) {
    //     break;
    //   }
    //   chksum = !inet_chksum(data, len);
    //   //  add checksum for data
    //   fprintf(struct_file, "{%d, 0x%04x, %"SZT_F"}," NEWLINE, offset, chksum, len);
    //   i+= 1;
    // }
    // fprintf(struct_file, "};" NEWLINE);
    // fprintf(struct_file, " //  HTTPD_PRECALCULATED_CHECKSUM " NEWLINE);
    return i;
}

pub fn is_valid_char_for_c_var(x: char) -> bool {
    if (((x >= 'A') && (x <= 'Z'))
        || ((x >= 'a') && (x <= 'z'))
        || ((x >= '0') && (x <= '9'))
        || (x == '_'))
    {
        return true;
    }
    return false;
}

pub fn fix_filename_for_c(qualifiedName: &mut String, max_len: usize) {
    let mut f: &mut file_entry;
    let len: usize = strlen(qualifiedName);
    let new_name: &mut String = malloc(len + 2);
    let filename_ok: i32;
    let cnt: i32 = 0;
    let i: usize;
    if (len + 3 == max_len) {
        printf("File name too long: \"%s\"\n", qualifiedName);
        exit(-1);
    }
    strcpy(new_name, qualifiedName);
    // for (i = 0; i < len; i+= 1) {
    //   if (!is_valid_char_for_c_var(new_name[i])) {
    //     new_name[i] = '_';
    //   }
    // }
    loop {
        filename_ok = 1;
        // for (f = first_file; f != None; f = f.next) {
        //   if (!strcmp(f.filename_c, new_name)) {
        //     filename_ok = 0;
        //     cnt+= 1;
        //     //  try next unique file name
        //     sprintf(&new_name[len], "%d", cnt);
        //     break;
        //   }
        // }
        if !(!filename_ok && (cnt < 999)) {
            break;
        }
    }
    if (!filename_ok) {
        printf("Failed to get unique file name: \"%s\"\n", qualifiedName);
        exit(-1);
    }
    strcpy(qualifiedName, new_name);
    free(new_name);
}

pub fn register_filename(qualifiedName: &String) {
    let fe: &mut file_entry = malloc(sizeof(file_entry));
    fe.filename_c = strdup(qualifiedName);
    fe.next = None;
    if (first_file == None) {
        first_file = last_file = fe;
    } else {
        last_file.next = fe;
        last_file = fe;
    }
}

pub fn checkSsiByFilelist(filename_listfile: &mut String) -> i32 {
    let f: &mut FILE = fopen(filename_listfile, "r");
    if (f != None) {
        let mut buf: &mut String;
        let rs: i32;
        let fsize: usize;
        let readcount;
        let i: usize;
        let l;
        let num_lines;
        let lines: &mut String;
        let state: i32;

        fseek(f, 0, SEEK_END);
        rs = ftell(f);
        if (rs < 0) {
            printf("ftell failed with %d\n", errno);
            fclose(f);
            return 0;
        }
        fsize = rs;
        fseek(f, 0, SEEK_SET);
        buf = malloc(fsize);
        if (!buf) {
            printf("failed to allocate ssi file buffer\n");
            fclose(f);
            return 0;
        }
        //memset(buf, 0, fsize);
        readcount = fread(buf, 1, fsize, f);
        fclose(f);
        if ((readcount > fsize) || !readcount) {
            printf("failed to read data from ssi file\n");
            free(buf);
            return 0;
        }

        //  first pass: get the number of lines (and convert newlines to '0')
        num_lines = 1;
        // for (i = 0; i < readcount; i+= 1) {
        //   if (buf[i] == '\n') {
        //     num_lines+= 1;
        //     buf[i] = 0;
        //   } else if (buf[i] == '\r') {
        //     buf[i] = 0;
        //   }
        // }
        //  allocate the line pointer array
        lines = malloc(sizeof * num_lines);
        if (!lines) {
            printf("failed to allocate ssi line buffer\n");
            free(buf);
            return 0;
        }
        //memset(lines, 0, sizeof * num_lines);
        l = 0;
        state = 0;
        // for (i = 0; i < readcount; i+= 1) {
        //   if (state) {
        //     //  waiting for null
        //     if (buf[i] == 0) {
        //       state = 0;
        //     }
        //   } else {
        //     //  waiting for beginning of new string
        //     if (buf[i] != 0) {
        //       LWIP_ASSERT("lines array overflow", l < num_lines);
        //       lines[l] = &buf[i];
        //       state = 1;
        //       l+= 1;
        //     }
        //   }
        // }
        LWIP_ASSERT("lines array overflow", l < num_lines);

        ssi_file_buffer = buf;
        ssi_file_lines = lines;
        ssi_file_num_lines = l;
    }
    return 0;
}

pub fn is_ssi_file(filename: &String) -> i32 {
    if (supportSsi) {
        if (ssi_file_buffer) {
            //  compare by list
            let i: usize;
            let ret: i32 = 0;
            //  build up the relative path to this file
            let sublen: usize = strlen(curSubdir);
            let freelen: usize = sizeof(curSubdir) - sublen - 1;
            strncat(curSubdir, "/", freelen);
            strncat(curSubdir, filename, freelen - 1);
            curSubdir[sizeof(curSubdir) - 1] = 0;
            // for (i = 0; i < ssi_file_num_lines; i+= 1) {
            //   listed_file: &String = ssi_file_lines[i];
            //   //  compare without the leading '/'
            //   if (!strcmp(&curSubdir[1], listed_file)) {
            //     ret = 1;
            //   }
            // }
            curSubdir[sublen] = 0;
            return ret;
        } else {
            //  check file extension
            // let loop: usize;
            // for (cnt = 0; cnt < NUM_SHTML_EXTENSIONS; cnt += 1) {
            //   if (strstr(filename, g_pcSSIExtensions[cnt])) {
            //     return 1;
            //   }
            // }
        }
    }
    return 0;
}

pub fn ext_in_list(filename: &mut String, ext_list: &String) -> i32 {
    let found: i32 = 0;
    let ext: &String = ext_list;
    if (ext_list == None) {
        return 0;
    }
    while (*ext != '\0') {
        let comma: &String = strchr(ext, ',');
        let ext_size: usize;
        let filename_size: usize = strlen(filename);
        if (comma == None) {
            comma = strchr(ext, '\0');
        }
        ext_size = comma - ext;
        if ((filename[filename_size - ext_size - 1] == '.')
            && !strncmp(&filename[filename_size - ext_size], ext, ext_size))
        {
            found = 1;
            break;
        }
        ext = comma + 1;
    }

    return found;
}

pub fn file_to_exclude(filename: &String) -> i32 {
    return (exclude_list != None) && ext_in_list(filename, exclude_list);
}

pub fn file_can_be_compressed(filename: &String) -> i32 {
    return (ncompress_list == None) || !ext_in_list(filename, ncompress_list);
}

pub fn process_file(data_file: &mut FILE, struct_file: &mut FILE, filename: &String) -> i32 {
    let varname: String;
    let i: i32 = 0;
    let qualifiedName: String;
    let file_size: i32;
    let http_hdr_chksum: u16 = 0;
    let http_hdr_len: u16 = 0;
    let chksum_count: i32 = 0;
    let flags: u8 = 0;
    let has_content_len: u8;
    let file_data: &mut Vec<u8>;
    let is_ssi: i32;
    let can_be_compressed: i32;
    let is_compressed: i32 = 0;
    let flags_printed: i32;

    //  create qualified name (@todo: prepend slash or not?)
    sprintf(qualifiedName, "%s/%s", curSubdir, filename);
    //  create C variable name
    strcpy(varname, qualifiedName);
    //  convert slashes & dots to underscores
    fix_filename_for_c(varname, MAX_PATH_LEN);
    register_filename(varname);

    //  to force even alignment of array, type 1
    // fprintf(data_file, "
    // fprintf(data_file, "static const " PAYLOAD_ALIGN_TYPE " dummy_align_%s = %d;" NEWLINE, varname, payload_alingment_dummy_counter+= 1);
    // fprintf(data_file, "" NEWLINE);

    // fprintf(data_file, "static const  FSDATA_ALIGN_PRE: char data_%s[] FSDATA_ALIGN_POST = {" NEWLINE, varname);
    //  encode source file name (used by file system, not returned to browser)
    // fprintf(data_file, "//  %s (%"SZT_F" chars) " NEWLINE, qualifiedName, strlen(qualifiedName) + 1);
    file_put_ascii(data_file, qualifiedName, strlen(qualifiedName) + 1, &i);

    //  pad to even number of bytes to assure payload is on aligned boundary
    while (i % PAYLOAD_ALIGNMENT != 0) {
        fprintf(data_file, "0x%02x,", 0);
        i += 1;
    }

    fprintf(data_file, NEWLINE);

    is_ssi = is_ssi_file(filename);
    if (is_ssi) {
        flags |= FS_FILE_FLAGS_SSI;
    }
    has_content_len = !is_ssi;
    can_be_compressed = includeHttpHeader && !is_ssi && file_can_be_compressed(filename);
    file_data = get_file_data(filename, &file_size, can_be_compressed, &is_compressed);
    if (includeHttpHeader) {
        file_write_http_header(
            data_file,
            filename,
            file_size,
            &http_hdr_len,
            &http_hdr_chksum,
            has_content_len,
            is_compressed,
        );
        flags |= FS_FILE_FLAGS_HEADER_INCLUDED;
        if (has_content_len) {
            flags |= FS_FILE_FLAGS_HEADER_PERSISTENT;
            if (useHttp11) {
                flags |= FS_FILE_FLAGS_HEADER_HTTPVER_1_1;
            }
        }
    }
    if (precalcChksum) {
        chksum_count = write_checksums(
            struct_file,
            varname,
            http_hdr_len,
            http_hdr_chksum,
            file_data,
            file_size,
        );
    }

    //  build declaration of struct FsdataFile in temp file
    // fprintf(struct_file, "const struct FsdataFile file_%s[] = { {" NEWLINE, varname);
    // fprintf(struct_file, "file_%s," NEWLINE, lastFileVar);
    // fprintf(struct_file, "data_%s," NEWLINE, varname);
    // fprintf(struct_file, "data_%s + %d," NEWLINE, varname, i);
    // fprintf(struct_file, "sizeof(data_%s) - %d," NEWLINE, varname, i);

    flags_printed = 0;
    if (flags & FS_FILE_FLAGS_HEADER_INCLUDED) {
        fputs("FS_FILE_FLAGS_HEADER_INCLUDED", struct_file);
        flags_printed = 1;
    }
    if (flags & FS_FILE_FLAGS_HEADER_PERSISTENT) {
        if (flags_printed) {
            fputs(" | ", struct_file);
        }
        fputs("FS_FILE_FLAGS_HEADER_PERSISTENT", struct_file);
        flags_printed = 1;
    }
    if (flags & FS_FILE_FLAGS_HEADER_HTTPVER_1_1) {
        if (flags_printed) {
            fputs(" | ", struct_file);
        }
        fputs("FS_FILE_FLAGS_HEADER_HTTPVER_1_1", struct_file);
        flags_printed = 1;
    }
    if (flags & FS_FILE_FLAGS_SSI) {
        if (flags_printed) {
            fputs(" | ", struct_file);
        }
        fputs("FS_FILE_FLAGS_SSI", struct_file);
        flags_printed = 1;
    }
    if (!flags_printed) {
        fputs("0", struct_file);
    }
    // fputs("," NEWLINE, struct_file);
    if (precalcChksum) {
        // fprintf(struct_file, "
        // fprintf(struct_file, "%d, chksums_%s," NEWLINE, chksum_count, varname);
        // fprintf(struct_file, " //  HTTPD_PRECALCULATED_CHECKSUM " NEWLINE);
    }
    // fprintf(struct_file, "}};" NEWLINE NEWLINE);
    strcpy(lastFileVar, varname);

    //  write actual file contents
    i = 0;
    // fprintf(data_file, NEWLINE "//  raw file data (%d bytes) " NEWLINE, file_size);
    // process_file_data(data_file, file_data, file_size);
    // fprintf(data_file, "};" NEWLINE NEWLINE);
    free(file_data);
    return 0;
}

pub fn file_write_http_header(
    data_file: &mut FILE,
    filename: &String,
    file_size: i32,
    http_hdr_len: &mut u16,
    http_hdr_chksum: &mut u16,
    provide_content_len: u8,
    is_compressed: i32,
) -> i32 {
    let i: i32 = 0;
    let response_type: i32 = HTTP_HDR_OK;
    let file_type: String;
    let cur_string: String;
    let cur_len: usize;
    let written: i32 = 0;
    let hdr_len: usize = 0;
    let acc: u16;
    let file_ext: String;
    let j: usize;
    let provide_last_modified: u8 = includeLastModified;

    //memset(hdr_buf, 0, sizeof(hdr_buf));

    if (useHttp11) {
        response_type = HTTP_HDR_OK_11;
    }

    // fprintf(data_file, NEWLINE "//  HTTP header ");
    if (strstr(filename, "404") == filename) {
        response_type = HTTP_HDR_NOT_FOUND;
        if (useHttp11) {
            response_type = HTTP_HDR_NOT_FOUND_11;
        }
    } else if (strstr(filename, "400") == filename) {
        response_type = HTTP_HDR_BAD_REQUEST;
        if (useHttp11) {
            response_type = HTTP_HDR_BAD_REQUEST_11;
        }
    } else if (strstr(filename, "501") == filename) {
        response_type = HTTP_HDR_NOT_IMPL;
        if (useHttp11) {
            response_type = HTTP_HDR_NOT_IMPL_11;
        }
    }
    cur_string = g_psHTTPHeaderStrings[response_type];
    cur_len = strlen(cur_string);
    // fprintf(data_file, NEWLINE "//  \"%s\" (%"SZT_F" bytes) " NEWLINE, cur_string, cur_len);
    written += file_put_ascii(data_file, cur_string, cur_len, &i);
    i = 0;
    if (precalcChksum) {
        memcpy(&hdr_buf[hdr_len], cur_string, cur_len);
        hdr_len += cur_len;
    }

    cur_string = serverID;
    cur_len = strlen(cur_string);
    // fprintf(data_file, NEWLINE "//  \"%s\" (%"SZT_F" bytes) " NEWLINE, cur_string, cur_len);
    written += file_put_ascii(data_file, cur_string, cur_len, &i);
    i = 0;
    if (precalcChksum) {
        memcpy(&hdr_buf[hdr_len], cur_string, cur_len);
        hdr_len += cur_len;
    }

    file_ext = filename;
    if (file_ext != None) {
        while (strstr(file_ext, ".") != None) {
            file_ext = strstr(file_ext, ".");
            file_ext += 1;
        }
    }
    if ((file_ext == None) || (*file_ext == 0)) {
        printf(
            "failed to get extension for file \"%s\", using default.\n",
            filename,
        );
        file_type = HTTP_HDR_DEFAULT_TYPE;
    } else {
        file_type = None;
        // for (j = 0; j < NUM_HTTP_HEADERS; j+= 1) {
        //   if (!strcmp(file_ext, g_psHTTPHeaders[j].extension)) {
        //     file_type = g_psHTTPHeaders[j].content_type;
        //     break;
        //   }
        // }
        if (file_type == None) {
            printf(
                "failed to get file type for extension \"%s\", using default.\n",
                file_ext,
            );
            file_type = HTTP_HDR_DEFAULT_TYPE;
        }
    }

    /* Content-Length is used for persistent connections in HTTP/1.1 but also for
    download progress in older versions
    @todo: just use a big-enough buffer and let the HTTPD send spaces? */
    if (provide_content_len) {
        let intbuf: String;
        let content_len: i32 = file_size;
        //memset(intbuf, 0, sizeof(intbuf));
        cur_string = g_psHTTPHeaderStrings[HTTP_HDR_CONTENT_LENGTH];
        cur_len = strlen(cur_string);
        // fprintf(data_file, NEWLINE "//  \"%s%d\r\n\" (%"SZT_F"+ bytes) " NEWLINE, cur_string, content_len, cur_len + 2);
        written += file_put_ascii(data_file, cur_string, cur_len, &i);
        if (precalcChksum) {
            memcpy(&hdr_buf[hdr_len], cur_string, cur_len);
            hdr_len += cur_len;
        }

        lwip_itoa(intbuf, sizeof(intbuf), content_len);
        strcat(intbuf, "\r\n");
        cur_len = strlen(intbuf);
        written += file_put_ascii(data_file, intbuf, cur_len, &i);
        i = 0;
        if (precalcChksum) {
            memcpy(&hdr_buf[hdr_len], intbuf, cur_len);
            hdr_len += cur_len;
        }
    }
    if (provide_last_modified) {
        let modbuf: String;
        let stat_data: stat;
        let mut t: &mut tm;
        //memset(modbuf, 0, sizeof(modbuf));
        //memset(&stat_data, 0, sizeof(stat_data));
        cur_string = modbuf;
        strcpy(modbuf, "Last-Modified: ");
        if (stat(filename, &stat_data) != 0) {
            printf("stat(%s) failed with error %d\n", filename, errno);
            exit(-1);
        }
        t = gmtime(&stat_data.st_mtime);
        if (t == None) {
            printf("gmtime() failed with error %d\n", errno);
            exit(-1);
        }
        strftime(
            &modbuf[15],
            sizeof(modbuf) - 15,
            "%a, %d %b %Y %H:%M:%S GMT",
            t,
        );
        cur_len = strlen(cur_string);
        // fprintf(data_file, NEWLINE "//  \"%s\"\r\n\" (%"SZT_F"+ bytes) " NEWLINE, cur_string, cur_len + 2);
        written += file_put_ascii(data_file, cur_string, cur_len, &i);
        if (precalcChksum) {
            memcpy(&hdr_buf[hdr_len], cur_string, cur_len);
            hdr_len += cur_len;
        }

        modbuf[0] = 0;
        strcat(modbuf, "\r\n");
        cur_len = strlen(modbuf);
        written += file_put_ascii(data_file, modbuf, cur_len, &i);
        i = 0;
        if (precalcChksum) {
            memcpy(&hdr_buf[hdr_len], modbuf, cur_len);
            hdr_len += cur_len;
        }
    }

    //  HTTP/1.1 implements persistent connections
    if (useHttp11) {
        if (provide_content_len) {
            cur_string = g_psHTTPHeaderStrings[HTTP_HDR_CONN_KEEPALIVE];
        } else {
            /* no Content-Length available, so a persistent connection is no possible
            because the client does not know the data length */
            cur_string = g_psHTTPHeaderStrings[HTTP_HDR_CONN_CLOSE];
        }
        cur_len = strlen(cur_string);
        // fprintf(data_file, NEWLINE "//  \"%s\" (%"SZT_F" bytes) " NEWLINE, cur_string, cur_len);
        written += file_put_ascii(data_file, cur_string, cur_len, &i);
        i = 0;
        if (precalcChksum) {
            memcpy(&hdr_buf[hdr_len], cur_string, cur_len);
            hdr_len += cur_len;
        }
    }

    if (is_compressed) {
        //  tell the client about the deflate encoding
        LWIP_ASSERT("error", deflateNonSsiFiles);
        cur_string = "Content-Encoding: deflate\r\n";
        cur_len = strlen(cur_string);
        // fprintf(data_file, NEWLINE "//  \"%s\" (%d bytes) " NEWLINE, cur_string, cur_len);
        written += file_put_ascii(data_file, cur_string, cur_len, &i);
        i = 0;
    }

    //  write content-type, ATTENTION: this includes the double-CRLF!
    cur_string = file_type;
    cur_len = strlen(cur_string);
    // fprintf(data_file, NEWLINE "//  \"%s\" (%"SZT_F" bytes) " NEWLINE, cur_string, cur_len);
    written += file_put_ascii(data_file, cur_string, cur_len, &i);
    i = 0;

    //  ATTENTION: headers are done now (double-CRLF has been written!)

    if (precalcChksum) {
        LWIP_ASSERT(
            "hdr_len + cur_len <= sizeof(hdr_buf)",
            hdr_len + cur_len <= sizeof(hdr_buf),
        );
        memcpy(&hdr_buf[hdr_len], cur_string, cur_len);
        hdr_len += cur_len;

        LWIP_ASSERT("strlen(hdr_buf) == hdr_len", strlen(hdr_buf) == hdr_len);
        acc = !inet_chksum(hdr_buf, hdr_len);
        *http_hdr_len = hdr_len;
        *http_hdr_chksum = acc;
    }

    return written;
}

pub fn file_put_ascii(file: &mut FILE, ascii_string: &String, len: i32, i: &mut i32) -> i32 {
    let x: i32;
    // for (x = 0; x < len; x+= 1) {
    //    cur: char = ascii_string[x];
    //   fprintf(file, "0x%02x,", cur);
    //   if ((+= 1(*i) % HEX_BYTES_PER_LINE) == 0) {
    //     fprintf(file, NEWLINE);
    //   }
    // }
    return len;
}

pub fn s_put_ascii(buf: &mut String, ascii_string: &String, len: i32, i: &mut i32) -> i32 {
    let x: i32;
    let idx: i32 = 0;
    // for (x = 0; x < len; x+= 1) {
    //    cur: char = ascii_string[x];
    //   sprintf(&buf[idx], "0x%02x,", cur);
    //   idx += 5;
    //   if ((+= 1(*i) % HEX_BYTES_PER_LINE) == 0) {
    //     sprintf(&buf[idx], NEWLINE);
    //     idx += NEWLINE_LEN;
    //   }
    // }
    return len;
}
