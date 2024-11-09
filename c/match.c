/******************************************************************************

                            Online C Compiler.
                Code, Compile, Run and Debug C program online.
Write your code in this editor and press "Run" button to compile and execute it.

*******************************************************************************/

#include <stdio.h>
#include <string.h>

char corresponding_bracket(char c) {
    if(c == ')' ) {
      return '(';  
    }
    if(c == ']' ) {
      return '[';  
    }
    if(c == '}' ) {
      return '{';  
    }
    return 0;
}

int check(char* str){
    const char* open = "([{";
    const char* close = ")]}";
    char stack[10] = "";
    char c;
    int len = 0;
    int s = 0;
    int i = 0;
    char* p;
    
    len = strlen(str);
    printf("check matching brackets for \'%s\'[len=%i]\n",str,len);
    while (i < len) {
        c = str[i];
        printf("c>[%c]\n",c);
        //check open
        p = strchr(open, c);
        if(p != NULL) {
            printf("open[%s]> %c - %c\n", open, *p, c);
            stack[s++] = c;
        }
        //ckeck close
        p = strchr(close, c);
        if (p != NULL) {
            printf("close[%s]> %c - %c\n", close, *p, c);
            if(s > 0 && corresponding_bracket(c) == stack[s-1]){
                s--;
            } else {
                return -1;
            }
        }
        i++;
    }
    if(s == 0) {
        return 0;
    }
    return -1;
}

int main()
{
    int result = 0;
    char str[100] = "a[b]";
    
    result = check(str);
    if(result == 0) {
       printf(": %s is ok!",str); 
    } else {
        printf(": %s do not match",str);
    }
    
    
    return 0;
}
