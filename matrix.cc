/*
 * AFAIK, we will only need vec3 and maybe vec4,
 * and only square matrix multiplication. But this is easy to
 * change.
 */

/* TODO:
 vector to struct? array to struct?*/
struct {
  int x;
  int y;
  int z;
} vec3;

struct {
  int x;
  int y;
  int z;
  int w
} vec4;

struct {
  vec3 r1;
  vec3 r2;
  vec3 r3;
} m3;

struct {
  vec4 r1;
  vec4 r2;
  vec4 r3;
  vec4 r4;
} m4;
