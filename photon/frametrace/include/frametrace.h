/* photon-frametrace C ABI. A C or C++ D3D11 vtable hook drives the Rust core
   through these functions. View/resource ids are the ID3D11 pointers cast to
   uint64; id 0 means null / unbound.

   ViewKind: 0=Srv 1=Rtv 2=Dsv 3=DsvReadOnly 4=Uav
   Stage:    0=Vs  1=Ps  2=Cs  3=Gs 4=Hs 5=Ds */
#ifndef PHOTON_FRAMETRACE_H
#define PHOTON_FRAMETRACE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct FrameState FrameState;

FrameState* ft_new(void);
void        ft_free(FrameState*);

void ft_register_view(FrameState*, uint64_t view, uint64_t resource, int kind);
void ft_set_shader_resources(FrameState*, int stage, uint32_t start, const uint64_t* views, size_t n);
void ft_set_unordered_access_views(FrameState*, uint32_t start, const uint64_t* views, size_t n);
void ft_set_render_targets(FrameState*, const uint64_t* rtvs, size_t n, uint64_t dsv);
void ft_draw(FrameState*);
void ft_dispatch(FrameState*);

size_t   ft_hazard_count(const FrameState*);
int      ft_hazard_kind(const FrameState*, size_t i); /* 0=ReadWrite 1=WriteWrite -1=none */
const char* ft_hazard_kind_name(const FrameState*, size_t i); /* "ReadWrite"|"WriteWrite"|"none" */
uint64_t ft_hazard_resource(const FrameState*, size_t i);

#ifdef __cplusplus
}
#endif
#endif
