use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Matrix {
    width: usize,
    height: usize,
    d: Vec<f64>
}

impl Matrix {

    pub fn new(width: usize, height: usize) -> Matrix {
        let d = vec![0.0; width * height];
        Matrix {width, height, d}
    }

    pub fn new_from_vec_2d(vec: Vec<Vec<f64>>) -> Matrix {
        assert!(vec.len() > 0 && vec[0].len() > 0);
        let width = vec.len();
        let height = vec[0].len(); 
        let mut matrix = Matrix::new(width, height);
        matrix.fill(vec);
        matrix
    }

    pub fn new_from_vec(vec: Vec<f64>, width: usize) -> Matrix {
        assert!(vec.len() % width == 0);
        let height = vec.len() / width;
        Matrix {width, height, d: vec}
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> bool {
        assert!(true);
        self.d[(row * self.width + col)] = value;
        true
    }

    pub fn set_row(&mut self, row: usize, values: Vec<f64>) -> bool {
        assert!(true);
        for (i, value) in values.iter().enumerate() {
            self.set(row, i, *value);
        }
        true
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        assert!(true);
        self.d[row * self.width + col]
    }

    pub fn fill(&mut self, vec: Vec<Vec<f64>>) -> bool {  
        assert!(vec.len() == self.width && vec[0].len() == self.height);
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                self.set(i, j, *value);
            }
        }
        true
    }

    pub fn to_vec(&mut self) -> Vec<f64> {
        self.d.clone()
    }
}

#[wasm_bindgen]
pub fn points(a: bool) -> Vec<f64> {

    // let mut pts: Matrix = Matrix::new(3, 10);

    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![20.0, 30.0, 2.0]);
    pts.push(vec![120.0, 33.0, 12.5]);
    pts.push(vec![124.0, 222.0, 7.65]);
    pts.push(vec![20.0, 133.0, 21.0]);
    pts.push(vec![60.0, 60.0, 33.0]);

    if a {
        pts.push(vec![60.0, 70.0, 33.0]);
    }

    Matrix::new_from_vec_2d(pts).to_vec()
}