#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CFProgress {
  CFDone = 0,
  CFMore = 1,
  CFError = 2,
} CFProgress;

/**
 * An enum that represents the return status of the streaming API
 *
 * CFSDone means we have a result
 * CFSMore means we can add headers or body, or run the analysis
 * CFSError means there is an error, that can be read using curiefense_stream_error
 */
typedef enum CFStreamStatus {
  CFSDone = 0,
  CFSMore = 1,
  CFSError = 2,
} CFStreamStatus;

typedef struct CFExec CFExec;

typedef struct CFHashmap CFHashmap;

typedef struct CFResult CFResult;

/**
 * C streaming API configuration item
 */
typedef struct CFStreamConfig CFStreamConfig;

/**
 * Handle for the C streaming API
 */
typedef struct CFStreamHandle CFStreamHandle;

/**
 * # Safety
 *
 * New C hashmap
 */
struct CFHashmap *cf_hashmap_new(void);

/**
 * # Safety
 *
 * Insert into the hashmap. The key and value are not consumed by this API (it copies them).
 */
void cf_hashmap_insert(struct CFHashmap *hm,
                       const char *key,
                       uintptr_t key_size,
                       const char *value,
                       uintptr_t value_size);

/**
 * # Safety
 *
 * Frees a hashmap, and all its content.
 */
void cf_hashmap_free(struct CFHashmap *ptr);

/**
 * # Safety
 *
 * Returns false is the decision is to let pass, true otherwise.
 */
bool curiefense_cfr_is_blocking(const struct CFResult *ptr);

/**
 * # Safety
 *
 * Returns the status code of a blocking action.
 */
uint32_t curiefense_cfr_block_status(const struct CFResult *ptr);

/**
 * # Safety
 *
 * Returns the content length of a blocking action.
 */
uintptr_t curiefense_cfr_block_contentlength(const struct CFResult *ptr);

/**
 * # Safety
 *
 * Copies the body of a blocking action. The input buffer must have a size that is larger than
 * what the curiefense_str_block_contentlength returned.
 */
void curiefense_cfr_block_content(const struct CFResult *ptr, unsigned char *tgt);

/**
 * # Safety
 *
 * Returns the log string, json encoded. Can be freed with curiefense_str_free.
 */
char *curiefense_cfr_log(struct CFResult *ptr, uintptr_t *ln);

/**
 * # Safety
 *
 * Populate the curiefense log string (json encoded)
 */
void curiefense_cfr_logs(struct CFResult *ptr,
                         void (*cb)(uint8_t, const char*, void*),
                         void *cb_data);

/**
 * # Safety
 *
 * Returns the error, if available. The returned string can be freed with curiefense_str_free.
 */
char *curiefense_cfr_error(const struct CFResult *ptr);

/**
 * # Safety
 *
 * Frees a string that has been returned by this API.
 */
void curiefense_str_free(char *ptr);

/**
 * # Safety
 *
 * Initializes the inspection, returning an executor in case of success, or a null pointer in case of failure.
 *
 * Note that the hashmaps raw_meta and raw_headers are consumed and freed by this function.
 *
 * Arguments
 *
 * loglevel:
 *     0. debug
 *     1. info
 *     2. warning
 *     3. error
 * raw_configpath: path to the configuration directory
 * raw_meta: hashmap containing the meta properties.
 *     * required: method and path
 *     * technically optional, but highly recommended: authority, x-request-id
 * raw_headers: hashmap containing the request headers
 * raw_ip: a string representing the source IP for the request
 * mbody: body as a single buffer, or NULL if no body is present
 * mbody_len: length of the body. It MUST be 0 if mbody is NULL.
 * cb: the callback that will be used to signal an asynchronous function finished
 * data: data for the callback
 */
struct CFExec *curiefense_async_init(uint8_t loglevel,
                                     const char *raw_configpath,
                                     struct CFHashmap *raw_meta,
                                     struct CFHashmap *raw_headers,
                                     const char *raw_ip,
                                     const unsigned char *mbody,
                                     uintptr_t mbody_len,
                                     void (*cb)(uint64_t),
                                     uint64_t data);

/**
 * # Safety
 *
 * Steps a valid executor. Note that the executor is freed when CFDone is returned, and the pointer
 * is no longer valid.
 */
enum CFProgress curiefense_async_step(struct CFExec *ptr, struct CFResult **out);

/**
 * # Safety
 *
 * Frees the executor, should be run with the output of executor_init, and only once.
 * Generally, you should wait until the step function returns CFDone, but you can use
 * this function to abort early.
 */
void curiefense_async_free(struct CFExec *ptr);

/**
 * # Safety
 *
 * Returns a configuration handle for the stream API. Must be called when configuration changes.
 * Is freed using curiefense_stream_config_free
 */
struct CFStreamConfig *curiefense_stream_config_init(uint8_t loglevel, const char *raw_configpath);

/**
 * # Safety
 *
 * frees the CFStreamConfig object
 *
 * note that it is perfectly safe to free it while other requests are being processed, as the underlying
 * data is protected by refcounted pointers.
 */
void curiefense_stream_config_free(struct CFStreamConfig *config);

/**
 * # Safety
 *
 * Initializes the inspection, returning a stream object.
 * This never returns a null pointer, even if the function fails.
 * In case of failure, you can get the error message by calling the
 * curiefense_stream_error function on the returned object
 *
 * Note that the hashmap raw_meta is freed by this function.
 *
 * Arguments
 *
 * loglevel:
 *     0. debug
 *     1. info
 *     2. warning
 *     3. error
 * raw_configpath: path to the configuration directory
 * raw_meta: hashmap containing the meta properties.
 *     * required: method and path
 *     * technically optional, but highly recommended: authority, x-request-id
 * raw_ip: a string representing the source IP for the request
 * success: a pointer to a value that will be set to true on success, and false on failure
 */
struct CFStreamHandle *curiefense_stream_start(const struct CFStreamConfig *config,
                                               struct CFHashmap *raw_meta,
                                               const char *raw_ip,
                                               enum CFStreamStatus *success);

/**
 * # Safety
 *
 * Frees the stream object.
 *
 * You should use this function, when aborting:
 *  * the object is in an error state, and you already retrieved the error message from curiefense_stream_error
 *  * you want to abort early
 */
void curiefense_stream_free(struct CFStreamHandle *ptr);

/**
 * # Safety
 *
 * Returns the streaming error, if available. The returned string can be freed with curiefense_str_free.
 */
char *curiefense_stream_error(const struct CFStreamHandle *ptr);

/**
 * # Safety
 *
 * Adds a header to the stream handle object
 */
enum CFStreamStatus curiefense_stream_add_header(struct CFStreamHandle **sh,
                                                 const char *key,
                                                 uintptr_t key_size,
                                                 const char *value,
                                                 uintptr_t value_size);

/**
 * # Safety
 *
 * Adds a body part to the stream handle object
 */
enum CFStreamStatus curiefense_stream_add_body(struct CFStreamHandle **sh,
                                               const uint8_t *body,
                                               uintptr_t body_size);

/**
 * # Safety
 *
 * Runs the analysis on the stream handle object. If the stream handle object is in an error state,
 * this will return a null pointer.
 *
 * Note that the CFStreamHandle object is freed by this function, even when it represents an error.
 *
 * cb: the callback that will be used to signal an asynchronous function finished
 * data: data for the callback
 */
struct CFExec *curiefense_stream_exec(const struct CFStreamConfig *config,
                                      struct CFStreamHandle *sh,
                                      void (*cb)(uint64_t),
                                      uint64_t data);
