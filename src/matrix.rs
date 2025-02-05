/// Matrix type
pub type Matrix<const H: usize, const W: usize, T> = [[Option<T>; W]; H];

pub trait ToMatrix<T> {
    fn to_matrix<const H: usize, const W: usize>(self) -> Matrix<H, W, T>;
}

impl<T: std::fmt::Debug> ToMatrix<T> for Vec<Vec<Option<T>>> {
    fn to_matrix<const H: usize, const W: usize>(mut self) -> Matrix<H, W, T> {
        self.resize_with(H, || [const { None }; W].into_iter().collect());
        self.into_iter()
            .map(|mut r| {
                r.resize_with(W, || None);
                r.try_into().unwrap()
            })
            .take(H)
            .collect::<Vec<[Option<T>; W]>>()
            .try_into()
            .unwrap()
    }
}

pub trait FmtTable<T> {
    fn fmt_table(&self) -> comfy_table::Table;
}

impl<const H: usize, const W: usize, T: ToString> FmtTable<T> for Matrix<H, W, T> {
    fn fmt_table(&self) -> comfy_table::Table {
        let mx = self.iter().map(|r| {
            r.iter().map(|v| match v {
                Some(x) => x.to_string(),
                None => "".to_owned(),
            })
        });
        let mut table = comfy_table::Table::new();
        table.add_rows(mx);
        table
    }
}

// let s = "HITMAN\\u2122 Free Trial";
// let s: String = serde_json::from_str(&format!("\"{}\"", s)).unwrap();
// assert_eq!(s, "HITMAN™ Free Trial",);
