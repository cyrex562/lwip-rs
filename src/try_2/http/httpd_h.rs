/*
 * @file
 * HTTP server
 */

/*
 * Copyright (c) 2001-2003 Swedish Institute of Computer Science.
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
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Adam Dunkels <adam@sics.se>
 *
 * This version of the file has been modified by Texas Instruments to offer
 * simple server-side-include (SSI) and Common Gateway Interface (CGI)
 * capability.
 */

//

/*
 * @ingroup httpd
 * Function pointer for a CGI script handler.
 *
 * This function is called each time the HTTPD server is asked for a file
 * whose name was previously registered as a CGI function using a call to
 * http_set_cgi_handlers. The iIndex parameter provides the index of the
 * CGI within the cgis array passed to http_set_cgi_handlers. Parameters
 * pcParam and pcValue provide access to the parameters provided along with
 * the URI. iNumParams provides a count of the entries in the pcParam and
 * pcValue arrays. Each entry in the pcParam array contains the name of a
 * parameter with the corresponding entry in the pcValue array containing the
 * value for that parameter. Note that pcParam may contain multiple elements
 * with the same name if, for example, a multi-selection list control is used
 * in the form generating the data.
 *
 * The function should return a pointer to a character string which is the
 * path and filename of the response that is to be sent to the connected
 * browser, for example "/thanks.htm" or "/response/error.ssi".
 *
 * The maximum number of parameters that will be passed to this function via
 * iNumParams is defined by LWIP_HTTPD_MAX_CGI_PARAMETERS. Any parameters in
 * the incoming HTTP request above this number will be discarded.
 *
 * Requests intended for use by this CGI mechanism must be sent using the GET
 * method (which encodes all parameters within the URI rather than in a block
 * later in the request). Attempts to use the POST method will result in the
 * request being ignored.
 *
 */
type tCGIHandler =
    fn(iIndex: i32, iNumParams: i32, pcParam: &mut String, pcValue: &mut String) -> String;

/*
 * @ingroup httpd
 * Structure defining the base filename (URL) of a CGI and the associated
 * function which is to be called when that URL is requested.
 */
pub struct tCGI {
    pub pcCGIName: String,
    pub pfnCGIHandler: tCGIHandler,
}

// pub fn  http_set_cgi_handlers( tCGI *pCGIs, iNumHandlers: i32);

//  we have to prototype this struct here to make it available for the handler
// struct FsFile;

/* Define this generic CGI handler in your application.
 * It is called once for every URI with parameters.
 * The parameters can be stored to the object passed as connection_state, which
 * is allocated to file.state via fs_state_init() from fs_open() or fs_open_custom().
 * Content creation via SSI or complete dynamic files can retrieve the CGI params from there.
 */
// extern void httpd_cgi_handler(file: &mut FsFile,  uri: &mut String, iNumParams: i32,
//                               pcParam: &mut String, pcValue: &mut String

//                                      , connection_state: &mut Vec<u8>

//                                      );

/*
 * @ingroup httpd
 * Function pointer for the SSI tag handler callback.
 *
 * This function will be called each time the HTTPD server detects a tag of the
 * form <!--#name--> in files with extensions mentioned in the g_pcSSIExtensions
 * array (currently .shtml, .shtm, .ssi, .xml, .json) where "name" appears as
 * one of the tags supplied to http_set_ssi_handler in the tags array.  The
 * returned insert string, which will be appended after the the string
 * "<!--#name-->" in file sent back to the client, should be written to pointer
 * pcInsert. iInsertLen contains the size of the buffer pointed to by
 * pcInsert. The iIndex parameter provides the zero-based index of the tag as
 * found in the tags array and identifies the tag that is to be processed.
 *
 * The handler returns the number of characters written to pcInsert excluding
 * any terminating NULL or HTTPD_SSI_TAG_UNKNOWN when tag is not recognized.
 *
 * Note that the behavior of this SSI mechanism is somewhat different from the
 * "normal" SSI processing as found in, for example, the Apache web server.  In
 * this case, the inserted text is appended following the SSI tag rather than
 * replacing the tag entirely.  This allows for an implementation that does not
 * require significant additional buffering of output data yet which will still
 * offer usable SSI functionality. One downside to this approach is when
 * attempting to use SSI within JavaScript.  The SSI tag is structured to
 * resemble an HTML comment but this syntax does not constitute a comment
 * within JavaScript and, hence, leaving the tag in place will result in
 * problems in these cases. In order to avoid these problems, define
 * LWIP_HTTPD_SSI_INCLUDE_TAG as zero in your lwip options file, or use JavaScript
 * style block comments in the form / * # name * / (without the spaces).
 */
