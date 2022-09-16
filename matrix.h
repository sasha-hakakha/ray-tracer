#ifndef MATRIX
#define MATRIX

struct vec3;
struct vec4;
struct ray;
struct plane;
float dot_v3v3();
vec3 cross_v3v3();
vec3 sub_v3v3();
vec3 add_v3v3();
vec3 mul_v3fl();
vec3 points_to_plane();
vec3 ray_isect_plane();
float dist_v3v3();
bool p_inside_tri();
vec3 p_inside_tri();
void m3m();
void m4m();

#endif
