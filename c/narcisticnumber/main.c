#include <stdbool.h>
#include <stdio.h>
#include <math.h>

bool narcissistic(int num) {
    int nod = 1;
    bool found = false;
    while (!found) {
        if ((int)(num / pow(10,nod)) == 0) {
            found=true;
        } else {
            nod++;
        }
    }
    int sum = 0;
    int temp = num;
    while(temp!= 0) {
        sum += pow(temp%10, nod);
        temp /= 10;
    }
    return (sum==num);
}

#include <criterion/criterion.h>

bool narcissistic(int num);

Test(Sample_Test, should_return_whether_a_number_is_narcissistic) {
    cr_assert(narcissistic(7));
    cr_assert(narcissistic(371));
    cr_assert_not(narcissistic(122));
    cr_assert_not(narcissistic(4887));
}
