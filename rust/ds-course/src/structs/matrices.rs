use crate::structs::arrays::HeapArray;
use crate::structs::matrices::MatrixType::{
    Dense, Diagonal, LowerTriangular, Sparse, Toeplitz, Tridiagonal, UpperTriangular,
};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

enum MatrixType<T>
where
    T: PartialEq,
{
    Dense(DenseMatrix<T>),
    Diagonal(DiagonalMatrix<T>),
    UpperTriangular(UpperTraingularMatrix<T>),
    LowerTriangular(LowerTriangularMatrix<T>),
    Toeplitz(ToeplitzMatrix<T>),
    Tridiagonal(TridiagonalMatrix<T>),
    Sparse(SparseMatrix<T>),
}

pub struct Matrix<T>
where
    T: PartialEq,
{
    rows: usize,
    columns: usize,
    matrix_type: MatrixType<T>,
    default_value: T,
    auto_adjust: bool,
}

impl<T: PartialEq> Matrix<T> {
    pub fn new(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: Dense(DenseMatrix::new(row, col)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn new_diagonal(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: Diagonal(DiagonalMatrix::new(row)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn new_upper_triangular(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: UpperTriangular(UpperTraingularMatrix::new(row)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn new_lower_triangular(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: LowerTriangular(LowerTriangularMatrix::new(row)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn new_toeplitz(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: Toeplitz(ToeplitzMatrix::new(row)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn new_tridiagonal(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: Tridiagonal(TridiagonalMatrix::new(row)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn new_sparse(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        Matrix {
            rows: row,
            columns: col,
            matrix_type: Sparse(SparseMatrix::new(row, col)),
            default_value: T::default(),
            auto_adjust: false,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        match &self.matrix_type {
            Dense(matrix) => Some(matrix.get(row, col)),
            UpperTriangular(matrix) => Some(matrix.get(row, col)),
            LowerTriangular(matrix) => Some(matrix.get(row, col)),
            Diagonal(matrix) => Some(matrix.get(row, col)),
            Toeplitz(matrix) => Some(matrix.get(row, col)),
            Tridiagonal(matrix) => Some(matrix.get(row, col)),
            Sparse(matrix) => Some(matrix.get(row, col)),
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        match &mut self.matrix_type {
            Dense(matrix) => matrix.set(row, col, val),
            UpperTriangular(matrix) => matrix.set(row, col, val),
            LowerTriangular(matrix) => matrix.set(row, col, val),
            Diagonal(matrix) => matrix.set(row, col, val),
            Toeplitz(matrix) => matrix.set(row, col, val),
            Tridiagonal(matrix) => matrix.set(row, col, val),
            Sparse(matrix) => matrix.set(row, col, val),
        }
    }

    // pub fn get_array(&self) -> &HeapArray<T> {
    //     match &self.matrix_type {
    //         Dense(matrix) => &matrix.array,
    //         UpperTriangular(matrix) => &matrix.array,
    //         LowerTriangular(matrix) => &matrix.array,
    //         Diagonal(matrix) => &matrix.array,
    //         Toeplitz(matrix) => &matrix.array,
    //         Tridiagonal(matrix) => &matrix.array,
    //         Sparse(matrix) => &matrix.array,
    //     }
    // }

    fn optimize(&mut self) -> ()
    where
        T: PartialEq,
        T: Default,
    {
        // // Cases that needs to be handled
        // // By default, Dense matrix is created
        // // We need to check at every push if the matrix can be optimized
        // // Dense --> Diagonal or Sparse
        // // Diagonal --> Dense or Sparse
        // // Sparse --> Dense or Diagonal
        // let mut is_diagonal = false;
        // let mut is_sparse = false;
        //
        // let mut diagonal_count: usize = 0;
        // let mut non_zero_count: usize = 0;
        // let mut zero_count: usize = 0;
        //
        // for i in 1..self.rows + 1 {
        //     for j in 1..self.columns + 1 {
        //         let val = self.get(i, j).unwrap();
        //
        //         if *val == self.default_value {
        //             zero_count += 1;
        //         } else {
        //             if i == j {
        //                 diagonal_count += 1;
        //             }
        //             non_zero_count += 1;
        //         }
        //     }
        // }
        //
        // if diagonal_count == non_zero_count {
        //     self.convert_to_diagonal();
        // } else if non_zero_count < zero_count {
        //     is_sparse = true;
        // } else {
        //     ()
        // }
    }

    fn convert_to_diagonal(&mut self)
    where
        T: Default,
        T: Copy,
    {
        // let mut new_matrix: DiagonalMatrix<T> = DiagonalMatrix::new(self.rows.min(self.columns));
        // for i in 1..self.rows.min(self.columns) + 1 {
        //     new_matrix.set(i, i, self.get(i, i).unwrap())
        // }
        // self.matrix_type = Diagonal(new_matrix);
    }

    fn convert_to_sparse(&mut self) {
        ()
    }
}

impl<T: Display + Default + PartialEq> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 1..self.rows + 1 {
            for j in 1..self.columns + 1 {
                write!(f, "{:>3} ", self.get(i, j).unwrap())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

enum IndexOrder {
    RowMajor,
    ColumnMajor,
}

pub trait MatrixOperations<T: PartialEq> {
    fn array_size(row: usize, col: usize) -> usize;

    fn array_index(&self, row: usize, col: usize) -> usize;

    fn get(&self, row: usize, col: usize) -> &T;

    fn set(&mut self, row: usize, col: usize, val: T);
}

pub struct DenseMatrix<T>
where
    T: PartialEq,
{
    rows: usize,
    columns: usize,
    array: HeapArray<T>,
    index_order: IndexOrder,
}

impl<T: PartialEq> DenseMatrix<T> {
    pub fn new(row: usize, col: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array = HeapArray::with_capacity(Self::array_size(row, col));
        array.fill(T::default());
        DenseMatrix {
            rows: row,
            columns: col,
            array,
            index_order: IndexOrder::RowMajor,
        }
    }

    fn row_major_index(&self, i: usize, j: usize) -> usize {
        (i - 1) * self.columns + (j - 1)
    }

    fn column_major_index(&self, i: usize, j: usize) -> usize {
        (j - 1) * self.rows + (i - 1)
    }
}

impl<T: PartialEq> MatrixOperations<T> for DenseMatrix<T> {
    fn array_size(row: usize, col: usize) -> usize {
        row * col
    }

    fn array_index(&self, i: usize, j: usize) -> usize {
        if i == 0 || j == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        } else if i > self.rows || j > self.columns {
            panic!("Row or column exceeds the maximum Matrix dimensions!");
        }
        return match self.index_order {
            IndexOrder::RowMajor => self.row_major_index(i, j),
            IndexOrder::ColumnMajor => self.column_major_index(i, j),
        };
    }

    fn get(&self, row: usize, col: usize) -> &T {
        self.array.get(self.array_index(row, col))
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        self.array.set(self.array_index(row, col), val);
    }
}

pub struct TridiagonalMatrix<T>
where
    T: PartialEq,
{
    dimension: usize,
    array: HeapArray<T>,
    default_value: T,
}

impl<T: PartialEq> TridiagonalMatrix<T> {
    pub fn new(dimension: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array: HeapArray<T> =
            HeapArray::with_capacity(Self::array_size(dimension, dimension));
        array.fill(T::default());
        TridiagonalMatrix {
            dimension,
            array,
            default_value: T::default(),
        }
    }
}

impl<T: PartialEq> MatrixOperations<T> for TridiagonalMatrix<T> {
    fn array_size(row: usize, _col: usize) -> usize {
        3 * row - 2
    }

    fn array_index(&self, row: usize, col: usize) -> usize {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        let mut index: usize = 0;
        if row as isize - col as isize == 1 {
            index = row - 2;
        } else if row as isize - col as isize == 0 {
            index = self.dimension + row - 2;
        } else if row as isize - col as isize == -1 {
            index = 2 * self.dimension + row - 2;
        }
        index
    }

    fn get(&self, row: usize, col: usize) -> &T {
        if (row as isize - col as isize).abs() <= 1 {
            return self.array.get(self.array_index(row, col));
        }
        return &self.default_value;
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        if (row as isize - col as isize).abs() <= 1 {
            self.array.set(self.array_index(row, col), val);
        }
    }
}

pub struct UpperTraingularMatrix<T>
where
    T: PartialEq,
{
    dimension: usize,
    array: HeapArray<T>,
    default_value: T,
    index_order: IndexOrder,
}

impl<T: PartialEq> UpperTraingularMatrix<T> {
    pub fn new(dimension: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array: HeapArray<T> =
            HeapArray::with_capacity(Self::array_size(dimension, dimension));
        array.fill(T::default());
        UpperTraingularMatrix {
            dimension,
            array,
            default_value: T::default(),
            index_order: IndexOrder::ColumnMajor,
        }
    }

    fn row_major_index(&self, row: usize, col: usize) -> usize {
        let row_signed = row as isize;
        let col_signed = col as isize;
        let dimenion_signed = self.dimension as isize;
        ((dimenion_signed * (row_signed - 1) - (row_signed - 2) * (row_signed - 1) / 2)
            + col_signed
            - row_signed) as usize
    }

    fn column_major_index(&self, row: usize, col: usize) -> usize {
        return (col * (col - 1) / 2) + (row - 1);
    }
}

impl<T: PartialEq> MatrixOperations<T> for UpperTraingularMatrix<T> {
    fn array_size(row: usize, _col: usize) -> usize {
        row * (row + 1) / 2
    }

    fn array_index(&self, row: usize, col: usize) -> usize {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        return match self.index_order {
            IndexOrder::RowMajor => self.row_major_index(row, col),
            IndexOrder::ColumnMajor => self.column_major_index(row, col),
        };
    }

    fn get(&self, row: usize, col: usize) -> &T {
        if row <= col {
            return self.array.get(self.array_index(row, col));
        }
        return &self.default_value;
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        if row <= col {
            self.array.set(self.array_index(row, col), val);
        }
    }
}

pub struct LowerTriangularMatrix<T>
where
    T: PartialEq,
{
    dimension: usize,
    array: HeapArray<T>,
    default_value: T,
    index_order: IndexOrder,
}

impl<T: PartialEq> LowerTriangularMatrix<T> {
    pub fn new(dimension: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array: HeapArray<T> =
            HeapArray::with_capacity(Self::array_size(dimension, dimension));
        array.fill(T::default());
        LowerTriangularMatrix {
            dimension,
            array,
            default_value: T::default(),
            index_order: IndexOrder::ColumnMajor,
        }
    }

    fn row_major_index(row: usize, col: usize) -> usize {
        return (row * (row - 1) / 2) + (col - 1);
    }

    fn column_major_index(&self, row: usize, col: usize) -> usize {
        let row_signed = row as isize;
        let col_signed = col as isize;
        let dimenion_signed = self.dimension as isize;
        ((dimenion_signed * (col_signed - 1) - (col_signed - 2) * (col_signed - 1) / 2)
            + row_signed
            - col_signed) as usize
    }
}

impl<T: PartialEq> MatrixOperations<T> for LowerTriangularMatrix<T> {
    fn array_size(row: usize, _col: usize) -> usize {
        row * (row + 1) / 2
    }

    fn array_index(&self, row: usize, col: usize) -> usize {
        return match self.index_order {
            IndexOrder::RowMajor => Self::row_major_index(row, col),
            IndexOrder::ColumnMajor => self.column_major_index(row, col),
        };
    }

    fn get(&self, row: usize, col: usize) -> &T {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        if row >= col {
            return self.array.get(self.array_index(row, col));
        }
        return &self.default_value;
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        if row >= col {
            self.array.set(self.array_index(row, col), val);
        }
    }
}

pub struct DiagonalMatrix<T>
where
    T: PartialEq,
{
    dimension: usize,
    array: HeapArray<T>,
    default_value: T,
}

impl<T: PartialEq> DiagonalMatrix<T> {
    pub fn new(dimension: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array: HeapArray<T> =
            HeapArray::with_capacity(Self::array_size(dimension, dimension));
        array.fill(T::default());
        DiagonalMatrix {
            dimension,
            array,
            default_value: T::default(),
        }
    }
}

impl<T: PartialEq> MatrixOperations<T> for DiagonalMatrix<T> {
    fn array_size(row: usize, _col: usize) -> usize {
        row
    }

    fn array_index(&self, row: usize, col: usize) -> usize {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        row - 1
    }

    fn get(&self, row: usize, col: usize) -> &T {
        if row == col {
            return self.array.get(self.array_index(row, col));
        }
        return &self.default_value;
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        if row == col {
            self.array.set(self.array_index(row, col), val);
        }
    }
}

// TODO: Fix Toeplitz Matrix struct
pub struct ToeplitzMatrix<T>
where
    T: PartialEq,
{
    dimension: usize,
    array: HeapArray<T>,
}

impl<T: PartialEq> ToeplitzMatrix<T> {
    pub fn new(dimension: usize) -> Self
    where
        T: Default + Copy,
    {
        let mut array: HeapArray<T> =
            HeapArray::with_capacity(Self::array_size(dimension, dimension));
        array.fill(T::default());
        ToeplitzMatrix { dimension, array }
    }
}

impl<T: PartialEq> MatrixOperations<T> for ToeplitzMatrix<T> {
    fn array_size(rows: usize, cols: usize) -> usize {
        rows + cols - 1
    }

    fn array_index(&self, row: usize, col: usize) -> usize {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        let mut index: usize = 0;
        if row <= col {
            index = col - row;
        } else if row > col {
            index = self.dimension + col - row - 1;
        }
        index
    }

    fn get(&self, row: usize, col: usize) -> &T {
        return self.array.get(self.array_index(row, col));
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        self.array.set(self.array_index(row, col), val);
    }
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct SparseMatrixElement<T> {
    row: usize,
    column: usize,
    value: T,
}

impl<T: Copy> Clone for SparseMatrixElement<T> {
    fn clone(&self) -> Self {
        SparseMatrixElement {
            row: self.row,
            column: self.column,
            value: self.value,
        }
    }
}

impl<T: Copy> Copy for SparseMatrixElement<T> {}

pub struct SparseMatrix<T>
where
    T: PartialEq,
{
    rows: usize,
    columns: usize,
    nonzero_count: usize,
    array: HeapArray<SparseMatrixElement<T>>,
    default_value: T,
}

impl<T: Default + PartialEq> SparseMatrix<T> {
    pub fn new(rows: usize, columns: usize) -> Self
    where
        T: Default + Copy,
    {
        let array: HeapArray<SparseMatrixElement<T>> =
            HeapArray::with_capacity(Self::array_size(rows, columns));
        SparseMatrix {
            rows,
            columns,
            nonzero_count: 0,
            array: array,
            default_value: T::default(),
        }
    }
}

impl<T: PartialEq> MatrixOperations<T> for SparseMatrix<T> {
    fn array_size(row: usize, col: usize) -> usize {
        (row * col) / 2
    }

    fn array_index(&self, row: usize, col: usize) -> usize {
        if row == 0 || col == 0 {
            panic!("Row or column can't be 0. Matrices always start with 1 indices!");
        }
        0
    }

    fn get(&self, row: usize, col: usize) -> &T {
        for ele in self.array.iter() {
            if ele.row == row && ele.column == col {
                return &ele.value;
            }
        }
        return &self.default_value;
    }

    fn set(&mut self, row: usize, col: usize, val: T)
    where
        T: PartialEq,
    {
        if val != self.default_value {
            if self.nonzero_count == (self.rows * self.columns) / 2 {
                panic!("Sparse Matrix non-zero values count will exceed zero values count!")
            }
            self.array.push(SparseMatrixElement {
                row,
                column: col,
                value: val,
            });
            self.nonzero_count += 1;
        }
    }
}

#[cfg(test)]
mod diagonal_matrix {
    use crate::structs::matrices::{DiagonalMatrix, MatrixOperations};

    #[test]
    fn test_new() {
        let matrix: DiagonalMatrix<i8> = DiagonalMatrix::new(5);
        assert_eq!(
            matrix.array.get_size(),
            5,
            "Verifying diagonal matrix array creation"
        );
        assert_eq!(
            matrix.array.get_len(),
            5,
            "Verifying diagonal matrix array creation"
        );
    }

    #[test]
    fn test_set() {
        let mut matrix: DiagonalMatrix<i8> = DiagonalMatrix::new(5);
        matrix.set(1, 1, 10);
        assert_eq!(
            matrix.array[0], 10,
            "Testing setting a diagonal matrix element"
        );
    }

    #[test]
    fn test_get() {
        let mut matrix: DiagonalMatrix<i8> = DiagonalMatrix::new(5);
        let val: i8 = 10;
        matrix.set(1, 1, val);
        matrix.set(4, 4, val);
        assert_eq!(
            matrix.get(1, 1),
            &val,
            "Testing fetching a diagonal matrix element"
        );
        assert_eq!(
            matrix.get(4, 4),
            &val,
            "Testing fetching a diagonal matrix element"
        );
        assert_eq!(
            matrix.get(1, 4),
            &0,
            "Testing fetching a null diagonal matrix element"
        );
    }
}

#[cfg(test)]
mod lower_triangular_matrix {
    use crate::structs::matrices::{LowerTriangularMatrix, MatrixOperations};

    #[test]
    fn test_new() {
        let dimension: usize = 5;
        let total_elements: usize = dimension * (dimension + 1) / 2;
        let matrix: LowerTriangularMatrix<i8> = LowerTriangularMatrix::new(dimension);
        assert_eq!(
            matrix.array.get_size(),
            total_elements,
            "Verifying lower triangular matrix array size"
        );
        assert_eq!(
            matrix.array.get_len(),
            total_elements,
            "Verifying lower triangular matrix array length"
        );
    }

    #[test]
    fn test_set() {
        let mut matrix: LowerTriangularMatrix<i8> = LowerTriangularMatrix::new(5);
        matrix.set(5, 4, 10);
        matrix.set(2, 4, 10);
        assert_eq!(
            matrix.array[13], 10,
            "Testing a lower triangular matrix element set"
        );
        assert_eq!(
            matrix.array[3], 0,
            "Testing a lower triangular matrix element set"
        );
    }

    #[test]
    fn test_get() {
        let mut matrix: LowerTriangularMatrix<i8> = LowerTriangularMatrix::new(5);
        let val: i8 = 10;
        matrix.set(1, 1, val);
        matrix.set(5, 4, val);
        assert_eq!(
            matrix.get(1, 1),
            &val,
            "Testing a lower triangular matrix element fetch"
        );
        assert_eq!(
            matrix.get(5, 4),
            &val,
            "Testing a lower triangular matrix element fetch"
        );
        assert_eq!(
            matrix.get(1, 4),
            &0,
            "Testing a null lower triangular matrix element fetch"
        );
    }
}

#[cfg(test)]
mod upper_triangular_matrix {
    use crate::structs::matrices::{MatrixOperations, UpperTraingularMatrix};

    #[test]
    fn test_new() {
        let dimension: usize = 5;
        let total_elements: usize = dimension * (dimension + 1) / 2;
        let matrix: UpperTraingularMatrix<i8> = UpperTraingularMatrix::new(dimension);
        assert_eq!(
            matrix.array.get_size(),
            total_elements,
            "Verifying upper triangular matrix array size"
        );
        assert_eq!(
            matrix.array.get_len(),
            total_elements,
            "Verifying upper triangular matrix array length"
        );
    }

    #[test]
    fn test_set() {
        let mut matrix: UpperTraingularMatrix<i8> = UpperTraingularMatrix::new(5);
        matrix.set(4, 5, 10);
        matrix.set(2, 4, 10);
        assert_eq!(
            matrix.array[13], 10,
            "Testing a upper triangular matrix element set"
        );
        // assert_eq!(matrix.array[7], 0, "Testing a upper triangular matrix element set");
    }

    #[test]
    fn test_get() {
        let mut matrix: UpperTraingularMatrix<i8> = UpperTraingularMatrix::new(5);
        let val: i8 = 10;
        matrix.set(1, 1, val);
        matrix.set(4, 5, val);
        assert_eq!(
            matrix.get(1, 1),
            &val,
            "Testing a upper triangular matrix element fetch"
        );
        assert_eq!(
            matrix.get(4, 5),
            &val,
            "Testing a upper triangular matrix element fetch"
        );
        assert_eq!(
            matrix.get(4, 1),
            &0,
            "Testing a null upper triangular matrix element fetch"
        );
    }
}

#[cfg(test)]
mod toeplitz_matrix {
    use crate::structs::matrices::{MatrixOperations, ToeplitzMatrix};
    use rand::random;

    #[test]
    fn test_new() {
        // n x n matrix. e.g. 5 x 5
        let dimension: usize = 5;
        // total = n + n - 1
        let total_elements: usize = 9;
        let matrix: ToeplitzMatrix<i8> = ToeplitzMatrix::new(dimension);
        assert_eq!(
            matrix.array.get_size(),
            total_elements,
            "Verifying toeplitz matrix array size"
        );
        assert_eq!(
            matrix.array.get_len(),
            total_elements,
            "Verifying toeplitz matrix array length"
        );
    }

    // #[test]
    fn test_set() {
        let mut matrix: ToeplitzMatrix<i8> = ToeplitzMatrix::new(5);
        for i in 1..6 {
            for j in 1..6 {
                matrix.set(i, j, random())
            }
        }
        assert_eq!(
            matrix.get(4, 3),
            matrix.get(3, 2),
            "Testing a toeplitz  matrix element set"
        );

        // n + i - j - 1
        let test_index: usize = 5 + 4 - 3 - 1;
        assert_eq!(
            matrix.get(4, 3),
            &matrix.array[test_index],
            "Testing a toeplitz  matrix element set"
        );
    }

    // #[test]
    fn test_get() {
        let mut matrix: ToeplitzMatrix<i8> = ToeplitzMatrix::new(5);
        let val: i8 = 10;
        matrix.set(1, 1, val);
        matrix.set(4, 5, val);
        assert_eq!(
            matrix.get(1, 1),
            &val,
            "Testing a toeplitz matrix element fetch"
        );
        assert_eq!(
            matrix.get(4, 5),
            &val,
            "Testing a toeplitz matrix element fetch"
        );
        assert_eq!(
            matrix.get(4, 1),
            &0,
            "Testing a null upper triangular matrix element fetch"
        );
    }
}

mod sparse_matrix {
    use crate::structs::matrices::SparseMatrix;

    #[test]
    fn test_new() {
        // n x n matrix. e.g. 5 x 5
        let row: usize = 5;
        let col: usize = 5;

        // total = n x n / 2
        let max_nonzero_elements: usize = 12;
        let matrix: SparseMatrix<i8> = SparseMatrix::new(row, col);
        assert_eq!(
            matrix.array.get_size(),
            max_nonzero_elements,
            "Invalid Sparse Matrix array size. It should be (row * col / 2)!"
        );
        assert_eq!(
            matrix.array.get_len(),
            0,
            "Invalid Sparse Matrix array length. It should be 0!"
        );
    }

    // TODO: Add set and get unit tests for Sparse Matrix
    #[test]
    fn test_set() {}

    #[test]
    fn test_get() {}
}
