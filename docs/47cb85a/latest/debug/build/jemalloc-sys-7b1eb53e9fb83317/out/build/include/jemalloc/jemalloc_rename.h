/*
 * Name mangling for public symbols is controlled by --with-mangling and
 * --with-jemalloc-prefix.  With default settings the je_ prefix is stripped by
 * these macro definitions.
 */
#ifndef JEMALLOC_NO_RENAME
#  define je_aligned_alloc _rjem_aligned_alloc
#  define je_calloc _rjem_calloc
#  define je_dallocx _rjem_dallocx
#  define je_free _rjem_free
#  define je_mallctl _rjem_mallctl
#  define je_mallctlbymib _rjem_mallctlbymib
#  define je_mallctlnametomib _rjem_mallctlnametomib
#  define je_malloc _rjem_malloc
#  define je_malloc_conf _rjem_malloc_conf
#  define je_malloc_message _rjem_malloc_message
#  define je_malloc_stats_print _rjem_malloc_stats_print
#  define je_malloc_usable_size _rjem_malloc_usable_size
#  define je_mallocx _rjem_mallocx
#  define je_nallocx _rjem_nallocx
#  define je_posix_memalign _rjem_posix_memalign
#  define je_rallocx _rjem_rallocx
#  define je_realloc _rjem_realloc
#  define je_sallocx _rjem_sallocx
#  define je_sdallocx _rjem_sdallocx
#  define je_xallocx _rjem_xallocx
#  define je_memalign _rjem_memalign
#  define je_valloc _rjem_valloc
#endif
