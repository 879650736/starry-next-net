# iperf, Copyright (c) 2014-2022, The Regents of the University of
# California, through Lawrence Berkeley National Laboratory (subject
# to receipt of any required approvals from the U.S. Dept. of
# Energy).  All rights reserved.
#
# If you have questions about your rights to use or distribute this
# software, please contact Berkeley Lab's Technology Transfer
# Department at TTD@lbl.gov.
#
# NOTICE.  This software is owned by the U.S. Department of Energy.
# As such, the U.S. Government has been granted for itself and others
# acting on its behalf a paid-up, nonexclusive, irrevocable,
# worldwide license in the Software to reproduce, prepare derivative
# works, and perform publicly and display publicly.  Beginning five
# (5) years after the date permission to assert copyright is obtained
# from the U.S. Department of Energy, and subject to any subsequent
# five (5) year renewals, the U.S. Government is granted for itself
# and others acting on its behalf a paid-up, nonexclusive,
# irrevocable, worldwide license in the Software to reproduce,
# prepare derivative works, distribute copies to the public, perform
# publicly and display publicly, and to permit others to do so.
#
# This code is distributed under a BSD style license, see the LICENSE
# file for complete information.

# Initialize the autoconf system for the specified tool, version and mailing list
AC_PREREQ([2.71])
AC_INIT([iperf],[3.13],[https://github.com/esnet/iperf],[iperf],[https://software.es.net/iperf/])
m4_include([config/ax_check_openssl.m4])
m4_include([config/iperf_config_static_bin.m4])
AC_LANG(C)

# Specify where the auxiliary files created by configure should go. The config
# directory is picked so that they don't clutter up more useful directories.
AC_CONFIG_AUX_DIR(config)


# Initialize the automake system
AM_INIT_AUTOMAKE([foreign])
m4_ifdef([AM_SILENT_RULES], [AM_SILENT_RULES([yes])])
LT_INIT

AM_MAINTAINER_MODE
AC_CONFIG_HEADERS(src/iperf_config.h)

AC_CANONICAL_HOST

# Checks for tools: c compiler, ranlib (used for creating static libraries),
# symlinks and libtool
AC_PROG_CC
AC_PROG_LN_S
LT_INIT

# Add -Wall if we are using GCC.
if test "x$GCC" = "xyes"; then
  CFLAGS="$CFLAGS -Wall"
fi

# Check if enable profiling
AC_ARG_ENABLE([profiling],
    AS_HELP_STRING([--enable-profiling], [Enable iperf3 profiling binary]))
AM_CONDITIONAL([ENABLE_PROFILING], [test x$enable_profiling = xyes])

# Check for the math library (needed by cjson on some platforms)
AC_SEARCH_LIBS(floor, [m], [], [
echo "floor()"
exit 1
])

# On illumos we need -lsocket
AC_SEARCH_LIBS(socket, [socket], [], [
echo "socket()"
exit 1
])

# On illumos inet_ntop in in -lnsl
AC_SEARCH_LIBS(inet_ntop, [nsl], [], [
echo "inet_ntop()"
exit 1
])

# Checks for typedefs, structures, and compiler characteristics.
AC_C_CONST

# Check for poll.h (it's in POSIX so everyone should have it?)
AC_CHECK_HEADERS([poll.h])

# SCTP.  Allow user to disable SCTP support with --without-sctp.
# Otherwise we try to find whatever support is required.
try_sctp=true
AC_ARG_WITH([sctp],
    [AS_HELP_STRING([--without-sctp],
        [disable SCTP])],
    [
        case "$withval" in
	y | ye | yes)
	  ;;
	n | no)
	try_sctp=false
	  ;;
	*)
	AC_MSG_ERROR([Invalid --with-sctp value])
	  ;;
	esac
    ], [
        try_sctp=true
    ]
)

AC_CHECK_HEADERS([linux/tcp.h])

