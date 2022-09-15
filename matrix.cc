/*
 * AFAIK, we will only need vec3 and maybe vec4,
 * and only square matrix multiplication. But this is easy to
 * change.
 */

/* TODO:
 I'm only doing 4x4 matrix multiplication if I am sure that I need to
*/
struct vec3{
    float x;
    float y;
    float z;
};

struct vec4{
    float x;
    float y;
    float z;
    float w
};

struct m3{
    vec3 r1;
    vec3 r2;
    vec3 r3;
};

struct m4{
    vec4 r1;
    vec4 r2;
    vec4 r3; 
    vec4 r4;
};

m3 mm3 (m4 one, m4 two){
    m3 result;
    // top left, one top row by two left col
    result.r1.x = one.r1.x * two.r1.x + one.r1.y * two.r2.x + one.r1.z * two.r3.x;
    // top mid, one top row by two middle col
    result.r1.y = one.r1.x * two.r1.y + one.r1.y * two.r2.y + one.r1.z + two.r3.y;
    // top right, one top row by two right col
    result.r1.z = one.r1.x * two.r1.z * one.r1.y * two.r2.z + one.r1.z + two.r3.z;
    // mid left
}
