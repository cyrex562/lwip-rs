/*
Copyright (c) 2013-2019, tinydir authors:
- Cong Xu
- Lautis Sun
- Baudouin Feildel
- Andargor <andargor@yahoo.com>
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/







#define UNICODE



#define _UNICODE






# ifndef WIN32_LEAN_AND_MEAN
#  define WIN32_LEAN_AND_MEAN
# endif
# include <windows.h>
# include <tchar.h>
# pragma warning(push)
# pragma warning (disable : 4996)

# include <dirent.h>
# include <libgen.h>
# include <sys/stat.h>
# include <stddef.h>


# include <tchar.h>



/* types */

/* Windows UNICODE wide character support */

# define _tinydir_char_t TCHAR
# define TINYDIR_STRING(s) _TEXT(s)
# define _tinydir_strlen _tcslen
# define _tinydir_strcpy _tcscpy
# define _tinydir_strcat _tcscat
# define _tinydir_strcmp _tcscmp
# define _tinydir_strrchr _tcsrchr
# define _tinydir_strncmp _tcsncmp

# define _tinydir_char_t char
# define TINYDIR_STRING(s) s
# define _tinydir_strlen strlen
# define _tinydir_strcpy strcpy
# define _tinydir_strcat strcat
# define _tinydir_strcmp strcmp
# define _tinydir_strrchr strrchr
# define _tinydir_strncmp strncmp



# include <windows.h>
# define _TINYDIR_PATH_MAX MAX_PATH
#elif defined  __linux__
# include <limits.h>
# ifdef PATH_MAX
#  define _TINYDIR_PATH_MAX PATH_MAX
# endif
#elif defined(__unix__) || (defined(__APPLE__) && defined(__MACH__))
# include <sys/param.h>
# if defined(BSD)
#  include <limits.h>
#  ifdef PATH_MAX
#   define _TINYDIR_PATH_MAX PATH_MAX
#  endif
# endif



pub const _TINYDIR_PATH_MAX: u32 = 4096;



/* extra chars for the "\\*" mask */
# define _TINYDIR_PATH_EXTRA 2

# define _TINYDIR_PATH_EXTRA 0


pub const _TINYDIR_FILENAME_MAX: u32 = 256;


pub const _TINYDIR_DRIVE_MAX: u32 = 3;



# define _TINYDIR_FUNC static __inline
#elif !defined __STDC_VERSION__ || __STDC_VERSION__ < 199901L
# define _TINYDIR_FUNC static __inline__

# define _TINYDIR_FUNC static inline


/* readdir_r usage; define TINYDIR_USE_READDIR_R to use it (if supported) */


/* readdir_r is a POSIX-only function, and may not be available under various
 * environments/settings, e.g. MinGW. Use readdir fallback */

	_POSIX_SOURCE
# define _TINYDIR_HAS_READDIR_R


# define _TINYDIR_HAS_FPATHCONF
# include <unistd.h>


	(_POSIX_C_SOURCE >= 200809L || _XOPEN_SOURCE >= 700)
# define _TINYDIR_HAS_DIRFD
# include <sys/types.h>


	defined _PC_NAME_MAX
# define _TINYDIR_USE_FPATHCONF


	!(defined _TINYDIR_USE_FPATHCONF || defined NAME_MAX)
# define _TINYDIR_USE_READDIR


/* Use readdir by default */

# define _TINYDIR_USE_READDIR


/* MINGW32 has two versions of dirent, ASCII and UNICODE*/


pub const _TINYDIR_DIR: u32 = _WDIR;
pub const _tinydir_dirent: u32 = _wdirent;
pub const _tinydir_opendir: u32 = _wopendir;
pub const _tinydir_readdir: u32 = _wreaddir;
pub const _tinydir_closedir: u32 = _wclosedir;

pub const _TINYDIR_DIR: u32 = DIR;
pub const _tinydir_dirent: u32 = dirent;
pub const _tinydir_opendir: u32 = opendir;
pub const _tinydir_readdir: u32 = readdir;
pub const _tinydir_closedir: u32 = closedir;



/* Allow user to use a custom allocator by defining _TINYDIR_MALLOC and _TINYDIR_FREE. */

#elif !defined(_TINYDIR_MALLOC) && !defined(_TINYDIR_FREE)

