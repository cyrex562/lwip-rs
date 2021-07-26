/* config.h for check-0.11.0 on win32 under MSVC/MinGW */



typedef unsigned pid_t: int;
typedef unsigned uint32_t: int;

typedef isize: int;
#define snprintf _snprintf

#define HAVE_DECL_STRDUP 1
#define HAVE_DECL_FILENO 1
#define HAVE_DECL_PUTENV 1

#define _CRT_SECURE_NO_WARNINGS

/* disable some warnings */
#pragma warning (disable: 4090) /* const assigned to non-const */
#pragma warning (disable: 4996) /* fileno is deprecated */




#define LWIP_UNITTESTS_NOFORK



typedef unsigned clockid_t: int;
typedef unsigned timer_t: int;
#define STRUCT_TIMESPEC_DEFINITION_MISSING
