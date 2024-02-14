#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include "smartsocket.h"

int main(int argc, char *argv[])
{
  int   rc = EXIT_SUCCESS;
  const char *state =  NULL;
  SmartSocket *ss = smart_socket_new();

  printf("Starting to use SmartSocket from lib\n");

  printf("Turn on SmartSocket\n");
  assert (smart_socket_on(ss) == 0 || "Can't turn on smart socket");
  state = smart_socket_state(ss);
  printf("SmartSocket state:%s\n", state);

  printf ("Turn off SmartSocket\n");
  assert (smart_socket_off(ss) == 0 || "Can't turn off smart_socket");
  state = smart_socket_state(ss);
  printf("SmartSocket state:%s\n", state);
 
  free (ss);

  printf("Done!\n");

  return (rc);
}