# Check for SCTP support
if $try_sctp; then
AC_CHECK_HEADERS([sys/socket.h])
AC_CHECK_HEADERS([netinet/sctp.h],
		 AC_DEFINE([HAVE_SCTP_H], [1], [Have SCTP support.])
		 AC_SEARCH_LIBS(sctp_bindx, [sctp])
		 AC_CHECK_TYPES([struct sctp_assoc_value], [], [],
				[[#include <netinet/sctp.h>]]),
		 [],
		 [#ifdef HAVE_SYS_SOCKET_H
#include <sys/socket.h>
#endif
])
fi

AC_CHECK_HEADER([endian.h],
		AC_DEFINE([HAVE_ENDIAN_H], [1], [Define to 1 if you have the <endian.h> header file.]),
		AC_CHECK_HEADER([sys/endian.h],
				AC_DEFINE([HAVE_SYS_ENDIAN_H], [1], [Define to 1 if you have the <sys/endian.h> header file.]),
				AC_MSG_WARN([Couldn't find endian.h or sys/endian.h files: doing compile-time tests.])
				)
		)

# Check if OpenSSL should be disabled
AC_ARG_ENABLE([openssl],
    [AS_HELP_STRING([--disable-openssl], [Disable OpenSSL support])],
    [
        case "$enableval" in
        yes|"")
            disable_openssl=false
            ;;
        no)
            disable_openssl=true
            with_openssl=no
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-openssl value])
            ;;
        esac
    ],
    [disable_openssl=false]
)

if test "x$with_openssl" = "xno" -o "x$disable_openssl" = "xtrue"; then
    AC_MSG_WARN( [Building without OpenSSL; disabling iperf_auth functionality.] )
else
    # Check for OPENSSL support
    have_ssl=false
    AX_CHECK_OPENSSL(
        [ AC_DEFINE([HAVE_SSL], [1], [OpenSSL Is Available])
          have_ssl=true ],
	[ if test "x$with_openssl" != "x"; then
	  AC_MSG_FAILURE([--with-openssl was given, but test for OpenSSL failed])
	  fi ]
    )
    if $have_ssl; then
        case $host in
           *-*-cygwin)
             CFLAGS="$CFLAGS -DNOCRYPT"
             ;;
        esac
        LDFLAGS="$LDFLAGS $OPENSSL_LDFLAGS"
        LIBS="$OPENSSL_LIBS $LIBS"
        CPPFLAGS="$OPENSSL_INCLUDES $CPPFLAGS"
    fi
fi

# Check for TCP_CONGESTION sockopt (believed to be Linux and FreeBSD only)
AC_CACHE_CHECK([TCP_CONGESTION socket option],
[iperf3_cv_header_tcp_congestion],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <netinet/tcp.h>]],
                   [[int foo = TCP_CONGESTION;]])],
  iperf3_cv_header_tcp_congestion=yes,
  iperf3_cv_header_tcp_congestion=no))

# Check if TCP_CONGESTION support should be disabled
AC_ARG_ENABLE([have-tcp-congestion],
    [AS_HELP_STRING([--disable-have-tcp-congestion], [Disable TCP_CONGESTION socket option support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_tcp_congestion=false
            ;;
        no)
            disable_have_tcp_congestion=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-tcp-congestion value])
            ;;
        esac
    ],
    [disable_have_tcp_congestion=false]
)

if test "x$disable_have_tcp_congestion" = "xtrue"; then
    AC_MSG_WARN([TCP_CONGESTION socket option support disabled by user])
else
    if test "x$iperf3_cv_header_tcp_congestion" = "xyes"; then
        AC_DEFINE([HAVE_TCP_CONGESTION], [1], [Have TCP_CONGESTION sockopt.])
    fi
fi

# Check for TCP_USER_TIMEOUT sockopt (believed to be Linux 2.6.37+ only)
AC_CACHE_CHECK([TCP_USER_TIMEOUT socket option],
[iperf3_cv_header_tcp_user_timeout],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <netinet/tcp.h>]],
                   [[int foo = TCP_USER_TIMEOUT;]])],
  iperf3_cv_header_tcp_user_timeout=yes,
  iperf3_cv_header_tcp_user_timeout=no))

# Check if TCP_USER_TIMEOUT support should be disabled
AC_ARG_ENABLE([have-tcp-user-timeout],
    [AS_HELP_STRING([--disable-have-tcp-user-timeout], [Disable TCP_USER_TIMEOUT socket option support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_tcp_user_timeout=false
            ;;
        no)
            disable_have_tcp_user_timeout=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-tcp-user-timeout value])
            ;;
        esac
    ],
    [disable_have_tcp_user_timeout=false]
)

