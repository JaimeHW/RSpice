#include <stddef.h>

#if defined(__APPLE__)
#include <stdint.h>
#include <string.h>
#include <libkern/OSCacheControl.h>
#include <pthread.h>

struct rspice_jit_copy_context {
    uint8_t *arena_start;
    uint8_t *arena_end;
    uint8_t *destination;
    const uint8_t *source;
    size_t length;
};

static int rspice_jit_copy_callback(void *opaque) {
    const struct rspice_jit_copy_context *context =
        (const struct rspice_jit_copy_context *)opaque;
    if (context == NULL || context->arena_start == NULL ||
        context->arena_end == NULL || context->destination == NULL ||
        context->source == NULL || context->arena_start > context->arena_end) {
        return -1;
    }

    const uintptr_t arena_start = (uintptr_t)context->arena_start;
    const uintptr_t arena_end = (uintptr_t)context->arena_end;
    const uintptr_t destination = (uintptr_t)context->destination;
    if (context->length == 0 || destination < arena_start ||
        context->length > arena_end - arena_start ||
        destination > arena_end - context->length) {
        return -1;
    }

    memcpy(context->destination, context->source, context->length);
    return 0;
}

PTHREAD_JIT_WRITE_ALLOW_CALLBACKS_NP(rspice_jit_copy_callback);

int rspice_jit_publish(void *arena_start, void *arena_end, void *destination,
                       const void *source, size_t length) {
    struct rspice_jit_copy_context context = {
        .arena_start = (uint8_t *)arena_start,
        .arena_end = (uint8_t *)arena_end,
        .destination = (uint8_t *)destination,
        .source = (const uint8_t *)source,
        .length = length,
    };
    return pthread_jit_write_with_callback_np(rspice_jit_copy_callback, &context);
}

void rspice_clear_instruction_cache(void *start, size_t length) {
    sys_icache_invalidate(start, length);
}

#else

void rspice_clear_instruction_cache(void *start, size_t length) {
    __builtin___clear_cache((char *)start, (char *)start + length);
}

#endif
