//! Keyswitch key with Cuda.
use crate::backends::cuda::private::crypto::glwe::ciphertext::CudaGlweCiphertext;
use crate::backends::cuda::private::crypto::lwe::list::CudaLweList;
use crate::backends::cuda::private::device::{CudaStream, GpuIndex, NumberOfGpus};
use crate::backends::cuda::private::vec::CudaVec;
use crate::backends::cuda::private::{compute_number_of_samples_on_gpu, number_of_active_gpus};
use crate::commons::numeric::UnsignedInteger;
use crate::prelude::{
    CiphertextCount, DecompositionBaseLog, DecompositionLevelCount,
    FunctionalPackingKeyswitchKeyCount, GlweDimension, LweDimension, PolynomialSize,
};

#[derive(Debug)]
pub(crate) struct CudaLweKeyswitchKey<T: UnsignedInteger> {
    // Pointers to GPU data: one cuda vec per GPU
    pub(crate) d_vecs: Vec<CudaVec<T>>,
    // Input LWE dimension
    pub(crate) input_lwe_dimension: LweDimension,
    // Output LWE dimension
    pub(crate) output_lwe_dimension: LweDimension,
    // Number of decomposition levels
    pub(crate) decomp_level: DecompositionLevelCount,
    // Value of the base log for the decomposition
    pub(crate) decomp_base_log: DecompositionBaseLog,
}

unsafe impl<T> Send for CudaLweKeyswitchKey<T> where T: Send + UnsignedInteger {}
unsafe impl<T> Sync for CudaLweKeyswitchKey<T> where T: Sync + UnsignedInteger {}

pub(crate) unsafe fn execute_lwe_ciphertext_vector_keyswitch_on_gpu<T: UnsignedInteger>(
    streams: &[CudaStream],
    output: &mut CudaLweList<T>,
    input: &CudaLweList<T>,
    ksk: &CudaLweKeyswitchKey<T>,
    number_of_available_gpus: NumberOfGpus,
) {
    let number_of_gpus = number_of_active_gpus(
        number_of_available_gpus,
        CiphertextCount(input.lwe_ciphertext_count.0),
    );

    for gpu_index in 0..number_of_gpus.0 {
        let samples_per_gpu = compute_number_of_samples_on_gpu(
            number_of_available_gpus,
            CiphertextCount(input.lwe_ciphertext_count.0),
            GpuIndex(gpu_index),
        );
        let stream = &streams.get(gpu_index).unwrap();

        stream.discard_keyswitch_lwe_ciphertext_vector::<T>(
            output.d_vecs.get_mut(gpu_index).unwrap(),
            input.d_vecs.get(gpu_index).unwrap(),
            input.lwe_dimension,
            output.lwe_dimension,
            ksk.d_vecs.get(gpu_index).unwrap(),
            ksk.decomp_base_log,
            ksk.decomp_level,
            samples_per_gpu,
        );
    }

    // Required to ensure correctness of async operation across GPUs
    for stream in streams.iter(){
        stream.synchronize_stream()
    }
}

#[derive(Debug)]
pub(crate) struct CudaLwePrivateFunctionalPackingKeyswitchKeyList<T: UnsignedInteger> {
    // Pointers to GPU data: one cuda vec per GPU
    pub(crate) d_vecs: Vec<CudaVec<T>>,
    // Input LWE dimension
    pub(crate) input_lwe_key_dimension: LweDimension,
    // Output LWE dimension
    pub(crate) output_glwe_key_dimension: GlweDimension,
    // Output polynomial size
    pub(crate) output_polynomial_size: PolynomialSize,
    // Number of decomposition levels
    pub(crate) decomposition_level_count: DecompositionLevelCount,
    // Value of the base log for the decomposition
    pub(crate) decomposition_base_log: DecompositionBaseLog,
    // Number of PFPKS keys
    pub(crate) fpksk_count: FunctionalPackingKeyswitchKeyCount,
}

unsafe impl<T> Send for CudaLwePrivateFunctionalPackingKeyswitchKeyList<T> where
    T: Send + UnsignedInteger
{
}
unsafe impl<T> Sync for CudaLwePrivateFunctionalPackingKeyswitchKeyList<T> where
    T: Sync + UnsignedInteger
{
}

pub(crate) unsafe fn execute_lwe_ciphertext_vector_fp_keyswitch_on_gpu<T: UnsignedInteger>(
    streams: &[CudaStream],
    output: &mut CudaGlweCiphertext<T>,
    input: &CudaLweList<T>,
    fp_ksk_list: &CudaLwePrivateFunctionalPackingKeyswitchKeyList<T>,
    number_of_available_gpus: NumberOfGpus,
) {
    let number_of_gpus = number_of_active_gpus(
        number_of_available_gpus,
        CiphertextCount(input.lwe_ciphertext_count.0),
    );

    for gpu_index in 0..number_of_gpus.0 {
        let samples_per_gpu = compute_number_of_samples_on_gpu(
            number_of_available_gpus,
            CiphertextCount(input.lwe_ciphertext_count.0),
            GpuIndex(gpu_index),
        );
        let stream = &streams.get(gpu_index).unwrap();

        stream.discard_fp_keyswitch_lwe_to_glwe::<T>(
            &mut output.d_vec,
            input.d_vecs.get(gpu_index).unwrap(),
            fp_ksk_list.d_vecs.get(gpu_index).unwrap(),
            input.lwe_dimension,
            output.glwe_dimension,
            output.polynomial_size,
            fp_ksk_list.decomposition_base_log,
            fp_ksk_list.decomposition_level_count,
            samples_per_gpu,
            fp_ksk_list.fpksk_count,
        );
    }
}
