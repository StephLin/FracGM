#include <vector>

#include "Eigen/Dense"

Eigen::Matrix3Xd to_eigen_pc(double *data, int n_cols) { return Eigen::Map<Eigen::Matrix3Xd>(data, 3, n_cols); }

struct CBufferI32 {
  int *data;
  size_t len;
};

CBufferI32 to_c_int_buffer(const std::vector<int> &array) {
  CBufferI32 buf;

  int len = static_cast<int>(array.size());

  buf.data = new int[len];
  for (int i = 0; i < len; i++) {
    buf.data[i] = array[i];
  }

  buf.len = len;

  return buf;
}

extern "C" {
void free_c_int_buffer(CBufferI32 buf) {
  if (buf.data) delete[] buf.data;
}
}