if test "x$disable_have_tcp_user_timeout" = "xtrue"; then
    AC_MSG_WARN([TCP_USER_TIMEOUT socket option support disabled by user])
else
    if test "x$iperf3_cv_header_tcp_user_timeout" = "xyes"; then
        AC_DEFINE([HAVE_TCP_USER_TIMEOUT], [1], [Have TCP_USER_TIMEOUT sockopt.])
    fi
fi

# Check for IPv6 flowlabel support (believed to be Linux only)
# We check for IPV6_FLOWLABEL_MGR in <linux/in6.h> even though we
# don't use that file directly (we have our own stripped-down
# copy, see src/flowlabel.h for more details).
AC_CACHE_CHECK([IPv6 flowlabel support],
[iperf3_cv_header_flowlabel],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <sys/types.h>
                     #include <linux/in6.h>]],
                   [[int foo = IPV6_FLOWLABEL_MGR;]])],
  iperf3_cv_header_flowlabel=yes,
  iperf3_cv_header_flowlabel=no))

# Check if IPv6 flowlabel support should be disabled
AC_ARG_ENABLE([have-flowlabel],
    [AS_HELP_STRING([--disable-have-flowlabel], [Disable IPv6 flowlabel support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_flowlabel=false
            ;;
        no)
            disable_have_flowlabel=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-flowlabel value])
            ;;
        esac
    ],
    [disable_have_flowlabel=false]
)

if test "x$disable_have_flowlabel" = "xtrue"; then
    AC_MSG_WARN([IPv6 flowlabel support disabled by user])
else
    if test "x$iperf3_cv_header_flowlabel" = "xyes"; then
        AC_DEFINE([HAVE_FLOWLABEL], [1], [Have IPv6 flowlabel support.])
    fi
fi


# Check if CPU affinity support should be disabled
AC_ARG_ENABLE([cpu-affinity],
    [AS_HELP_STRING([--disable-cpu-affinity], [Disable CPU affinity support])],
    [
        case "$enableval" in
        yes|"")
            disable_cpu_affinity=false
            ;;
        no)
            disable_cpu_affinity=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-cpu-affinity value])
            ;;
        esac
    ],
    [disable_cpu_affinity=false]
)

if test "x$disable_cpu_affinity" = "xtrue"; then
    AC_MSG_WARN([CPU affinity support disabled by user])
else
    # Check for CPU affinity support.  FreeBSD and Linux do this differently
    # unfortunately so we have to check separately for each of them.
    # FreeBSD uses cpuset_setaffinity while Linux uses sched_setaffinity.
    # Define HAVE_CPU_AFFINITY to indicate the CPU affinity setting as a
    # generic concept is available.
    AC_CHECK_FUNCS([cpuset_setaffinity sched_setaffinity SetProcessAffinityMask],
           AC_DEFINE([HAVE_CPU_AFFINITY], [1], 
                 [Have CPU affinity support.]))
fi

# Check if daemon support should be disabled
AC_ARG_ENABLE([daemon],
    [AS_HELP_STRING([--disable-daemon], [Disable daemon support])],
    [
        case "$enableval" in
        yes|"")
            disable_daemon=false
            ;;
        no)
            disable_daemon=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-daemon value])
            ;;
        esac
    ],
    [disable_daemon=false]
)

if test "x$disable_daemon" = "xtrue"; then
    AC_MSG_WARN([Daemon support disabled by user])
else
    # Check for daemon().  Most systems have this but a few (IRIX) don't.
    AC_CHECK_FUNCS([daemon])
fi

# Check if sendfile support should be disabled
AC_ARG_ENABLE([sendfile],
    [AS_HELP_STRING([--disable-sendfile], [Disable sendfile support])],
    [
        case "$enableval" in
        yes|"")
            disable_sendfile=false
            ;;
        no)
            disable_sendfile=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-sendfile value])
            ;;
        esac
    ],
    [disable_sendfile=false]
)

if test "x$disable_sendfile" = "xtrue"; then
    AC_MSG_WARN([Sendfile support disabled by user])
else
    # Check for sendfile support.  FreeBSD, Linux, and MacOS all support
    # this system call, but they're all different in terms of what headers
    # it needs and what arguments it expects.
    AC_CHECK_FUNCS([sendfile])
fi

# Check for getline support, used as a part of authenticated
# connections.
AC_CHECK_FUNCS([getline])

