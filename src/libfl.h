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

union Val32 {
    int32_t i;
    float f;
};

struct Result32
{
    union Val32 value;
    uint32_t exception;
} Result32;

void add_f32(float a, float b, enum RoundingMode rm, struct Result32 *out);
void div_f32(float a, float b, enum RoundingMode rm, struct Result32 *out);
void mul_f32(float a, float b, enum RoundingMode rm, struct Result32 *out);
void cvt_u32_f32(uint32_t val, enum RoundingMode rm, struct Result32 *out);
void cvt_f32_u32(float val, enum RoundingMode rm, struct Result32 *out);
