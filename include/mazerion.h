/**
 * Mazerion FFI - C Interface
 *
 * Provides C-compatible bindings for Mazerion calculator library.
 * All strings are UTF-8 encoded, null-terminated C strings.
 */

#ifndef MAZERION_FFI_H
#define MAZERION_FFI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Error codes returned by Mazerion functions
 */
typedef enum {
    MAZERION_SUCCESS = 0,
    MAZERION_ERROR_NULL_CALC_ID = 1,
    MAZERION_ERROR_NULL_JSON = 2,
    MAZERION_ERROR_INVALID_UTF8_CALC_ID = 3,
    MAZERION_ERROR_INVALID_UTF8_JSON = 4,
    MAZERION_ERROR_CALC_ID_MISMATCH = 5,
    MAZERION_ERROR_JSON_PARSE = 6,
    MAZERION_ERROR_PANIC = 7,
    MAZERION_ERROR_CALC_NOT_FOUND = 8,
    MAZERION_ERROR_CALCULATION = 9
} MazerionErrorCode;

/**
 * Error structure containing code and message
 */
typedef struct {
    int32_t code;
    char* message;  // NULL if success, allocated string otherwise
} MazerionError;

/**
 * Result structure for API calls
 */
typedef struct {
    MazerionError error;
    char* json_output;  // NULL on error, allocated JSON string on success
} MazerionResult;

/**
 * Initialize the Mazerion calculator system.
 * Must be called before any other functions.
 *
 * @return 0 on success, error code on failure
 */
int32_t mazerion_init(void);

/**
 * Get the Mazerion library version.
 *
 * @return Allocated string with version (must be freed with mazerion_free_string)
 */
char* mazerion_version(void);

/**
 * List all available calculators.
 *
 * @return MazerionResult with JSON array of calculator info
 *         Each calculator has: id, name, description, category
 *
 * Example output:
 * [
 *   {
 *     "id": "abv",
 *     "name": "ABV Calculator",
 *     "description": "Calculate alcohol by volume",
 *     "category": "Basic"
 *   },
 *   ...
 * ]
 */
MazerionResult mazerion_list_calculators(void);

/**
 * Get calculator categories with counts.
 *
 * @return MazerionResult with JSON object mapping categories to counts
 *
 * Example output:
 * {
 *   "Basic": 8,
 *   "Advanced": 6,
 *   "Brewing": 5
 * }
 */
MazerionResult mazerion_get_categories(void);

/**
 * Execute a calculation.
 *
 * SAFETY: The caller must ensure that:
 * - calculator_id points to a valid, null-terminated C string
 * - json_input points to a valid, null-terminated C string
 * - Both strings remain valid for the duration of this call
 * - Both strings contain valid UTF-8 data
 *
 * @param calculator_id ID of the calculator to use (e.g., "abv", "dilution")
 * @param json_input JSON string with calculation parameters
 *
 * The json_input must be a valid JSON object with:
 * {
 *   "calculator_id": "same_as_first_param",
 *   "params": {
 *     "param1": "value1",
 *     "param2": "value2",
 *     ...
 *   }
 * }
 *
 * @return MazerionResult with calculation result as JSON
 *
 * Example output:
 * {
 *   "value": "14.25",
 *   "unit": "%",
 *   "warnings": ["Gravity reading below 1.000"],
 *   "metadata": {
 *     "formula": "Standard ABV",
 *     "calories": "1500"
 *   }
 * }
 */
MazerionResult mazerion_calculate(const char* calculator_id, const char* json_input);

/**
 * Free a string allocated by Mazerion (e.g., from mazerion_version).
 *
 * SAFETY: The caller must ensure that:
 * - ptr was allocated by a Mazerion function (e.g., mazerion_version)
 * - ptr has not been freed already
 * - ptr is not used after this call
 *
 * @param ptr String to free (can be NULL)
 */
void mazerion_free_string(char* ptr);

/**
 * Free a MazerionResult structure.
 * This frees both the error message and JSON output if allocated.
 *
 * SAFETY: The caller must ensure that:
 * - result was returned by a Mazerion function
 * - result has not been freed already
 * - result is not used after this call
 *
 * @param result Result structure to free
 */
void mazerion_free_result(MazerionResult result);

#ifdef __cplusplus
}
#endif

#endif /* MAZERION_FFI_H */