#pragma once
#define BUFFER_SIZE 32
typedef enum {
  OutOfRange = -1,
  NotAvailable = -2,
} ErrorCode;
int set_global_variable(void* data, unsigned int pos);
void* get_global_variable(unsigned int pos);

int clear_variable(unsigned int pos);
int has_data_at(unsigned int pos);
int is_null_ptr(void* ptr);