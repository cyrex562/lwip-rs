/* config.h for check-0.11.0 on win32 under MSVC/MinGW */

#ifdef _MSC_VER

typedef unsigned int pid_t;
typedef unsigned int uint32_t;

typedef int ssize_t;
pub const snprintf: u32 = _snprintf;

pub const HAVE_DECL_STRDUP: u32 = 1;
pub const HAVE_DECL_FILENO: u32 = 1;
pub const HAVE_DECL_PUTENV: u32 = 1;

#define _CRT_SECURE_NO_WARNINGS

/* disable some warnings */
#pragma warning (disable: 4090) /* const assigned to non-const */
#pragma warning (disable: 4996) /* fileno is deprecated */

 /* _ MSC_VER */


#define LWIP_UNITTESTS_NOFORK



typedef unsigned int clockid_t;
typedef unsigned int timer_t;
#define STRUCT_TIMESPEC_DEFINITION_MISSING
