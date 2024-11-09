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
    char open[4] = "([{\0";
    char close[4] = ")]}\0";
    char stack[10] = "";
    char c;
    int len = 0;
    int s = 0;
    int i = 0;
    
    len = strlen(str);
    printf("check matching brackets for %s[%i]\n",str,len);
    while (i < len) {
        c = str[i];
        if(strchr(open,c) != NULL) {
            stack[s++] = c;
        } else if (strchr(close,c) != NULL) {
            if(s > 0 && corresponding_bracket(c) == stack[s-1]){
                s--;
                continue; //ok : matched
            } else {
                return -1;
            }
        }
        i++;
    }
    return 0;
}

int main()
{
    int result = 0;
    char str[100] = "a(b)";
    
    result = check(str);
    if(result == 0) {
       printf(": %s is ok!",str); 
    } else {
        printf(": %s do not match",str);
    }
    
    
    return 0;
}
