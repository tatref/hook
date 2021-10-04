// gcc main.c

#include <stdio.h>
#include <unistd.h>
#include <string.h>

int main()
{
  char s1[100] = "test string";
  printf("%s: %d\n", s1, strlen(s1));

  char s2[] = "string 2";
  printf("%s: %d\n", s2, strlen(s2));
  return 0;
}
