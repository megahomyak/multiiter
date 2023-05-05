// (a) -> a.into_iter()
// (a => &i.b) -> a.into_iter().map(|i| &(i.b).into_iter()).flatten()
// (a => &i.b => &i.c) -> a.into_iter().map(|i| &(i.b).into_iter().map(|i| &(i.c).into_iter()).flatten()).flatten()
//
// => for every expr: expr.into_iter().map(|i| next_expr).flatten(),
//    and for the base one: expr.into_iter()
macro_rules! iter {
    (&, $expr:expr) => { $expr.into_iter() };
    (&mut, $expr:expr) => { $expr.into_iter() };
    ($expr:expr) => { $expr.into_iter() };
    (&, $expr:expr => $($rest:tt)+) => { $expr.iter().map(|i| iter!(&, $($rest)+)).flatten() };
    (&mut, $expr:expr => $($rest:tt)+) => { $expr.iter_mut().map(|i| iter!(&mut, $($rest)+)).flatten() };
    ($expr:expr => $($rest:tt)+) => { $expr.into_iter().map(|i| iter!($($rest)+)).flatten() };
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Product {
        price: u32,
    }

    struct Store {
        products: Vec<Product>,
    }

    struct City {
        stores: Vec<Store>,
    }

    #[test]
    fn it_works() {
        let city = City {
            stores: vec![
                Store {
                    products: vec![Product { price: 5 }, Product { price: 18 }],
                },
                Store {
                    products: vec![Product { price: 3 }, Product { price: 17 }],
                },
            ],
        };
        let iter = iter!(city => &i.stores => &i.products);
    }
}
