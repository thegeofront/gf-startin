use wasm_bindgen::prelude::*;
use startin;

// NOTE: inspired by: https://github.com/hugoledoux/startin_wasm/blob/master/src/lib.rs

#[wasm_bindgen]
pub struct Triangulation {
    dt: startin::Triangulation,
}

#[wasm_bindgen]
impl Triangulation {

    pub fn triangulation(pts: Vec<f64>) -> Triangulation {
        let mut tri = Triangulation::new();
        tri.insert_multiple(pts);
        tri
    } 

    pub fn new() -> Triangulation {
        let dt = startin::Triangulation::new();
        Triangulation { dt }
    }

    pub fn insert_multiple(&mut self, pts: Vec<f64>) {
        
        const STRIDE: usize = 3;
        for i in (0..pts.len()).step_by(STRIDE) {
            self.insert_one_pt(pts[i], pts[i+1], pts[i+2]);
        }
    }

    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> bool {
        let _re = self.dt.insert_one_pt(px, py, pz);
        true
    }

    pub fn number_of_vertices(&self) -> usize {
        self.dt.number_of_vertices()
    }

    pub fn number_of_triangles(&self) -> usize {
        self.dt.number_of_triangles()
    }

    pub fn all_vertices(&self) -> Vec<f64> {
        let mut pts: Vec<f64> = Vec::new();
        let opts = self.dt.all_vertices();
        for each in opts.iter() {
            pts.push(each[0]);
            pts.push(each[1]);
            pts.push(each[2]);
        }
        pts
    }

    pub fn all_edges(&self) -> Vec<usize> {
        self.dt.all_edges()
    }

    pub fn all_triangles(&self) -> Vec<usize> {
        let mut trs: Vec<usize> = Vec::new();
        let otrs = self.dt.all_triangles();
        for each in otrs.iter() {
            trs.push(each.v[0]);
            trs.push(each.v[1]);
            trs.push(each.v[2]);
        }
        trs
    }

    pub fn closest_point(&self, px: f64, py: f64) -> usize {
        let re = self.dt.closest_point(px, py);
        if re.is_none() {
            return 0;
        } else {
            return re.unwrap();
        }
    }

    pub fn remove(&mut self, v: usize) -> bool {
        let re = self.dt.remove(v);
        if re.is_err() == true {
            return false;
        } else {
            return true;
        }
    }
}