# Check for packet pacing socket option (Linux only for now).
AC_CACHE_CHECK([SO_MAX_PACING_RATE socket option],
[iperf3_cv_header_so_max_pacing_rate],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <sys/socket.h>]],
                   [[int foo = SO_MAX_PACING_RATE;]])],
  iperf3_cv_header_so_max_pacing_rate=yes,
  iperf3_cv_header_so_max_pacing_rate=no))

# Check if SO_MAX_PACING_RATE support should be disabled
AC_ARG_ENABLE([have-so-max-pacing-rate],
    [AS_HELP_STRING([--disable-have-so-max-pacing-rate], [Disable SO_MAX_PACING_RATE socket option support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_so_max_pacing_rate=false
            ;;
        no)
            disable_have_so_max_pacing_rate=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-so-max-pacing-rate value])
            ;;
        esac
    ],
    [disable_have_so_max_pacing_rate=false]
)

if test "x$disable_have_so_max_pacing_rate" = "xtrue"; then
    AC_MSG_WARN([SO_MAX_PACING_RATE socket option support disabled by user])
else
    if test "x$iperf3_cv_header_so_max_pacing_rate" = "xyes"; then
        AC_DEFINE([HAVE_SO_MAX_PACING_RATE], [1], [Have SO_MAX_PACING_RATE sockopt.])
    fi
fi

# Check for SO_BINDTODEVICE sockopt (believed to be Linux only)
AC_CACHE_CHECK([SO_BINDTODEVICE socket option],
[iperf3_cv_header_so_bindtodevice],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <sys/socket.h>]],
                   [[int foo = SO_BINDTODEVICE;]])],
  iperf3_cv_header_so_bindtodevice=yes,
  iperf3_cv_header_so_bindtodevice=no))

# Check if SO_BINDTODEVICE support should be disabled
AC_ARG_ENABLE([have-so-bindtodevice],
    [AS_HELP_STRING([--disable-have-so-bindtodevice], [Disable SO_BINDTODEVICE socket option support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_so_bindtodevice=false
            ;;
        no)
            disable_have_so_bindtodevice=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-so-bindtodevice value])
            ;;
        esac
    ],
    [disable_have_so_bindtodevice=false]
)

if test "x$disable_have_so_bindtodevice" = "xtrue"; then
    AC_MSG_WARN([SO_BINDTODEVICE socket option support disabled by user])
else
    if test "x$iperf3_cv_header_so_bindtodevice" = "xyes"; then
        AC_DEFINE([HAVE_SO_BINDTODEVICE], [1], [Have SO_BINDTODEVICE sockopt.])
    fi
fi

# Check for IP_MTU_DISCOVER (mostly on Linux)
AC_CACHE_CHECK([IP_MTU_DISCOVER socket option],
[iperf3_cv_header_ip_mtu_discover],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <sys/types.h>
                     #include <sys/socket.h>
                     #include <netinet/in.h>]],
                   [[int foo = IP_MTU_DISCOVER;]])],
  iperf3_cv_header_ip_mtu_discover=yes,
  iperf3_cv_header_ip_mtu_discover=no))

# Check if IP_MTU_DISCOVER support should be disabled
AC_ARG_ENABLE([have-ip-mtu-discover],
    [AS_HELP_STRING([--disable-have-ip-mtu-discover], [Disable IP_MTU_DISCOVER socket option support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_ip_mtu_discover=false
            ;;
        no)
            disable_have_ip_mtu_discover=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-ip-mtu-discover value])
            ;;
        esac
    ],
    [disable_have_ip_mtu_discover=false]
)

if test "x$disable_have_ip_mtu_discover" = "xtrue"; then
    AC_MSG_WARN([IP_MTU_DISCOVER socket option support disabled by user])
else
    if test "x$iperf3_cv_header_ip_mtu_discover" = "xyes"; then
        AC_DEFINE([HAVE_IP_MTU_DISCOVER], [1], [Have IP_MTU_DISCOVER sockopt.])
    fi
fi

# Check for IP_DONTFRAG (BSD?)
AC_CACHE_CHECK([IP_DONTFRAG socket option],
[iperf3_cv_header_ip_dontfrag],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <sys/types.h>
                     #include <sys/socket.h>
                     #include <netinet/in.h>]],
                   [[int foo = IP_DONTFRAG;]])],
  iperf3_cv_header_ip_dontfrag=yes,
  iperf3_cv_header_ip_dontfrag=no))
