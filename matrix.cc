/*
 * I'm not using structs for vecs and matricies,
 * a matrix will be an array of arrays
 */

/* TODO:
*/


// matrix multiplication
void m3m(int mat1[3][3], int mat3[3][3], int(&result)[3][3]) {
    for (int i = 0; i < 3; i++) {
        for (int j = 0; j < 3; j++) {
            result[i][j] = 0;
            for (int k = 0; k < 3; k++) {
                result[i][j] += mat1[i][k] * mat3[k][j];
            }
        }
    }
}
void m4m(int mat1[4][4], int mat2[4][4] , int(&result)[4][4]) {
 
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 4; j++) {
            result[i][j] = 0;
            for (int k = 0; k < 4; k++) {
                result[i][j] += mat1[i][k] * mat2[k][j];
            } 
        }
    }
}

