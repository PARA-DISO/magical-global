#include "magical-global.h"

void* global_buffer[BUFFER_SIZE] = {0};
/**
 * @brief Set the global variable object
 * 
 * @param data 
 * @param pos 
 * @return int return 0 if success, otherwise return -1
 */
int set_global_variable(void* data, unsigned int pos) {
  if (pos < BUFFER_SIZE) {
    global_buffer[pos] = data;
    return 0;
  } else {
    return -1;
  }
}
/**
 * @brief Get the global variable object
 * 
 * @param pos 
 * @return void* 
 */
void* get_global_variable(unsigned int pos) {
   if (pos < BUFFER_SIZE) {
    void* ptr = global_buffer[pos];
    return ptr;
  } else {
    return 0;
  }
}
/**
 * @brief 
 * 
 * @param pos 
 * @return int return 0 if success, otherwise return -1
 */
int clear_variable(unsigned int pos) {
  if (pos < BUFFER_SIZE) {
    global_buffer[pos] = 0;
    return 0;
  } else {
    return -1;
  }
}
/**
 * @brief 
 * 
 * @param pos 
 * @return int return 1 if the data is in the buffer, otherwise return 0
 */
int has_data_at(unsigned int pos) {
  if (pos < BUFFER_SIZE && global_buffer[pos] != 0) {
    return 1;
  } else {
    return 0;
  }
}
/**
 * @brief 
 * 
 * @param ptr
 * @return int return 1 if the pointer is null, otherwise return 0
 */
int is_null_ptr(void* ptr) {
  return ptr == 0;
}