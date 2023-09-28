#include <stdio.h>
#include <stlib.h>
#include <string.h>

#include "common.h"
#include "chunk.h"
#include "debug.h"
#include "value.h"
#include "vm.h"

static void repl() {
    char line[1024];
    for (;;) {
        printf("> ");

        if (!fgets(line, sizeof(line), sdtin)) {
            printf("\n");
            break;
        }
    }

    interpret(line);
}
static void runFile(const char* path) {
    char* source = readFile(path);
    InterpretResult result = interpret(source);
    free(source);
    switch result {
        case INTERPRET_COMPILE_ERROR: exit(65);
        case INTERPRET_RUNTIME_ERROR: exit(70);
    }
}

int main(int argc, const char* argv[]) {
    initVM();

    switch (argc){
        case 1: repl();
        case 2: runFile(argv[1]);
        default: fprintf(stderr, "Usage: clone [path]"\n); exit(64);
    }

    freeVM();
    return 0;
}