#error "Either define both alloc and free or none of them!"



	#define _TINYDIR_MALLOC(_size) malloc(_size)
	#define _TINYDIR_FREE(_ptr)    free(_ptr)
 /* !defined(_TINYDIR_MALLOC) */

typedef struct tinydir_file
{
	_tinydir_char_t path[_TINYDIR_PATH_MAX];
	_tinydir_char_t name[_TINYDIR_FILENAME_MAX];
	_tinydir_char_t *extension;
	int is_dir;
	int is_reg;



	struct _stat _s;

	struct stat _s;


} tinydir_file;

typedef struct tinydir_dir
{
	_tinydir_char_t path[_TINYDIR_PATH_MAX];
	int has_next;
	size_t n_files;

	tinydir_file *_files;

	HANDLE _h;
	WIN32_FIND_DATA _f;

	_TINYDIR_DIR *_d;
	struct _tinydir_dirent *_e;

	struct _tinydir_dirent *_ep;


} tinydir_dir;


/* declarations */

_TINYDIR_FUNC
int tinydir_open(tinydir_dir *dir, const _tinydir_char_t *path);
_TINYDIR_FUNC
int tinydir_open_sorted(tinydir_dir *dir, const _tinydir_char_t *path);
_TINYDIR_FUNC
void tinydir_close(tinydir_dir *dir);

_TINYDIR_FUNC
int tinydir_next(tinydir_dir *dir);
_TINYDIR_FUNC
int tinydir_readfile(const tinydir_dir *dir, tinydir_file *file);
_TINYDIR_FUNC
int tinydir_readfile_n(const tinydir_dir *dir, tinydir_file *file, size_t i);
_TINYDIR_FUNC
int tinydir_open_subdir_n(tinydir_dir *dir, size_t i);

_TINYDIR_FUNC
int tinydir_file_open(tinydir_file *file, const _tinydir_char_t *path);
_TINYDIR_FUNC
void _tinydir_get_ext(tinydir_file *file);
_TINYDIR_FUNC
int _tinydir_file_cmp(const void *a, const void *b);


_TINYDIR_FUNC
size_t _tinydir_dirent_buf_size(_TINYDIR_DIR *dirp);




/* definitions*/

