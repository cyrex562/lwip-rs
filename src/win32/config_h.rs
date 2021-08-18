/* config.h for check-0.11.0 on win32 under MSVC/MinGW */



typedef  pid_t: i32;
typedef  uint32_t: i32;

typedef isize: i32;
#define snprintf _snprintf

#define HAVE_DECL_STRDUP 1
#define HAVE_DECL_FILENO 1
#define HAVE_DECL_PUTENV 1

#define _CRT_SECURE_NO_WARNINGS

/* disable some warnings */
#pragma warning (disable: 4090) /* const assigned to non-const */
#pragma warning (disable: 4996) /* fileno is deprecated */




// #define LWIP_UNITTESTS_NOFORK



typedef  clockid_t: i32;
typedef  timer_t: i32;
#define STRUCT_TIMESPEC_DEFINITION_MISSING
