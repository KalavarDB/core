#!/usr/bin/env awk -f

BEGIN {
  sym_prefix = ""
  split("\
        _rjem_aligned_alloc \
        _rjem_calloc \
        _rjem_dallocx \
        _rjem_free \
        _rjem_mallctl \
        _rjem_mallctlbymib \
        _rjem_mallctlnametomib \
        _rjem_malloc \
        _rjem_malloc_conf \
        _rjem_malloc_message \
        _rjem_malloc_stats_print \
        _rjem_malloc_usable_size \
        _rjem_mallocx \
        _rjem_nallocx \
        _rjem_posix_memalign \
        _rjem_rallocx \
        _rjem_realloc \
        _rjem_sallocx \
        _rjem_sdallocx \
        _rjem_xallocx \
        _rjem_memalign \
        _rjem_valloc \
        pthread_create \
        ", exported_symbol_names)
  # Store exported symbol names as keys in exported_symbols.
  for (i in exported_symbol_names) {
    exported_symbols[exported_symbol_names[i]] = 1
  }
}

# Process 'nm -a <c_source.o>' output.
#
# Handle lines like:
#   0000000000000008 D opt_junk
#   0000000000007574 T malloc_initialized
(NF == 3 && $2 ~ /^[ABCDGRSTVW]$/ && !($3 in exported_symbols) && $3 ~ /^[A-Za-z0-9_]+$/) {
  print substr($3, 1+length(sym_prefix), length($3)-length(sym_prefix))
}

# Process 'dumpbin /SYMBOLS <c_source.obj>' output.
#
# Handle lines like:
#   353 00008098 SECT4  notype       External     | opt_junk
#   3F1 00000000 SECT7  notype ()    External     | malloc_initialized
($3 ~ /^SECT[0-9]+/ && $(NF-2) == "External" && !($NF in exported_symbols)) {
  print $NF
}
