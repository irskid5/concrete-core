use crate::backends::fftw::private::math::fft::{Complex64, Fft, FourierPolynomial};
use crate::commons::crypto::glwe::GlweCiphertext;
use crate::commons::math::tensor::Tensor;
use crate::commons::math::torus::UnsignedTorus;
use concrete_commons::parameters::{GlweSize, PolynomialSize};
use concrete_fftw::array::AlignedVec;

#[derive(Debug, Clone)]
pub struct FftBuffers {
    // The FFT plan is stored here. This way, we don't pay the price of allocating it every
    // time we need to bootstrap with the same key.
    pub fft: Fft,
    // The buffers used to perform the FFT are also stored in the bootstrap key. Again, the same
    // logic apply, and we don't have to allocate them multiple times.
    pub first_buffer: FourierPolynomial<AlignedVec<Complex64>>,
    pub second_buffer: FourierPolynomial<AlignedVec<Complex64>>,
    pub output_buffer: Tensor<AlignedVec<Complex64>>,
}

#[derive(Debug, Clone)]
pub struct FourierBuffers<Scalar> {
    // Those buffers are also used to store the lut and the rounded input during the bootstrap.
    pub lut_buffer: GlweCiphertext<Vec<Scalar>>,
    pub rounded_buffer: GlweCiphertext<Vec<Scalar>>,
    pub fft_buffers: FftBuffers,
}

impl<Scalar> FourierBuffers<Scalar>
where
    Scalar: UnsignedTorus,
{
    pub fn for_params(poly_size: PolynomialSize, glwe_size: GlweSize) -> Self {
        Self::new(poly_size, glwe_size)
    }

    pub fn new(poly_size: PolynomialSize, glwe_size: GlweSize) -> Self {
        let fft = Fft::new(poly_size);
        let first_buffer = FourierPolynomial::allocate(Complex64::new(0., 0.), poly_size);
        let second_buffer = FourierPolynomial::allocate(Complex64::new(0., 0.), poly_size);
        let output_buffer = Tensor::from_container(AlignedVec::new(poly_size.0 * glwe_size.0));
        let lut_buffer = GlweCiphertext::allocate(Scalar::ZERO, poly_size, glwe_size);
        let rounded_buffer = GlweCiphertext::allocate(Scalar::ZERO, poly_size, glwe_size);

        Self {
            lut_buffer,
            rounded_buffer,
            fft_buffers: FftBuffers {
                fft,
                first_buffer,
                second_buffer,
                output_buffer,
            },
        }
    }
}
