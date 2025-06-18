
#ifndef cJSON__h
#define cJSON__h


#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>


#define CJSON_CDECL
#define CJSON_STDCALL

#define CJSON_PUBLIC(type) type

/* project version */
#define CJSON_VERSION_MAJOR 1
#define CJSON_VERSION_MINOR 7
#define CJSON_VERSION_PATCH 15

#include <stddef.h>

/* cJSON Types: */
#define cJSON_Invalid (0)
#define cJSON_False  (1 << 0)
#define cJSON_True   (1 << 1)
#define cJSON_NULL   (1 << 2)
#define cJSON_Number (1 << 3)
#define cJSON_String (1 << 4)
#define cJSON_Array  (1 << 5)
#define cJSON_Object (1 << 6)
#define cJSON_Raw    (1 << 7) /* raw json */

#define cJSON_IsReference 256
#define cJSON_StringIsConst 512

/* The cJSON structure: */
typedef struct cJSON
{
    /* next/prev allow you to walk array/object chains. Alternatively, use GetArraySize/GetArrayItem/GetObjectItem */
    struct cJSON *next;
    struct cJSON *prev;
    /* An array or object item will have a child pointer pointing to a chain of the items in the array/object. */
    struct cJSON *child;

    /* The type of the item, as above. */
    int type;

    /* The item's string, if type==cJSON_String  and type == cJSON_Raw */
    char *valuestring;
    /* writing to valueint is DEPRECATED, use cJSON_SetNumberValue instead */
    int64_t valueint;
    /* The item's number, if type==cJSON_Number */
    double valuedouble;

    /* The item's name string, if this item is the child of, or is in the list of subitems of an object. */
    char *string;
} cJSON;

typedef struct internal_hooks
{
    void *(CJSON_CDECL *allocate)(size_t size);
    void (CJSON_CDECL *deallocate)(void *pointer);
    void *(CJSON_CDECL *reallocate)(void *pointer, size_t size);
} internal_hooks;

#define internal_malloc malloc
#define internal_free free
#define internal_realloc realloc

/* strlen of character literals resolved at compile time */
#define static_strlen(string_literal) (sizeof(string_literal) - sizeof(""))

static internal_hooks global_hooks = { internal_malloc, internal_free, internal_realloc };

#endif

/* Internal constructor. */
static cJSON *cJSON_New_Item(const internal_hooks * const hooks)
{
    printf("cJSON_New_Item: %p\n", hooks);
    printf("Validating global_hooks: %p\n", &global_hooks);
    printf("global_hooks.allocate: %p\n", global_hooks.allocate);
    printf("global_hooks.deallocate: %p\n", global_hooks.deallocate);
    printf("global_hooks.reallocate: %p\n", global_hooks.reallocate);
    cJSON* node = (cJSON*)hooks->allocate(sizeof(cJSON));
    printf("node: %p\n", node);
    if (node)
    {
        memset(node, '\0', sizeof(cJSON));
    }

    return node;
}

CJSON_PUBLIC(cJSON *) cJSON_CreateObject(void)
{
    printf("cJSON_CreateObject\n");
    cJSON *item = cJSON_New_Item(&global_hooks);
    printf("item = %p\n", item);
    if (item)
    {
        item->type = cJSON_Object;
    }

    return item;
}

void main(){
    cJSON *j;
    j = cJSON_CreateObject();
    printf("j = %p\n", j);
}




