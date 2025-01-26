#include "libfl.h"
#include <fenv.h>

int pushround(enum RoundingMode rm)
{
    int curround = fegetround();

    switch (rm)
    {
    case RMTONEAREST:
        fesetround(FE_TONEAREST);
        break;
    case RMTOWARDZERO:
        fesetround(FE_TOWARDZERO);
        break;
    case RMUPWARD:
        fesetround(FE_UPWARD);
        break;
    case RMDOWNWARD:
        fesetround(FE_DOWNWARD);
        break;
    }

    return curround;
}

void popround(int curround)
{
    fesetround(curround);
}

void clearexcept()
{
    feclearexcept(FE_ALL_EXCEPT);
}

void getexcept(uint32_t *out)
{
    int exception = fetestexcept(FE_ALL_EXCEPT);

    *out = 0;

    if (exception & FE_DIVBYZERO)
        *out |= EXDIVBYZERO;
    if (exception & FE_INVALID)
        *out |= EXINVALID;
    if (exception & FE_OVERFLOW)
        *out |= EXOVERFLOW;
    if (exception & FE_UNDERFLOW)
        *out |= EXUNDERFLOW;
    if (exception & FE_INEXACT)
        *out |= EXINEXACT;
}

void f32_add(float a, float b, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushround(rm);
    clearexcept();

    float res = a + b;

    getexcept(&out->exception);
    popround(curround);

    out->value = res;
}

void f32_div(float a, float b, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushround(rm);
    clearexcept();

    float res = a / b;

    getexcept(&out->exception);
    popround(curround);

    out->value = res;
}
