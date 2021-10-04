// gcc main.c

#include <stdio.h>
#include <unistd.h>
#include <string.h>

int main()
{
  char s1[] = "1234";

  char s2[] = "some secret string";
  if (strcmp(s1, s2) == 0) {
    printf("equal!\n");
  }
  else {
    printf("different!\n");
  }
  return 0;
}
