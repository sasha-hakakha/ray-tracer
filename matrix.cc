/*
 * I'm not using structs for matricies,
 * a matrix will be an array of arrays. Points are
 * vecs lol.
 */

/* TODO:
*/

#include <iostream>
#include <cmath>
// matrix multiplication
struct vec3{
    float x;
    float y;
    float z;
};

struct vec4{
    float x;
    float y;
    float z;
    float w;
};

struct ray{
    vec3* p;
    vec3* v;
};

struct plane{
    vec3* coord;
    vec3* norm;
};

void dot_v3v3(vec3* one, vec3* two, float &result){
    result = one->x * two->x + one->y * two->y + one->z * two->z;
}

void cross_v3v3(vec3* one, vec3* two, vec3 &result){
    result.x = one->y * two->z - one->z * two->y;
    result.y = one->x * two->z - one->z * two->x;
    result.z = one->x * two->y - one->y * two->x;
}
void sub_v3v3(vec3* one, vec3* two, vec3 &result){
    result.x = one->x - two->x;
    result.y = one->y - two->y;
    result.z = one->z - two->z;
}

void add_v3v3(vec3* one, vec3* two, vec3 &result){
    result.x = one->x - two->x;
    result.y = one->y - two->y;
    result.z = one->z - two->z;
}

void mul_v3fl(vec3* v, float f, vec3 &result){
    result.x = v->x * f;
    result.y = v->y * f;
    result.z = v->z * f;
}

void points_to_plane(vec3* one, vec3* two, vec3* three, plane result){
    vec3 sub_one;
    vec3 sub_two;
    //sub one from two and one from three
    sub_v3v3(two, one, sub_one);
    sub_v3v3(three, one, sub_two);
    //cross these results
    cross_v3v3(&sub_one, &sub_two, *result.coord);
    result.norm = one;
}

vec3 ray_isect_plane(ray* r, plane* pl){
    // ignoring parallelism here
    float epsilon = std::pow(10, -6);
    float dot;
    dot_v3v3(pl->norm, r->v, dot);

    vec3 w;
    sub_v3v3(r->p, pl->coord, w);

    float dot2;
    vec3* temp_w = &w;
    dot_v3v3(pl->norm, temp_w, dot2);
    
    float fac = -1.0 * dot2 / dot;
    vec3 u;
    mul_v3fl(r->v, fac, u);
    vec3 add_result;
    add_v3v3(r->p, &u, add_result);
    return add_result;
}
float dist_v3v3(vec3 a, vec3 b){
  return std::sqrt(std::pow((a.x-b.x),2) + std::pow((a.y-b.y),2) + std::pow((a.z - b.z), 2));
e
bool p_inside_tri(vec3 p, vec3 v1, vec3 v2, vec3 v3){
    float d1 = dist_v3v3(p, v1);
    float d2 = dist_v3v3(p, v2);
    float d3 = dist_v3v3(p, v3);
    float total = d1 + d2 + d3;
    return (d1 > 0 && d2 > 0 && d3 > 0 && d1 < total && d2 < total && d3 < total);
}

void m3m(float mat1[3][3], float mat3[3][3], float(&result)[3][3]) {
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            result[i][j] = 0;
            for (int k = 0; k < 3; k++) {
                result[i][j] += mat1[i][k] * mat3[k][j];
            }
        }
    }
}
void m4m(float mat1[4][4], float mat2[4][4] , float(&result)[4][4]) {
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            result[i][j] = 0;
            for (int k = 0; k < 4; k++) {
                result[i][j] += mat1[i][k] * mat2[k][j];
            } 
        }
    }
}

int main(void){
    return 0;
}
