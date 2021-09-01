
// #define LWIP_HTTPD_STRUCTS_H




/* This struct is used for a list of HTTP header strings for various
 * filename extensions. */
typedef struct {
  let extension: String;
  let content_type: String;
} tHTTPHeader;

/* A list of strings used in HTTP headers (see RFC 1945 HTTP/1.0 and
 * RFC 2616 HTTP/1.1 for header field definitions) */
static const: &String g_psHTTPHeaderStrings[] = {
  "HTTP/1.0 200 OK\r\n",
  "HTTP/1.0 404 File not found\r\n",
  "HTTP/1.0 400 Bad Request\r\n",
  "HTTP/1.0 501 Not Implemented\r\n",
  "HTTP/1.1 200 OK\r\n",
  "HTTP/1.1 404 File not found\r\n",
  "HTTP/1.1 400 Bad Request\r\n",
  "HTTP/1.1 501 Not Implemented\r\n",
  "Content-Length: ",
  "Connection: Close\r\n",
  "Connection: keep-alive\r\n",
  "Connection: keep-alive\r\nContent-Length: ",
  "Server: "HTTPD_SERVER_AGENT"\r\n",
  "\r\n<html><body><h2>404: The requested file cannot be found.</h2></body></html>\r\n"

  , "Connection: keep-alive\r\nContent-Length: 77\r\n\r\n<html><body><h2>404: The requested file cannot be found.</h2></body></html>\r\n"

};

/* Indexes into the g_psHTTPHeaderStrings array */
pub const HTTP_HDR_OK: u32 = 0;  /* 200 OK */pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; pub const HTTP_HDR_OK: u32 = 0; 
pub const HTTP_HDR_NOT_FOUND: u32 = 1; 04 File not found */
pub const HTTP_HDR_BAD_REQUEST: u32 = 2; /* 400 Bad request */
pub const HTTP_HDR_NOT_IMPL: u32 = 3; /* 501 Not Implemented */
pub const HTTP_HDR_OK_11: u32 = 4; /* 200 OK */
pub const HTTP_HDR_NOT_FOUND_11: u32 = 5; /* 404 File not found */
pub const HTTP_HDR_BAD_REQUEST_11: u32 = 6; /* 400 Bad request */
pub const HTTP_HDR_NOT_IMPL_11: u32 = 7; /* 501 Not Implemented */
pub const HTTP_HDR_CONTENT_LENGTH: u32 = 8; /* Content-Length: (HTTP 1.0)*/
pub const HTTP_HDR_CONN_CLOSE: u32 = 9; /* Connection: Close (HTTP 1.1) */
pub const HTTP_HDR_CONN_KEEPALIVE: u32 = 10; /* Connection: keep-alive (HTTP 1.1) */
pub const HTTP_HDR_KEEPALIVE_LEN: u32 = 11; /* Connection: keep-alive + Content-Length: (HTTP 1.1)*/
pub const HTTP_HDR_SERVER: u32 = 12; /* Server: HTTPD_SERVER_AGENT */
pub const DEFAULT_404_HTML: u32 = 13; /* default 404 body */

pub const DEFAULT_404_HTML_PERSISTENT: u32 = 14; /* default 404 body, but including Connection: keep-alive */


#define HTTP_CONTENT_TYPE(contenttype) "Content-Type: "contenttype"\r\n\r\n"
#define HTTP_CONTENT_TYPE_ENCODING(contenttype, encoding) "Content-Type: "contenttype"\r\nContent-Encoding: "encoding"\r\n\r\n"

