use crate::core::error::Error;

pub struct Tensor<T> {
    data:Vec<T>, 
    shape:Vec<usize>,
    strides:Vec<usize>,
}

impl<T> Tensor<T>{
    fn calculate_strides(shape:&Vec<usize>) -> Vec<usize>{
        
    }
    
    pub fn new(data:Vec<T>, shape:Vec<usize>) -> Result<Self, Error> {
        let expected_elements:usize = shape.iter().product();
       
        if data.len() != expected_elements {
            return Err(Error::NumbersOfElementsError);
        }

        let strides = Tensor::<T>::calculate_strides(&shape);
        Ok(Tensor { data, shape, strides })
    }


}