_TINYDIR_FUNC
int tinydir_open(tinydir_dir *dir, const _tinydir_char_t *path)
{


	int error;
	int size;	/* using int size */


	_tinydir_char_t path_buf[_TINYDIR_PATH_MAX];

	_tinydir_char_t *pathp;

	if (dir == NULL || path == NULL || _tinydir_strlen(path) == 0)
	{
		errno = EINVAL;
		return -1;
	}
	if (_tinydir_strlen(path) + _TINYDIR_PATH_EXTRA >= _TINYDIR_PATH_MAX)
	{
		errno = ENAMETOOLONG;
		return -1;
	}

	/* initialise dir */
	dir->_files = NULL;

	dir->_h = INVALID_HANDLE_VALUE;

	dir->_d = NULL;

	dir->_ep = NULL;


	tinydir_close(dir);

	_tinydir_strcpy(dir->path, path);
	/* Remove trailing slashes */
	pathp = &dir->path[_tinydir_strlen(dir->path) - 1];
	while (pathp != dir->path && (*pathp == TINYDIR_STRING('\\') || *pathp == TINYDIR_STRING('/')))
	{
		*pathp = TINYDIR_STRING('\0');
		pathp++;
	}

	_tinydir_strcpy(path_buf, dir->path);
	_tinydir_strcat(path_buf, TINYDIR_STRING("\\*"));

	dir->_h = FindFirstFileEx(path_buf, FindExInfoStandard, &dir->_f, FindExSearchNameMatch, NULL, 0);

	dir->_h = FindFirstFile(path_buf, &dir->_f);

	if (dir->_h == INVALID_HANDLE_VALUE)
	{
		errno = ENOENT;

	dir->_d = _tinydir_opendir(path);
	if (dir->_d == NULL)
	{

		goto bail;
	}

	/* read first file */
	dir->has_next = 1;


	dir->_e = _tinydir_readdir(dir->_d);

	/* allocate dirent buffer for readdir_r */
	size = _tinydir_dirent_buf_size(dir->_d); /* conversion to int */
	if (size == -1) return -1;
	dir->_ep = (struct _tinydir_dirent*)_TINYDIR_MALLOC(size);
	if (dir->_ep == NULL) return -1;

	error = readdir_r(dir->_d, dir->_ep, &dir->_e);
	if (error != 0) return -1;

	if (dir->_e == NULL)
	{
		dir->has_next = 0;
	}


	return 0;

bail:
	tinydir_close(dir);
	return -1;
}

_TINYDIR_FUNC
int tinydir_open_sorted(tinydir_dir *dir, const _tinydir_char_t *path)
{
	/* Count the number of files first, to pre-allocate the files array */
	size_t n_files = 0;
	if (tinydir_open(dir, path) == -1)
	{
		return -1;
	}
	while (dir->has_next)
	{
		n_files++;
		if (tinydir_next(dir) == -1)
		{
			goto bail;
		}
	}
	tinydir_close(dir);

	if (n_files == 0 || tinydir_open(dir, path) == -1)
	{
		return -1;
	}

	dir->n_files = 0;
	dir->_files = (tinydir_file *)_TINYDIR_MALLOC(sizeof *dir->_files * n_files);
	if (dir->_files == NULL)
	{
		goto bail;
	}
	while (dir->has_next)
	{
		tinydir_file *p_file;
		dir->n_files++;

		p_file = &dir->_files[dir->n_files - 1];
		if (tinydir_readfile(dir, p_file) == -1)
		{
			goto bail;
		}

		if (tinydir_next(dir) == -1)
		{
			goto bail;
		}

		/* Just in case the number of files has changed between the first and
		second reads, terminate without writing into unallocated memory */
		if (dir->n_files == n_files)
		{
			break;
		}
	}

	qsort(dir->_files, dir->n_files, sizeof(tinydir_file), _tinydir_file_cmp);

	return 0;

bail:
	tinydir_close(dir);
	return -1;
}

_TINYDIR_FUNC
void tinydir_close(tinydir_dir *dir)
{
	if (dir == NULL)
	{
		return;
	}

	memset(dir->path, 0, sizeof(dir->path));
	dir->has_next = 0;
	dir->n_files = 0;
	_TINYDIR_FREE(dir->_files);
	dir->_files = NULL;

	if (dir->_h != INVALID_HANDLE_VALUE)
	{
		FindClose(dir->_h);
	}
	dir->_h = INVALID_HANDLE_VALUE;

	if (dir->_d)
	{
		_tinydir_closedir(dir->_d);
	}
	dir->_d = NULL;
	dir->_e = NULL;

	_TINYDIR_FREE(dir->_ep);
	dir->_ep = NULL;


}

_TINYDIR_FUNC
int tinydir_next(tinydir_dir *dir)
{
	if (dir == NULL)
	{
		errno = EINVAL;
		return -1;
	}
	if (!dir->has_next)
	{
		errno = ENOENT;
		return -1;
	}


	if (FindNextFile(dir->_h, &dir->_f) == 0)


	dir->_e = _tinydir_readdir(dir->_d);

	if (dir->_ep == NULL)
	{
		return -1;
	}
	if (readdir_r(dir->_d, dir->_ep, &dir->_e) != 0)
	{
		return -1;
	}

	if (dir->_e == NULL)

	{
		dir->has_next = 0;

		if (GetLastError() != ERROR_SUCCESS &&
			GetLastError() != ERROR_NO_MORE_FILES)
		{
			tinydir_close(dir);
			errno = EIO;
			return -1;
		}

	}

	return 0;
}

_TINYDIR_FUNC
int tinydir_readfile(const tinydir_dir *dir, tinydir_file *file)
{
	const _tinydir_char_t *filename;
	if (dir == NULL || file == NULL)
	{
		errno = EINVAL;
		return -1;
	}

	if (dir->_h == INVALID_HANDLE_VALUE)

	if (dir->_e == NULL)

	{
		errno = ENOENT;
		return -1;
	}
	filename =

		dir->_f.cFileName;

		dir->_e->d_name;

	if (_tinydir_strlen(dir->path) +
		_tinydir_strlen(filename) + 1 + _TINYDIR_PATH_EXTRA >=
		_TINYDIR_PATH_MAX)
	{
		/* the path for the file will be too long */
		errno = ENAMETOOLONG;
		return -1;
	}
	if (_tinydir_strlen(filename) >= _TINYDIR_FILENAME_MAX)
	{
		errno = ENAMETOOLONG;
		return -1;
	}

	_tinydir_strcpy(file->path, dir->path);
	if (_tinydir_strcmp(dir->path, TINYDIR_STRING("/")) != 0)
		_tinydir_strcat(file->path, TINYDIR_STRING("/"));
	_tinydir_strcpy(file->name, filename);
	_tinydir_strcat(file->path, filename);


	if (_tstat(
#elif (defined _BSD_SOURCE) || (defined _DEFAULT_SOURCE)	\
	|| ((defined _XOPEN_SOURCE) && (_XOPEN_SOURCE >= 500))	\
	|| ((defined _POSIX_C_SOURCE) && (_POSIX_C_SOURCE >= 200112L))
	if (lstat(

	if (stat(

		file->path, &file->_s) == -1)
	{
		return -1;
	}

	_tinydir_get_ext(file);

	file->is_dir =

		!!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY);

		S_ISDIR(file->_s.st_mode);

	file->is_reg =

		!!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_NORMAL) ||
		(
			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_DEVICE) &&
			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) &&
			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_ENCRYPTED) &&

			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_INTEGRITY_STREAM) &&


			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_NO_SCRUB_DATA) &&

			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_OFFLINE) &&
			!(dir->_f.dwFileAttributes & FILE_ATTRIBUTE_TEMPORARY));

		S_ISREG(file->_s.st_mode);


	return 0;
}