pub const HTTP_HDR_HTML: u32 = HTTP_CONTENT_TYPE;("text/html")
pub const HTTP_HDR_SSI: u32 = HTTP_CONTENT_TYPE;("text/html\r\nExpires: Fri, 10 Apr 2008 14:00:00 GMT\r\nPragma: no-cache")
pub const HTTP_HDR_GIF: u32 = HTTP_CONTENT_TYPE;("image/gif")
pub const HTTP_HDR_PNG: u32 = HTTP_CONTENT_TYPE;("image/png")
pub const HTTP_HDR_JPG: u32 = HTTP_CONTENT_TYPE;("image/jpeg")
pub const HTTP_HDR_BMP: u32 = HTTP_CONTENT_TYPE;("image/bmp")
pub const HTTP_HDR_ICO: u32 = HTTP_CONTENT_TYPE;("image/x-icon")
pub const HTTP_HDR_APP: u32 = HTTP_CONTENT_TYPE;("application/octet-stream")
pub const HTTP_HDR_JS: u32 = HTTP_CONTENT_TYPE;("application/javascript")
pub const HTTP_HDR_RA: u32 = HTTP_CONTENT_TYPE;("application/javascript")
pub const HTTP_HDR_CSS: u32 = HTTP_CONTENT_TYPE;("text/css")
pub const HTTP_HDR_SWF: u32 = HTTP_CONTENT_TYPE;("application/x-shockwave-flash")
pub const HTTP_HDR_XML: u32 = HTTP_CONTENT_TYPE;("text/xml")
pub const HTTP_HDR_PDF: u32 = HTTP_CONTENT_TYPE;("application/pdf")
pub const HTTP_HDR_JSON: u32 = HTTP_CONTENT_TYPE;("application/json")
pub const HTTP_HDR_CSV: u32 = HTTP_CONTENT_TYPE;("text/csv")
pub const HTTP_HDR_TSV: u32 = HTTP_CONTENT_TYPE;("text/tsv")
pub const HTTP_HDR_SVG: u32 = HTTP_CONTENT_TYPE;("image/svg+xml")
pub const HTTP_HDR_SVGZ: u32 = HTTP_CONTENT_TYPE_ENCODING;("image/svg+xml", "gzip")

pub const HTTP_HDR_DEFAULT_TYPE: u32 = HTTP_CONTENT_TYPE;("text/plain")

/* A list of extension-to-HTTP header strings (see outdated RFC 1700 MEDIA TYPES
 * and http://www.iana.org/assignments/media-types for registered content types
 * and subtypes) */
static const tHTTPHeader g_psHTTPHeaders[] = {
  { "html", HTTP_HDR_HTML},
  { "htm",  HTTP_HDR_HTML},
  { "shtml", HTTP_HDR_SSI},
  { "shtm", HTTP_HDR_SSI},
  { "ssi",  HTTP_HDR_SSI},
  { "gif",  HTTP_HDR_GIF},
  { "png",  HTTP_HDR_PNG},
  { "jpg",  HTTP_HDR_JPG},
  { "bmp",  HTTP_HDR_BMP},
  { "ico",  HTTP_HDR_ICO},
  { "class", HTTP_HDR_APP},
  { "cls",  HTTP_HDR_APP},
  { "js",   HTTP_HDR_JS},
  { "ram",  HTTP_HDR_RA},
  { "css",  HTTP_HDR_CSS},
  { "swf",  HTTP_HDR_SWF},
  { "xml",  HTTP_HDR_XML},
  { "xsl",  HTTP_HDR_XML},
  { "pdf",  HTTP_HDR_PDF},
  { "json", HTTP_HDR_JSON}

  /* If you need to add content types not listed here:
   * #define HTTPD_ADDITIONAL_CONTENT_TYPES {"ct1", HTTP_CONTENT_TYPE("text/ct1")}, {"exe", HTTP_CONTENT_TYPE("application/exe")}
   */
  , HTTPD_ADDITIONAL_CONTENT_TYPES

};

pub const NUM_HTTP_HEADERS: u32 = LWIP_ARRAYSIZE;(g_psHTTPHeaders)




static const: &String g_pcSSIExtensions[] = {
  ".shtml", ".shtm", ".ssi", ".xml", ".json"
};
pub const NUM_SHTML_EXTENSIONS: u32 = LWIP_ARRAYSIZE;(g_pcSSIExtensions)



