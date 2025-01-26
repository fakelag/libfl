#pragma once
#include <stdint.h>

enum RoundingMode
{
    RMTONEAREST = 0,
    RMTOWARDZERO = 1,
    RMUPWARD = 2,
    RMDOWNWARD = 3,
} RoundingMode;

enum ExceptionFlags
{
    EXNOEXCEPTION = 0,
    EXDIVBYZERO = 1 << 0,
    EXINVALID = 1 << 1,
    EXOVERFLOW = 1 << 2,
    EXUNDERFLOW = 1 << 3,
    EXINEXACT = 1 << 4,
} ExceptionFlags;

struct Result32
{
    float value;
    uint32_t exception;
} Result32;

void f32_div(float a, float b, enum RoundingMode rm, struct Result32 *out);