_TINYDIR_FUNC
int tinydir_readfile_n(const tinydir_dir *dir, tinydir_file *file, size_t i)
{
	if (dir == NULL || file == NULL)
	{
		errno = EINVAL;
		return -1;
	}
	if (i >= dir->n_files)
	{
		errno = ENOENT;
		return -1;
	}

	memcpy(file, &dir->_files[i], sizeof(tinydir_file));
	_tinydir_get_ext(file);

	return 0;
}

_TINYDIR_FUNC
int tinydir_open_subdir_n(tinydir_dir *dir, size_t i)
{
	_tinydir_char_t path[_TINYDIR_PATH_MAX];
	if (dir == NULL)
	{
		errno = EINVAL;
		return -1;
	}
	if (i >= dir->n_files || !dir->_files[i].is_dir)
	{
		errno = ENOENT;
		return -1;
	}

	_tinydir_strcpy(path, dir->_files[i].path);
	tinydir_close(dir);
	if (tinydir_open_sorted(dir, path) == -1)
	{
		return -1;
	}

	return 0;
}

/* Open a single file given its path */
_TINYDIR_FUNC
int tinydir_file_open(tinydir_file *file, const _tinydir_char_t *path)
{
	tinydir_dir dir;
	int result = 0;
	int found = 0;
	_tinydir_char_t dir_name_buf[_TINYDIR_PATH_MAX];
	_tinydir_char_t file_name_buf[_TINYDIR_FILENAME_MAX];
	_tinydir_char_t *dir_name;
	_tinydir_char_t *base_name;

	_tinydir_char_t drive_buf[_TINYDIR_PATH_MAX];
	_tinydir_char_t ext_buf[_TINYDIR_FILENAME_MAX];


	if (file == NULL || path == NULL || _tinydir_strlen(path) == 0)
	{
		errno = EINVAL;
		return -1;
	}
	if (_tinydir_strlen(path) + _TINYDIR_PATH_EXTRA >= _TINYDIR_PATH_MAX)
	{
		errno = ENAMETOOLONG;
		return -1;
	}

	/* Get the parent path */


	errno = _tsplitpath_s(
		path,
		drive_buf, _TINYDIR_DRIVE_MAX,
		dir_name_buf, _TINYDIR_FILENAME_MAX,
		file_name_buf, _TINYDIR_FILENAME_MAX,
		ext_buf, _TINYDIR_FILENAME_MAX);

	_tsplitpath(
		path,
		drive_buf,
		dir_name_buf,
		file_name_buf,
		ext_buf);


	if (errno)
	{
		return -1;
	}

/* _splitpath_s not work fine with only filename and widechar support */

	if (drive_buf[0] == L'\xFEFE')
		drive_buf[0] = '\0';
	if (dir_name_buf[0] == L'\xFEFE')
		dir_name_buf[0] = '\0';


	/* Emulate the behavior of dirname by returning "." for dir name if it's
	empty */
	if (drive_buf[0] == '\0' && dir_name_buf[0] == '\0')
	{
		_tinydir_strcpy(dir_name_buf, TINYDIR_STRING("."));
	}
	/* Concatenate the drive letter and dir name to form full dir name */
	_tinydir_strcat(drive_buf, dir_name_buf);
	dir_name = drive_buf;
	/* Concatenate the file name and extension to form base name */
	_tinydir_strcat(file_name_buf, ext_buf);
	base_name = file_name_buf;

	_tinydir_strcpy(dir_name_buf, path);
	dir_name = dirname(dir_name_buf);
	_tinydir_strcpy(file_name_buf, path);
	base_name = basename(file_name_buf);


	/* Special case: if the path is a root dir, open the parent dir as the file */

	if (_tinydir_strlen(base_name) == 0)

	if ((_tinydir_strcmp(base_name, TINYDIR_STRING("/"))) == 0)

	{
		memset(file, 0, sizeof * file);
		file->is_dir = 1;
		file->is_reg = 0;
		_tinydir_strcpy(file->path, dir_name);
		file->extension = file->path + _tinydir_strlen(file->path);
		return 0;
	}

	/* Open the parent directory */
	if (tinydir_open(&dir, dir_name) == -1)
	{
		return -1;
	}

	/* Read through the parent directory and look for the file */
	while (dir.has_next)
	{
		if (tinydir_readfile(&dir, file) == -1)
		{
			result = -1;
			goto bail;
		}
		if (_tinydir_strcmp(file->name, base_name) == 0)
		{
			/* File found */
			found = 1;
			break;
		}
		tinydir_next(&dir);
	}
	if (!found)
	{
		result = -1;
		errno = ENOENT;
	}

bail:
	tinydir_close(&dir);
	return result;
}