if test "x$iperf3_cv_header_ip_dontfrag" = "xyes"; then
    AC_DEFINE([HAVE_IP_DONTFRAG], [1], [Have IP_DONTFRAG sockopt.])
fi

# Check for IP_DONTFRAGMENT (Windows?)
AC_CACHE_CHECK([IP_DONTFRAGMENT socket option],
[iperf3_cv_header_ip_dontfragment],
AC_COMPILE_IFELSE(
  [AC_LANG_PROGRAM([[#include <sys/types.h>
                     #include <sys/socket.h>
                     #include <netinet/in.h>]],
                   [[int foo = IP_DONTFRAGMENT;]])],
  iperf3_cv_header_ip_dontfragment=yes,
  iperf3_cv_header_ip_dontfragment=no))
if test "x$iperf3_cv_header_ip_dontfragment" = "xyes"; then
    AC_DEFINE([HAVE_IP_DONTFRAGMENT], [1], [Have IP_DONTFRAGMENT sockopt.])
fi

# Check for IP DF support
AC_CACHE_CHECK([any kind of DF socket option],
[iperf3_cv_header_dontfragment],
[if test "x$iperf3_cv_header_ip_mtu_discover" = "xyes" -o "x$iperf3_cv_header_ip_dontfrag" = "xyes" -o "x$iperf3_cv_header_ip_dontfragment" = "xyes"; then
  iperf3_cv_header_dontfragment=yes
else
  iperf3_cv_header_dontfragment=no
fi])

# Check if Don't Fragment support should be disabled
AC_ARG_ENABLE([have-dont-fragment],
    [AS_HELP_STRING([--disable-have-dont-fragment], [Disable Don't Fragment (DF) packet support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_dont_fragment=false
            ;;
        no)
            disable_have_dont_fragment=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-dont-fragment value])
            ;;
        esac
    ],
    [disable_have_dont_fragment=false]
)

if test "x$disable_have_dont_fragment" = "xtrue"; then
    AC_MSG_WARN([Don't Fragment (DF) packet support disabled by user])
else
    if test "x$iperf3_cv_header_dontfragment" = "xyes"; then
        AC_DEFINE([HAVE_DONT_FRAGMENT], [1], [Have IP_MTU_DISCOVER/IP_DONTFRAG/IP_DONTFRAGMENT sockopt.])
    fi
fi

#
# Check for tcpi_snd_wnd in struct tcp_info
#
AC_CHECK_MEMBER([struct tcp_info.tcpi_snd_wnd],
[iperf3_cv_header_tcp_info_snd_wnd=yes], [iperf3_cv_header_tcp_info_snd_wnd=no],
[#ifdef HAVE_LINUX_TCP_H
#include <linux/tcp.h>
#else
#include <sys/types.h>
#include <netinet/tcp.h>
#endif
])

# Check if TCP send window support should be disabled
AC_ARG_ENABLE([have-tcp-snd-wnd],
    [AS_HELP_STRING([--disable-have-tcp-snd-wnd], [Disable TCP send window information support])],
    [
        case "$enableval" in
        yes|"")
            disable_have_tcp_snd_wnd=false
            ;;
        no)
            disable_have_tcp_snd_wnd=true
            ;;
        *)
            AC_MSG_ERROR([Invalid --enable-have-tcp-snd-wnd value])
            ;;
        esac
    ],
    [disable_have_tcp_snd_wnd=false]
)

if test "x$disable_have_tcp_snd_wnd" = "xtrue"; then
    AC_MSG_WARN([TCP send window information support disabled by user])
else
    if test "x$iperf3_cv_header_tcp_info_snd_wnd" = "xyes"; then
        AC_DEFINE([HAVE_TCP_INFO_SND_WND], [1], [Have tcpi_snd_wnd field in tcp_info.])
    fi
fi

# Check if we need -lrt for clock_gettime
AC_SEARCH_LIBS(clock_gettime, [rt posix4])
# Check for clock_gettime support
AC_CHECK_FUNCS([clock_gettime])

AC_CONFIG_FILES([Makefile src/Makefile src/version.h iperf3.spec])
AC_OUTPUT
