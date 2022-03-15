pub struct Matrix {
    width: usize,
    height: usize,
    d: Vec<f64>
}

impl Matrix {

    fn new(width: usize, height: usize) -> Matrix {
        let d = vec![0.0; width * height];
        Matrix {width, height, d}
    }

    fn new_from_vec_2d(vec: Vec<Vec<f64>>) -> Matrix {
        assert!(vec.len() > 0 && vec[0].len() > 0);
        let width = vec.len();
        let height = vec[0].len(); 
        let mut matrix = Matrix::new(width, height);
        matrix.fill(vec);
        matrix
    }

    fn new_from_vec(vec: Vec<f64>, width: usize) -> Matrix {
        assert!(vec.len() % width == 0);
        let height = vec.len() / width;
        Matrix {width, height, d: vec}
    }

    fn set(&mut self, row: usize, col: usize, value: f64) -> bool {
        assert!(true);
        self.d[(row * self.width + col)] = value;
        true
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        assert!(true);
        self.d[row * self.width + col]
    }

    fn fill(&mut self, vec: Vec<Vec<f64>>) -> bool {  
        assert!(vec.len() == self.width && vec[0].len() == self.height);
        for (i, row) in vec.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                self.set(i, j, *value);
            }
        }
        true
    }

    fn to_vec(&mut self) -> Vec<f64> {
        self.d.clone()
    }

}