_TINYDIR_FUNC
void _tinydir_get_ext(tinydir_file *file)
{
	_tinydir_char_t *period = _tinydir_strrchr(file->name, TINYDIR_STRING('.'));
	if (period == NULL)
	{
		file->extension = &(file->name[_tinydir_strlen(file->name)]);
	}
	else
	{
		file->extension = period + 1;
	}
}

_TINYDIR_FUNC
int _tinydir_file_cmp(const void *a, const void *b)
{
	const tinydir_file *fa = (const tinydir_file *)a;
	const tinydir_file *fb = (const tinydir_file *)b;
	if (fa->is_dir != fb->is_dir)
	{
		return -(fa->is_dir - fb->is_dir);
	}
	return _tinydir_strncmp(fa->name, fb->name, _TINYDIR_FILENAME_MAX);
}



/*
The following authored by Ben Hutchings <ben@decadent.org.uk>
from https://womble.decadent.org.uk/readdir_r-advisory.html
*/
/* Calculate the required buffer size (in bytes) for directory      *
* entries read from the given directory handle.  Return -1 if this  *
* this cannot be done.                                              *
*                                                                   *
* This code does not trust values of NAME_MAX that are less than    *
* 255, since some systems (including at least HP-UX) incorrectly    *
* define it to be a smaller value.                                  */
_TINYDIR_FUNC
size_t _tinydir_dirent_buf_size(_TINYDIR_DIR *dirp)
{
	long name_max;
	size_t name_end;
	/* parameter may be unused */
	(void)dirp;


	name_max = fpathconf(dirfd(dirp), _PC_NAME_MAX);
	if (name_max == -1)

		name_max = (NAME_MAX > 255) ? NAME_MAX : 255;

		return (size_t)(-1);

#elif defined(NAME_MAX)
 	name_max = (NAME_MAX > 255) ? NAME_MAX : 255;

#error "buffer size for readdir_r cannot be determined"

	name_end = (size_t)offsetof(struct _tinydir_dirent, d_name) + name_max + 1;
	return (name_end > sizeof(struct _tinydir_dirent) ?
		name_end : sizeof(struct _tinydir_dirent));
}




}


# if defined (_MSC_VER)
# pragma warning(pop)
# endif