type tSSIHandler = fn(
    ssi_tag_name: &String, //  LWIP_HTTPD_SSI_RAW
    iIndex: i32,
    pcInsert: &mut String,
    iInsertLen: i32,
    current_tag_part: u16,
    next_tag_part: &mut u16,
    connection_state: &mut Vec<u8>,
) -> u16;

/* Set the SSI handler function
 * (if LWIP_HTTPD_SSI_RAW==1, only the first argument is used)
 */
// pub fn  http_set_ssi_handler(tSSIHandler pfnSSIHandler,
//                           const ppcTags: &mut String, iNumTags: i32);

/* For LWIP_HTTPD_SSI_RAW==1, return this to indicate the tag is unknown.
 * In this case, the webserver writes a warning into the page.
 * You can also just return 0 to write nothing for unknown tags.
 */
pub const HTTPD_SSI_TAG_UNKNOWN: u32 = 0xFFFF;

//  These functions must be implemented by the application

/*
 * @ingroup httpd
 * Called when a POST request has been received. The application can decide
 * whether to accept it or not.
 *
 * @param connection Unique connection identifier, valid until httpd_post_end
 *        is called.
 * @param uri The HTTP header URI receiving the POST request.
 * @param http_request The raw HTTP request (the first packet, normally).
 * @param http_request_len Size of 'http_request'.
 * @param content_len Content-Length from HTTP header.
 * @param response_uri Filename of response file, to be filled when denying the
 *        request
 * @param response_uri_len Size of the 'response_uri' buffer.
 * @param post_auto_wnd Set this to 0 to let the callback code handle window
 *        updates by calling 'httpd_post_data_recved' (to throttle rx speed)
 *        default is 1 (httpd handles window updates automatically)
 * @return ERR_OK: Accept the POST request, data may be passed in
 *         another err_t: Deny the POST request, send back 'bad request'.
 */
// pub fn  httpd_post_begin(connection: &mut Vec<u8>, uri: &String, http_request: &String,
//    http_request_len: u16, content_len: i32, response_uri: &mut String,
//    response_uri_len: u16, post_auto_wnd: &mut Vec<u8>);

/*
 * @ingroup httpd
 * Called for each pbuf of data that has been received for a POST.
 * ATTENTION: The application is responsible for freeing the pbufs passed in!
 *
 * @param connection Unique connection identifier.
 * @param p Received data.
 * @return ERR_OK: Data accepted.
 *         another err_t: Data denied, http_post_get_response_uri will be called.
 */
// pub fn  httpd_post_receive_data(connection: &mut Vec<u8>, p: &mut PacketBuffer);

/*
 * @ingroup httpd
 * Called when all data is received or when the connection is closed.
 * The application must return the filename/URI of a file to send in response
 * to this POST request. If the response_uri buffer is untouched, a 404
 * response is returned.
 *
 * @param connection Unique connection identifier.
 * @param response_uri Filename of response file, to be filled when denying the request
 * @param response_uri_len Size of the 'response_uri' buffer.
 */
// pub fn  httpd_post_finished(connection: &mut Vec<u8>, response_uri: &mut String, response_uri_len: u16);

// pub fn  httpd_post_data_recved(connection: &mut Vec<u8>, recved_len: u16);

// pub fn  httpd_init();

// pub fn  httpd_inits(conf: &mut altcp_tls_config);
