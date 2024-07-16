#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace rust {

struct CBuffer {
  double *data;
  size_t len;
};


extern "C" {

CBuffer fracgm_registration_solver(const double *pc1,
                                   size_t pc1_n_rows,
                                   size_t pc1_n_cols,
                                   const double *pc2,
                                   size_t pc2_n_rows,
                                   size_t pc2_n_cols,
                                   size_t max_iteration,
                                   double tol,
                                   double noise_bound,
                                   double c);

CBuffer fracgm_rotation_solver(const double *pc1,
                               size_t pc1_n_rows,
                               size_t pc1_n_cols,
                               const double *pc2,
                               size_t pc2_n_rows,
                               size_t pc2_n_cols,
                               size_t max_iteration,
                               double tol,
                               double noise_bound,
                               double c);

void free_buf(CBuffer buf);

} // extern "C"

} // namespace rust
