#include "libfl.h"
#include <fenv.h>
#include <math.h>

int pushroundexc(enum RoundingMode rm)
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

    feclearexcept(FE_ALL_EXCEPT);
    return curround;
}

void poproundexc(int prevround, uint32_t *outexc)
{
    int exception = fetestexcept(FE_ALL_EXCEPT);

    *outexc = 0;

    if (exception & FE_DIVBYZERO)
        *outexc |= EXDIVBYZERO;
    if (exception & FE_INVALID)
        *outexc |= EXINVALID;
    if (exception & FE_OVERFLOW)
        *outexc |= EXOVERFLOW;
    if (exception & FE_UNDERFLOW)
        *outexc |= EXUNDERFLOW;
    if (exception & FE_INEXACT)
        *outexc |= EXINEXACT;

    fesetround(prevround);
}

void add_f32(float a, float b, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushroundexc(rm);

    float res = a + b;

    poproundexc(curround, &out->exception);
    out->value.f = res;
}

void div_f32(float a, float b, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushroundexc(rm);

    float res = a / b;

    poproundexc(curround, &out->exception);
    out->value.f = res;
}

void mul_f32(float a, float b, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushroundexc(rm);

    float res = a * b;

    poproundexc(curround, &out->exception);
    out->value.f = res;
}

void cvt_u32_f32(uint32_t val, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushroundexc(rm);
    float res = (float)val;

    poproundexc(curround, &out->exception);
    out->value.f = res;
}

void cvt_f32_u32(float val, enum RoundingMode rm, struct Result32 *out)
{
    int curround = pushroundexc(rm);
    int32_t res = (int32_t)rintf(val);

    poproundexc(curround, &out->exception);
    out->value.i = res;
}
