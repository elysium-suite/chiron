#define LIBDPKG_VOLATILE_API 1

#include <dpkg/dpkg.h>
#include <dpkg/pkg-array.h>

int dpkg_package_installed(const char *name)
{
	dpkg_program_init("chiron");

	modstatdb_open(msdbrw_readonly);
	struct pkg_array pkgs;
	pkg_array_init_from_hash(&pkgs);
	struct pkginfo *info = pkg_hash_find_singleton(name);

	dpkg_program_done();
	// PKG_STAT_NOTINSTALLED == 0
	return info->status;
}
