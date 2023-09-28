#ifndef clox_value_h
#define clox_value_h

#include "common.h"

//Lox values
typedef double Value;

//Dynamic array of static values
typedef struct {
    int capacity;
    int count;
    Value* values;
} ValueArray;

void initValueArray(ValueArray* array);
void writeValueArray(ValueArray* array, Value value);
void freeValueArray(ValueArray* array);
void printValue(Value value);

#endif
