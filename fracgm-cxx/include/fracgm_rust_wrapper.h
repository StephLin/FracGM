#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace rust {

struct CBufferUSize {
  size_t *data;
  size_t len;
};

struct CBufferF64 {
  double *data;
  size_t len;
};


extern "C" {

CBufferUSize fracgm_max_clique_inlier_selection(const double *pc1,
                                                size_t pc1_n_rows,
                                                size_t pc1_n_cols,
                                                const double *pc2,
                                                size_t pc2_n_rows,
                                                size_t pc2_n_cols,
                                                double noise_bound,
                                                double pmc_timeout);

CBufferF64 fracgm_registration_solver(const double *pc1,
                                      size_t pc1_n_rows,
                                      size_t pc1_n_cols,
                                      const double *pc2,
                                      size_t pc2_n_rows,
                                      size_t pc2_n_cols,
                                      size_t max_iteration,
                                      double tol,
                                      double noise_bound,
                                      double c);

CBufferF64 fracgm_rotation_solver(const double *pc1,
                                  size_t pc1_n_rows,
                                  size_t pc1_n_cols,
                                  const double *pc2,
                                  size_t pc2_n_rows,
                                  size_t pc2_n_cols,
                                  size_t max_iteration,
                                  double tol,
                                  double noise_bound,
                                  double c);

void free_f64_buf(CBufferF64 buf);

void free_usize_buf(CBufferUSize buf);

}  // extern "C"

}  // namespace rust
