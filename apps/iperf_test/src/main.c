
#include "iperf_config.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <getopt.h>
#include <errno.h>
#include <signal.h>
#include <unistd.h>
#ifdef HAVE_STDINT_H
#include <stdint.h>
#endif
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>

#include "iperf.h"
#include "iperf_api.h"
#include "iperf_util.h"
#include "iperf_locale.h"
#include "net.h"
#include "units.h"


static int run(struct iperf_test *test);


/**************************************************************************/
int
main(int argc, char **argv)
{
    struct iperf_test *test;

    test = iperf_new_test();
    if (!test)
        iperf_errexit(NULL, "create new test error - %s", iperf_strerror(i_errno));
    iperf_defaults(test);	/* sets defaults */
    if (iperf_parse_arguments(test, argc, argv) < 0) {
        iperf_err(test, "parameter error - %s", iperf_strerror(i_errno));
        fprintf(stderr, "\n");
        usage();
        exit(1);
    }
    printf("test role: %c\n", test->role);
    if (run(test) < 0)
        iperf_errexit(test, "error - %s", iperf_strerror(i_errno));

    // iperf_free_test(test);

    // return 0;
}


static jmp_buf sigend_jmp_buf;

static void __attribute__ ((noreturn))
sigend_handler(int sig)
{
    longjmp(sigend_jmp_buf, 1);
}

/**************************************************************************/
static int
run(struct iperf_test *test)
{
    printf("beginning run\n");
    /* Termination signals. */
    iperf_catch_sigend(sigend_handler);
    if (setjmp(sigend_jmp_buf)){
        printf("caught SIGEND\n");
        iperf_got_sigend(test);
    }
	    

    /* Ignore SIGPIPE to simplify error handling */
    printf("ignoring SIGPIPE\n");
    signal(SIGPIPE, SIG_IGN);

    printf("test role: %c\n", test->role);
    switch (test->role) {
        case 's':
            printf("running server\n");
            // if (iperf_create_pidfile(test) < 0) {
            //     i_errno = IEPIDFILE;
            //     iperf_errexit(test, "error - %s", iperf_strerror(i_errno));
            // }
            // for (;;) {
            //     int rc;
            //     rc = iperf_run_server(test);
            //     test->server_last_run_rc = rc;
            //     if (rc < 0) {
            //         iperf_err(test, "error - %s", iperf_strerror(i_errno));
            //                 if (test->json_output) {
            //                     if (iperf_json_finish(test) < 0)
            //                         return -1;
            //                 }
            //                 iflush(test);

            //         if (rc < -1) {
            //             iperf_errexit(test, "exiting");
            //         }
            //             }
            //             iperf_reset_test(test);
            //             if (iperf_get_test_one_off(test) && rc != 2) {
            //         /* Authentication failure doesn't count for 1-off test */
            //         if (rc < 0 && i_errno == IEAUTHTEST) {
            //         continue;
            //         }
            //         break;
            //     }
            // }
	        // iperf_delete_pidfile(test);
            break;
        case 'c':
            printf("running client\n");
            if (iperf_create_pidfile(test) < 0) {
                i_errno = IEPIDFILE;
                iperf_errexit(test, "error - %s", iperf_strerror(i_errno));
            }
            if (iperf_run_client(test) < 0)
                iperf_errexit(test, "error - %s", iperf_strerror(i_errno));
            iperf_delete_pidfile(test);
            break;
        default:
            printf("default case\n");
            usage();
            break;
    }

    iperf_catch_sigend(SIG_DFL);
    signal(SIGPIPE, SIG_DFL);

    return 0;
}
